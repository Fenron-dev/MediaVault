//! # api::novel
//!
//! Webnovel source adapters: fetch a novel's table of contents and download
//! individual chapters as sanitized XHTML for EPUB packaging.
//!
//! ## Responsibilities:
//! - `NovelSource` trait — the adapter contract
//! - `PoliteClient` — blocking HTTP with per-host rate limiting and retries
//! - Shared HTML utilities (content extraction, XHTML sanitizing)
//! - `detect_source` — host-based adapter dispatch
//!
//! ## Adapter convention
//! All adapters return chapters **oldest-first** (reading order).
//!
//! ## Dependencies:
//! - `scraper` – HTML parsing
//! - `reqwest::blocking` – synchronous HTTP inside URI-scheme handler threads

pub mod generic;
pub mod novelupdates;
pub mod royalroad;
pub mod wordpress;

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use scraper::{Html, Selector};

use crate::error::{Result, VaultError};

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

/// Novel metadata plus the full chapter list, as scraped from the source.
#[derive(Debug, Clone)]
pub struct NovelInfo {
    /// Novel title.
    pub title: String,
    /// Author, if exposed by the source.
    pub author: Option<String>,
    /// Cover image URL, if exposed by the source.
    pub cover_url: Option<String>,
    /// Synopsis, if exposed by the source.
    pub description: Option<String>,
    /// `Some(true)` when the source marks the novel as finished.
    pub completed_hint: Option<bool>,
    /// All chapters in reading order (oldest first).
    pub chapters: Vec<ChapterRef>,
}

/// A single chapter reference from a table of contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChapterRef {
    /// Chapter title as listed in the ToC.
    pub title: String,
    /// Absolute chapter URL.
    pub url: String,
}

/// Downloaded and sanitized chapter content.
#[derive(Debug, Clone)]
pub struct ChapterContent {
    /// Chapter title (may be refined from the chapter page itself).
    pub title: String,
    /// Sanitized XHTML body fragment.
    pub xhtml: String,
}

/// Contract for one webnovel hosting site (or aggregator).
pub trait NovelSource {
    /// Stable adapter id (`royalroad`, `wordpress`, `generic`, `novelupdates`).
    fn id(&self) -> &'static str;

    /// Fetches the novel overview page and parses metadata plus the full ToC.
    fn fetch_novel_info(&self, client: &PoliteClient, url: &str) -> Result<NovelInfo>;

    /// Fetches one chapter page and extracts its sanitized content.
    fn fetch_chapter(&self, client: &PoliteClient, chapter: &ChapterRef) -> Result<ChapterContent>;
}

/// Picks the adapter for a subscription URL based on its host.
///
/// Unknown hosts fall back to the heuristic [`generic::GenericSource`].
pub fn detect_source(url: &str) -> Box<dyn NovelSource> {
    let host = host_of(url).unwrap_or_default();
    if host.ends_with("royalroad.com") {
        Box::new(royalroad::RoyalRoadSource)
    } else if host.ends_with("divinedaolibrary.com") {
        Box::new(wordpress::WordPressSource)
    } else if host.ends_with("novelupdates.com") {
        Box::new(novelupdates::NovelUpdatesSource)
    } else {
        Box::new(generic::GenericSource)
    }
}

// ---------------------------------------------------------------------------
// Polite HTTP client
// ---------------------------------------------------------------------------

/// Identifies the app to site operators; deliberately descriptive.
const USER_AGENT: &str = "MediaVault/0.1 (personal library tool)";
/// Minimum spacing between two requests to the same host.
const MIN_REQUEST_DELAY_MS: u64 = 1_500;
/// Per-request timeout.
const REQUEST_TIMEOUT_SECS: u64 = 30;
/// Retry attempts on transient failures (429/5xx/network).
const MAX_RETRIES: u32 = 3;
/// Backoff before each retry attempt, in seconds.
const RETRY_BACKOFF_SECS: [u64; 3] = [2, 5, 12];

/// Blocking HTTP client with per-host rate limiting and bounded retries.
///
/// All webnovel network traffic goes through this client so politeness rules
/// are enforced in one place.  Requests are strictly sequential per client.
pub struct PoliteClient {
    client: reqwest::blocking::Client,
    /// Last request instant per host, for enforcing `MIN_REQUEST_DELAY_MS`.
    last_request: Mutex<HashMap<String, Instant>>,
}

impl PoliteClient {
    /// Builds the client with the shared timeout and user agent.
    ///
    /// # Errors
    /// - `VaultError::ExternalApi` if the TLS backend fails to initialize
    pub fn new() -> Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(USER_AGENT)
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .map_err(|e| VaultError::ExternalApi(format!("HTTP client init failed: {e}")))?;
        Ok(Self {
            client,
            last_request: Mutex::new(HashMap::new()),
        })
    }

    /// Fetches a URL and returns `(final_url, body_text)`.
    ///
    /// Redirects are followed (reqwest default); `final_url` is the URL after
    /// redirects, which matters for aggregator links (NovelUpdates).  Applies
    /// the per-host delay, retries transient errors with fixed backoff, and
    /// maps Cloudflare challenges to a descriptive error.
    ///
    /// # Errors
    /// - `VaultError::ExternalApi` on HTTP errors after retries are exhausted
    pub fn get_text(&self, url: &str) -> Result<(String, String)> {
        let mut attempt = 0;
        loop {
            self.respect_delay(url);
            match self.try_get(url) {
                Ok(result) => return Ok(result),
                Err(RequestFailure::Fatal(error)) => return Err(error),
                Err(RequestFailure::Transient(error)) => {
                    if attempt as usize >= RETRY_BACKOFF_SECS.len() || attempt >= MAX_RETRIES {
                        return Err(error);
                    }
                    std::thread::sleep(Duration::from_secs(RETRY_BACKOFF_SECS[attempt as usize]));
                    attempt += 1;
                }
            }
        }
    }

    fn try_get(&self, url: &str) -> std::result::Result<(String, String), RequestFailure> {
        let response = self.client.get(url).send().map_err(|e| {
            if e.is_timeout() || e.is_connect() {
                RequestFailure::Transient(VaultError::ExternalApi(format!(
                    "request to {url} failed: {e}"
                )))
            } else {
                RequestFailure::Fatal(VaultError::ExternalApi(format!(
                    "request to {url} failed: {e}"
                )))
            }
        })?;

        let status = response.status();
        let final_url = response.url().to_string();

        if status.as_u16() == 429 || status.is_server_error() {
            // Cloudflare challenges answer with 403/503 and a server header;
            // 403 is handled below because retrying a challenge is pointless.
            if is_cloudflare_challenge(&response) {
                return Err(RequestFailure::Fatal(cloudflare_error(url)));
            }
            return Err(RequestFailure::Transient(VaultError::ExternalApi(format!(
                "{url} answered with status {status}"
            ))));
        }
        if status.as_u16() == 403 && is_cloudflare_challenge(&response) {
            return Err(RequestFailure::Fatal(cloudflare_error(url)));
        }
        if !status.is_success() {
            return Err(RequestFailure::Fatal(VaultError::ExternalApi(format!(
                "{url} answered with status {status}"
            ))));
        }

        let body = response.text().map_err(|e| {
            RequestFailure::Transient(VaultError::ExternalApi(format!(
                "reading body of {url} failed: {e}"
            )))
        })?;
        Ok((final_url, body))
    }

    /// Sleeps just long enough to honor the per-host minimum request spacing.
    fn respect_delay(&self, url: &str) {
        let Some(host) = host_of(url) else {
            return;
        };
        let min_delay = Duration::from_millis(MIN_REQUEST_DELAY_MS);
        let wait = {
            let Ok(mut map) = self.last_request.lock() else {
                return; // Poisoned lock: skip the delay rather than aborting.
            };
            let now = Instant::now();
            let wait = map
                .get(&host)
                .and_then(|last| min_delay.checked_sub(now.duration_since(*last)))
                .unwrap_or(Duration::ZERO);
            map.insert(host, now + wait);
            wait
        };
        if !wait.is_zero() {
            std::thread::sleep(wait);
        }
    }
}

/// Distinguishes retryable failures from permanent ones.
enum RequestFailure {
    Transient(VaultError),
    Fatal(VaultError),
}

fn cloudflare_error(url: &str) -> VaultError {
    VaultError::ExternalApi(format!(
        "Die Seite blockiert automatische Zugriffe (Cloudflare-Schutz): {url}"
    ))
}

fn is_cloudflare_challenge(response: &reqwest::blocking::Response) -> bool {
    response
        .headers()
        .get("server")
        .and_then(|value| value.to_str().ok())
        .map(|server| server.to_ascii_lowercase().contains("cloudflare"))
        .unwrap_or(false)
}

// ---------------------------------------------------------------------------
// Shared HTML utilities
// ---------------------------------------------------------------------------

/// Extracts the host part of an URL without pulling in an URL crate.
pub fn host_of(url: &str) -> Option<String> {
    let after_scheme = url.split_once("://").map(|(_, rest)| rest)?;
    let host_port = after_scheme.split(['/', '?', '#']).next()?;
    let host = host_port.split(':').next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_ascii_lowercase())
    }
}

/// Resolves an `href` against the page URL it appeared on.
pub fn absolutize(base_url: &str, href: &str) -> String {
    let href = href.trim();
    if href.starts_with("http://") || href.starts_with("https://") {
        return href.to_string();
    }
    if let Some(rest) = href.strip_prefix("//") {
        let scheme = base_url.split("://").next().unwrap_or("https");
        return format!("{scheme}://{rest}");
    }
    if href.starts_with('/') {
        if let (Some(scheme_end), Some(host)) = (base_url.find("://"), host_of(base_url)) {
            let scheme = &base_url[..scheme_end];
            return format!("{scheme}://{host}{href}");
        }
        return href.to_string();
    }
    // Relative path: append to the base URL's directory.
    let base_dir = match base_url.rfind('/') {
        // Keep everything up to (and including) the last slash after the scheme.
        Some(pos) if pos > base_url.find("://").map(|i| i + 2).unwrap_or(0) => &base_url[..=pos],
        _ => base_url,
    };
    format!("{}/{}", base_dir.trim_end_matches('/'), href)
}

/// Returns the inner HTML of the first element matching any of `selectors`
/// (tried in priority order).
pub fn extract_content(html: &Html, selectors: &[&str]) -> Option<String> {
    for raw_selector in selectors {
        let Ok(selector) = Selector::parse(raw_selector) else {
            continue;
        };
        if let Some(element) = html.select(&selector).next() {
            return Some(element.inner_html());
        }
    }
    None
}

/// Elements whose entire subtree is dropped during sanitizing.
const DROP_ELEMENTS: [&str; 14] = [
    "script", "style", "nav", "iframe", "form", "button", "aside", "footer", "header", "noscript",
    "svg", "video", "audio", "img",
];
/// Elements kept as-is (attributes stripped).
const KEEP_ELEMENTS: [&str; 15] = [
    "p",
    "br",
    "hr",
    "em",
    "strong",
    "i",
    "b",
    "h1",
    "h2",
    "h3",
    "h4",
    "blockquote",
    "ul",
    "ol",
    "li",
];
/// Void elements that must be self-closed in XHTML.
const VOID_ELEMENTS: [&str; 2] = ["br", "hr"];

/// Converts an arbitrary HTML fragment into a conservative XHTML fragment.
///
/// Only a small whitelist of structural tags survives; everything else is
/// either unwrapped to its children (e.g. `div`, `span`, `a`) or dropped
/// entirely (scripts, navigation, media).  All text is entity-escaped, so the
/// output is always well-formed XHTML — a requirement for EPUB readers.
pub fn sanitize_to_xhtml(fragment: &str) -> String {
    let parsed = Html::parse_fragment(fragment);
    let mut out = String::with_capacity(fragment.len());
    for child in parsed.tree.root().children() {
        sanitize_node(child, &mut out);
    }
    collapse_blank_paragraphs(&out)
}

fn sanitize_node(node: ego_tree::NodeRef<'_, scraper::Node>, out: &mut String) {
    match node.value() {
        scraper::Node::Text(text) => out.push_str(&crate::core::epub::escape_xml(&text.text)),
        scraper::Node::Element(element) => {
            let name = element.name();
            if DROP_ELEMENTS.contains(&name) {
                return;
            }
            if KEEP_ELEMENTS.contains(&name) {
                if VOID_ELEMENTS.contains(&name) {
                    out.push_str(&format!("<{name}/>"));
                    return;
                }
                out.push_str(&format!("<{name}>"));
                for child in node.children() {
                    sanitize_node(child, out);
                }
                out.push_str(&format!("</{name}>"));
                return;
            }
            // Unknown/inline wrapper (div, span, a, section, …): unwrap.
            for child in node.children() {
                sanitize_node(child, out);
            }
        }
        // Comments, doctypes, fragments: recurse into children only.
        _ => {
            for child in node.children() {
                sanitize_node(child, out);
            }
        }
    }
}

/// Removes paragraphs that contain only whitespace — a frequent artifact of
/// unwrapping ad/share containers.
fn collapse_blank_paragraphs(xhtml: &str) -> String {
    let mut result = xhtml.to_string();
    loop {
        let collapsed = result.replace("<p> </p>", "").replace("<p></p>", "");
        if collapsed == result {
            return result;
        }
        result = collapsed;
    }
}

/// Heuristic: does a link's text look like a chapter entry?
///
/// Implemented by hand because the project intentionally avoids a regex
/// dependency.  Matches `chapter`/`kapitel` substrings, `ch` + digits, and
/// titles that start with a number.
pub fn looks_like_chapter_text(text: &str) -> bool {
    let lower = text.trim().to_lowercase();
    if lower.is_empty() {
        return false;
    }
    if lower.contains("chapter") || lower.contains("kapitel") || lower.contains("episode") {
        return true;
    }
    // "ch 12", "ch. 12", "ch12"
    if let Some(rest) = lower.strip_prefix("ch") {
        let rest = rest.trim_start_matches(['.', ' ']);
        if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    // Titles starting with a number ("12 - The Fall", "103. Rebirth").
    lower.chars().next().is_some_and(|c| c.is_ascii_digit())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_extraction() {
        assert_eq!(
            host_of("https://www.royalroad.com/fiction/1"),
            Some("www.royalroad.com".to_string())
        );
        assert_eq!(
            host_of("https://example.com:8080/x?y#z"),
            Some("example.com".to_string())
        );
        assert_eq!(host_of("not a url"), None);
    }

    #[test]
    fn absolutize_variants() {
        assert_eq!(
            absolutize("https://a.com/novel/", "https://b.com/x"),
            "https://b.com/x"
        );
        assert_eq!(
            absolutize("https://a.com/novel/toc", "/chapter/1"),
            "https://a.com/chapter/1"
        );
        assert_eq!(
            absolutize("https://a.com/novel/toc", "chapter-1"),
            "https://a.com/novel/chapter-1"
        );
        assert_eq!(
            absolutize("https://a.com/x", "//cdn.a.com/img"),
            "https://cdn.a.com/img"
        );
    }

    #[test]
    fn sanitizer_keeps_whitelist_and_drops_scripts() {
        let dirty = r#"<div class="c"><p>Hello <a href="/x">world</a> &amp; more</p>
            <script>alert(1)</script><img src="x.png"/><p><br>Line</p></div>"#;
        let clean = sanitize_to_xhtml(dirty);
        assert!(clean.contains("<p>Hello world &amp; more</p>"));
        assert!(clean.contains("<br/>"));
        assert!(!clean.contains("script"));
        assert!(!clean.contains("img"));
        assert!(!clean.contains("href"));
    }

    #[test]
    fn sanitizer_escapes_raw_text() {
        let clean = sanitize_to_xhtml("<p>a < b & c</p>");
        // The parser recovers from the stray '<'; output must stay well-formed.
        assert!(!clean.contains("< b"));
        assert!(clean.contains("&amp;"));
    }

    #[test]
    fn chapter_text_heuristic() {
        assert!(looks_like_chapter_text("Chapter 12: The Fall"));
        assert!(looks_like_chapter_text("Kapitel 3"));
        assert!(looks_like_chapter_text("Ch. 44"));
        assert!(looks_like_chapter_text("103. Rebirth"));
        assert!(!looks_like_chapter_text("About the Author"));
        assert!(!looks_like_chapter_text(""));
    }

    #[test]
    fn detect_source_by_host() {
        assert_eq!(
            detect_source("https://www.royalroad.com/fiction/1").id(),
            "royalroad"
        );
        assert_eq!(
            detect_source("https://www.divinedaolibrary.com/novel-x/").id(),
            "wordpress"
        );
        assert_eq!(
            detect_source("https://www.novelupdates.com/series/x/").id(),
            "novelupdates"
        );
        assert_eq!(detect_source("https://random-site.org/n/1").id(), "generic");
    }
}

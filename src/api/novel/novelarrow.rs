//! # api::novel::novelarrow
//!
//! NovelArrow adapter (`novelarrow.com`) — a Next.js SPA whose chapter list
//! and chapter text are rendered client-side. Fetched exclusively through the
//! embedded browser window (see `api::novel::WEBVIEW_ROUTED_HOSTS`).
//!
//! ## Page structure (verified 07/2026)
//! - Novel page `/novel/<slug>`: SSR carries `og:novel:*` meta tags
//!   (title/author/genre/status/latest) but only chapters 1–30 + the newest
//!   in the DOM.
//! - Chapters tab `/novel/<slug>?tab=chapters`: the browser renders the full
//!   chapter list here → parsed for all `/chapter/<slug>/chapter-<N>-…` links.
//! - Chapter page `/chapter/<slug>/chapter-<N>-<title>`: content extracted
//!   with the generic readability heuristic.
//!
//! ## Dependencies:
//! - `api::novel` – shared HTTP client (browser-routed) and HTML utilities

use scraper::{Html, Selector};

use super::{
    generic::extract_best_content, sanitize_to_xhtml, ChapterContent, ChapterRef, NovelInfo,
    NovelSource, PoliteClient,
};
use crate::error::{Result, VaultError};

/// NovelArrow source adapter (browser-window routed).
pub struct NovelArrowSource;

impl NovelSource for NovelArrowSource {
    fn id(&self) -> &'static str {
        "novelarrow"
    }

    fn fetch_novel_info(&self, client: &PoliteClient, url: &str) -> Result<NovelInfo> {
        let slug = novel_slug(url).ok_or_else(|| {
            VaultError::ExternalApi(format!("NovelArrow-URL ohne Novel-Slug: {url}"))
        })?;
        // The chapters tab renders the full list in a real browser.
        let chapters_url = format!("{}?tab=chapters", strip_query(url));
        let (_final_url, body) = client.get_text(&chapters_url)?;
        let html = Html::parse_document(&body);

        let mut chapters = parse_chapter_links(&html, &slug);
        if chapters.is_empty() {
            // Fall back to the plain novel page (chapters 1–30 + latest).
            let (_f, body2) = client.get_text(url)?;
            chapters = parse_chapter_links(&Html::parse_document(&body2), &slug);
        }
        if chapters.is_empty() {
            return Err(VaultError::ExternalApi(format!(
                "Keine Kapitel auf der NovelArrow-Seite gefunden: {url}"
            )));
        }

        let status = meta_content(&html, "og:novel:status").unwrap_or_default();
        let completed_hint = if status.to_lowercase().contains("completed") {
            Some(true)
        } else if status.to_lowercase().contains("ongoing") {
            Some(false)
        } else {
            None
        };
        let genres = meta_content(&html, "og:novel:genre")
            .map(|g| {
                g.split(',')
                    .map(|s| title_case(s.trim()))
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        Ok(NovelInfo {
            title: meta_content(&html, "og:novel:novel_name")
                .or_else(|| first_text(&html, "h1"))
                .unwrap_or_else(|| slug.replace('-', " ")),
            author: meta_content(&html, "og:novel:author"),
            cover_url: meta_content(&html, "og:image"),
            description: meta_content(&html, "og:description")
                .or_else(|| first_text(&html, ".description, .synopsis")),
            completed_hint,
            genres,
            tags: Vec::new(),
            chapters,
        })
    }

    fn fetch_chapter(&self, client: &PoliteClient, chapter: &ChapterRef) -> Result<ChapterContent> {
        // NovelArrow server-renders the chapter body into its Flight (RSC)
        // payload — a `<id>:T<hexlen>,<html>` text chunk that
        // `chapterInfo.chapter_content` references as `$<id>`. Reading it from
        // a plain fetch is timing-independent (no waiting for a client render)
        // and reliable, so it is tried first.
        if let Ok(bytes) = client.get_bytes(&chapter.url) {
            let raw = String::from_utf8_lossy(&bytes);
            if let Some(html) = extract_flight_chapter(&raw) {
                return Ok(ChapterContent {
                    title: chapter.title.clone(),
                    xhtml: sanitize_to_xhtml(&html),
                });
            }
        }

        // Fallback: the fully rendered window HTML + the generic heuristic.
        let (_final_url, body) = client.get_text(&chapter.url)?;
        let content = extract_best_content(&body).ok_or_else(|| {
            VaultError::ExternalApi(format!(
                "Kapitelinhalt nicht erkannt: {} | Struktur: {}",
                chapter.url,
                content_outline(&body)
            ))
        })?;
        Ok(ChapterContent {
            title: chapter.title.clone(),
            xhtml: content,
        })
    }
}

/// Extracts the chapter body HTML from NovelArrow's server-rendered Flight
/// (RSC) payload. Returns `None` if no paragraph-bearing chunk is found.
fn extract_flight_chapter(raw: &str) -> Option<String> {
    let flight = collect_flight(raw);
    // Precise path: `chapterInfo.chapter_content` references the body chunk via
    // `$<id>`. Since this is *the* body, accept it even when short (some
    // chapters are author notes / "not a chapter" fillers with one paragraph).
    if let Some(html) = chapter_content_id(&flight).and_then(|id| flight_chunk(&flight, &id)) {
        if has_prose(&html) {
            return Some(html);
        }
    }
    // Heuristic fallback: the chunk with the most `<p>` tags. This is a guess,
    // so keep the stricter guard to avoid grabbing a metadata blob.
    largest_paragraph_chunk(&flight).filter(|html| html.matches("<p").count() >= 2)
}

/// Whether a Flight chunk carries real chapter prose (a paragraph or a
/// meaningful run of visible text).
fn has_prose(html: &str) -> bool {
    html.contains("<p") || html.chars().filter(|c| !c.is_whitespace()).count() >= 20
}

/// Concatenates and JSON-unescapes every `self.__next_f.push([1,"…"])` string
/// literal, reconstructing the raw Flight stream.
fn collect_flight(raw: &str) -> String {
    const MARKER: &str = "self.__next_f.push([1,";
    let bytes = raw.as_bytes();
    let mut out = String::new();
    let mut search = 0;
    while let Some(rel) = raw[search..].find(MARKER) {
        let start = search + rel + MARKER.len();
        if bytes.get(start) != Some(&b'"') {
            search = start;
            continue;
        }
        // Scan to the matching closing quote, honoring backslash escapes.
        let mut i = start + 1;
        while i < bytes.len() {
            match bytes[i] {
                b'\\' => i += 2,
                b'"' => break,
                _ => i += 1,
            }
        }
        if i >= bytes.len() {
            break;
        }
        // `start` and `i` are ASCII quotes → safe slice boundaries.
        if let Ok(decoded) = serde_json::from_str::<String>(&raw[start..=i]) {
            out.push_str(&decoded);
        }
        search = i + 1;
    }
    out
}

/// Reads the `$<id>` chunk id from `chapterInfo.chapter_content`.
fn chapter_content_id(flight: &str) -> Option<String> {
    const KEY: &str = "\"chapter_content\":\"$";
    let pos = flight.find(KEY)? + KEY.len();
    let id: String = flight[pos..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();
    (!id.is_empty()).then_some(id)
}

/// Reads a `<id>:T<hexlen>,<payload>` Flight text chunk by its declared length.
fn flight_chunk(flight: &str, id: &str) -> Option<String> {
    let head = format!("{id}:T");
    let at = if flight.starts_with(&head) {
        0
    } else {
        flight.find(&format!("\n{head}"))? + 1
    };
    let after = &flight[at + head.len()..];
    let comma = after.find(',')?;
    let payload = &after[comma + 1..];
    match usize::from_str_radix(after[..comma].trim(), 16) {
        Ok(len) if len > 0 && len <= payload.len() => {
            // `len` counts UTF-8 bytes of valid content → char boundary.
            Some(String::from_utf8_lossy(&payload.as_bytes()[..len]).into_owned())
        }
        // Length unusable (encoding drift) → cut at the next chunk marker.
        _ => Some(cut_at_next_chunk(payload)),
    }
}

/// Truncates a Flight payload at the next `\n<digits>:` chunk boundary.
fn cut_at_next_chunk(payload: &str) -> String {
    let bytes = payload.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 1 && bytes.get(j) == Some(&b':') {
                break;
            }
        }
        i += 1;
    }
    payload[..i].to_string()
}

/// Fallback: the `T`-chunk containing the most `<p>` tags (the chapter body).
fn largest_paragraph_chunk(flight: &str) -> Option<String> {
    let mut best: Option<String> = None;
    let mut best_p = 1usize;
    for (idx, _) in flight.match_indices(":T") {
        let after = &flight[idx + 2..];
        let Some(comma) = after.find(',') else {
            continue;
        };
        if usize::from_str_radix(after[..comma].trim(), 16).is_err() {
            continue;
        }
        let payload = cut_at_next_chunk(&after[comma + 1..]);
        let paragraphs = payload.matches("<p").count();
        if paragraphs > best_p {
            best_p = paragraphs;
            best = Some(payload);
        }
    }
    best
}

/// Builds a compact outline of the DOM's most text-heavy elements for
/// diagnostics: the top blocks by visible-text length as `tag#id.class(len)`.
fn content_outline(body: &str) -> String {
    let html = Html::parse_document(body);
    let Ok(selector) = Selector::parse("div, section, article, main, p") else {
        return "<selector-fehler>".to_string();
    };
    let mut blocks: Vec<(usize, String)> = Vec::new();
    for el in html.select(&selector) {
        let text_len: usize = el.text().map(|t| t.trim().len()).sum();
        if text_len < 40 {
            continue;
        }
        let v = el.value();
        let mut label = v.name().to_string();
        if let Some(id) = v.attr("id") {
            label.push('#');
            label.push_str(id);
        }
        if let Some(class) = v.attr("class") {
            // Keep the label short — first two class tokens are enough.
            let short: Vec<&str> = class.split_whitespace().take(2).collect();
            if !short.is_empty() {
                label.push('.');
                label.push_str(&short.join("."));
            }
        }
        blocks.push((text_len, format!("{label}({text_len})")));
    }
    blocks.sort_by(|a, b| b.0.cmp(&a.0));
    blocks.dedup_by(|a, b| a.1 == b.1);
    blocks
        .into_iter()
        .take(6)
        .map(|(_, label)| label)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Extracts the novel slug from a `/novel/<slug>` URL.
fn novel_slug(url: &str) -> Option<String> {
    let after = url.split("/novel/").nth(1)?;
    let slug = after.split(['/', '?', '#']).next()?;
    if slug.is_empty() {
        None
    } else {
        Some(slug.to_string())
    }
}

fn strip_query(url: &str) -> &str {
    url.split(['?', '#']).next().unwrap_or(url)
}

/// Collects every `/chapter/<slug>/chapter-<N>-…` link, deduped and sorted by
/// chapter number ascending (oldest first).
fn parse_chapter_links(html: &Html, slug: &str) -> Vec<ChapterRef> {
    let Ok(selector) = Selector::parse("a[href*='/chapter/']") else {
        return Vec::new();
    };
    let needle = format!("/chapter/{slug}/chapter-");
    let mut found: Vec<(u32, ChapterRef)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for link in html.select(&selector) {
        let Some(href) = link.value().attr("href") else {
            continue;
        };
        if !href.contains(&needle) {
            continue;
        }
        let Some(number) = chapter_number(href) else {
            continue;
        };
        let url = absolutize_novelarrow(href);
        if !seen.insert(number) {
            continue;
        }
        let text = link.text().collect::<Vec<_>>().join(" ");
        let title = clean_chapter_title(&text, number);
        found.push((number, ChapterRef { title, url }));
    }
    found.sort_by_key(|(n, _)| *n);
    found.into_iter().map(|(_, chapter)| chapter).collect()
}

/// Normalizes a NovelArrow chapter-link label. The anchors repeat a short
/// "C<N>: …" and a full "Chapter <N>: …" label; keep the canonical "Chapter …"
/// half to avoid duplicated EPUB titles like "C1: X Chapter 1: X".
fn clean_chapter_title(text: &str, number: u32) -> String {
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if collapsed.is_empty() {
        return format!("Chapter {number}");
    }
    // Prefer the exact "Chapter <number>: …" label; this drops a leading short
    // label ("C1: …" or a bare "Chapter") that the anchor prepends.
    if let Some(pos) = collapsed.find(&format!("Chapter {number}")) {
        return collapsed[pos..].trim().to_string();
    }
    collapsed
}

/// Parses the chapter number from a `.../chapter-<N>-…` URL.
fn chapter_number(href: &str) -> Option<u32> {
    let idx = href.find("/chapter-")?;
    let rest = &href[idx + "/chapter-".len()..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().ok()
}

fn absolutize_novelarrow(href: &str) -> String {
    if href.starts_with("http") {
        href.to_string()
    } else if let Some(rest) = href.strip_prefix('/') {
        format!("https://novelarrow.com/{rest}")
    } else {
        format!("https://novelarrow.com/{href}")
    }
}

fn meta_content(html: &Html, name: &str) -> Option<String> {
    for attr in ["name", "property"] {
        let sel = format!("meta[{attr}='{name}']");
        // `let-else` drops the borrowing `SelectorErrorKind` immediately;
        // an `if let Ok(..)` would keep `sel` borrowed past its scope.
        let Ok(selector) = Selector::parse(&sel) else {
            continue;
        };
        if let Some(el) = html.select(&selector).next() {
            if let Some(content) = el.value().attr("content") {
                if !content.trim().is_empty() {
                    return Some(content.trim().to_string());
                }
            }
        }
    }
    None
}

fn first_text(html: &Html, raw_selector: &str) -> Option<String> {
    let selector = Selector::parse(raw_selector).ok()?;
    let el = html.select(&selector).next()?;
    let text = el.text().collect::<Vec<_>>().join(" ");
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

/// Title-cases an all-caps genre token ("SCI-FI" → "Sci-Fi").
fn title_case(value: &str) -> String {
    value
        .split(['-', ' '])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const CHAPTERS_HTML: &str = r#"
    <html><head>
      <meta name="og:novel:novel_name" content="My Gene Evolves Infinitely"/>
      <meta name="og:novel:author" content="Blue Sky Washing Rain"/>
      <meta name="og:novel:status" content="Completed"/>
      <meta name="og:novel:genre" content="SCI-FI,ACTION,FANTASY"/>
      <meta name="og:image" content="https://images.novelarrow.com/x.webp"/>
    </head><body>
      <a href="/chapter/my-gene-evolves-infinitely/chapter-2-b">Chapter 2</a>
      <a href="/chapter/my-gene-evolves-infinitely/chapter-1-a">Chapter 1</a>
      <a href="/chapter/my-gene-evolves-infinitely/chapter-10-c">Chapter 10</a>
      <a href="/chapter/my-gene-evolves-infinitely/chapter-1-a">Chapter 1 dup</a>
      <a href="/novel/other">unrelated</a>
    </body></html>"#;

    #[test]
    fn parses_and_sorts_chapters() {
        let html = Html::parse_document(CHAPTERS_HTML);
        let chapters = parse_chapter_links(&html, "my-gene-evolves-infinitely");
        assert_eq!(chapters.len(), 3);
        assert_eq!(
            chapters[0].url,
            "https://novelarrow.com/chapter/my-gene-evolves-infinitely/chapter-1-a"
        );
        assert!(chapters[2].url.contains("chapter-10-c"));
    }

    #[test]
    fn reads_metadata() {
        let html = Html::parse_document(CHAPTERS_HTML);
        assert_eq!(
            meta_content(&html, "og:novel:novel_name").as_deref(),
            Some("My Gene Evolves Infinitely")
        );
        assert_eq!(title_case("SCI-FI"), "Sci Fi");
    }

    #[test]
    fn chapter_number_parsing() {
        assert_eq!(chapter_number("/chapter/x/chapter-780-final"), Some(780));
        assert_eq!(chapter_number("/chapter/x/chapter-1-a"), Some(1));
        assert_eq!(chapter_number("/novel/x"), None);
    }

    #[test]
    fn slug_extraction() {
        assert_eq!(
            novel_slug("https://novelarrow.com/novel/my-gene?tab=chapters").as_deref(),
            Some("my-gene")
        );
    }

    #[test]
    fn dedupes_repeated_chapter_title() {
        assert_eq!(
            clean_chapter_title("C1: Awakening Chapter 1: Awakening", 1),
            "Chapter 1: Awakening"
        );
        // A bare "Chapter" prefix on filler entries must not double up.
        assert_eq!(
            clean_chapter_title("Chapter Chapter 74: Not a Chapter", 74),
            "Chapter 74: Not a Chapter"
        );
        assert_eq!(clean_chapter_title("", 5), "Chapter 5");
        assert_eq!(clean_chapter_title("Prologue", 0), "Prologue");
    }

    #[test]
    fn accepts_single_paragraph_flight_body() {
        // Author-note chapters have one short paragraph — still valid content.
        let raw = concat!(
            "<script>self.__next_f.push([1,\"1:{\\\"chapterInfo\\\":",
            "{\\\"chapter_content\\\":\\\"$7\\\"}}\\n\"])</script>",
            "<script>self.__next_f.push([1,\"7:T15,",
            "<p>Not a chapter.</p>\\n8:x\"])</script>"
        );
        let html = extract_flight_chapter(raw).expect("single paragraph is valid");
        assert_eq!(html, "<p>Not a chapter.</p>");
    }

    // Two `__next_f` pushes: chapter metadata pointing at chunk `11` via
    // `$11`, then the body chunk `11:T<hexlen>,<html>`. The body HTML is 30
    // bytes → 0x1e. Quotes inside the pushed JSON string are `\"`-escaped and
    // the chunk boundary is a `\n`, exactly like NovelArrow's real output.
    const FLIGHT_HTML: &str = concat!(
        "<html><body>",
        "<script>self.__next_f.push([1,\"1:{\\\"chapterInfo\\\":",
        "{\\\"chapter_content\\\":\\\"$11\\\"}}\\n\"])</script>",
        "<script>self.__next_f.push([1,\"11:T1e,",
        "<p>Hello world.</p><p>Bye.</p>\\n12:x\"])</script>",
        "</body></html>"
    );

    #[test]
    fn extracts_flight_chapter_body() {
        let html = extract_flight_chapter(FLIGHT_HTML).expect("body chunk should parse");
        assert_eq!(html, "<p>Hello world.</p><p>Bye.</p>");
    }

    #[test]
    fn rejects_flight_without_prose() {
        let raw = "<script>self.__next_f.push([1,\"3:{\\\"x\\\":1}\"])</script>";
        assert!(extract_flight_chapter(raw).is_none());
    }
}

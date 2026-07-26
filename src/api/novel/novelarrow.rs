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
    generic::extract_best_content, ChapterContent, ChapterRef, NovelInfo, NovelSource, PoliteClient,
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
        let (_final_url, body) = client.get_text(&chapter.url)?;
        let content = extract_best_content(&body).ok_or_else(|| {
            VaultError::ExternalApi(format!("Kapitelinhalt nicht erkannt: {}", chapter.url))
        })?;
        Ok(ChapterContent {
            title: chapter.title.clone(),
            xhtml: content,
        })
    }
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
        let title = {
            let t = text.split_whitespace().collect::<Vec<_>>().join(" ");
            if t.is_empty() {
                format!("Chapter {number}")
            } else {
                t
            }
        };
        found.push((number, ChapterRef { title, url }));
    }
    found.sort_by_key(|(n, _)| *n);
    found.into_iter().map(|(_, chapter)| chapter).collect()
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
}

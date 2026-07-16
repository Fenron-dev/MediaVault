//! # api::novel::wordpress
//!
//! Adapter for WordPress-based translation sites (e.g. `divinedaolibrary.com`).
//!
//! ## Page structure
//! - Novel page: chapter links live inside the post body (`div.entry-content`),
//!   pointing at same-host chapter posts.
//! - Chapter page: content is the post body itself; WordPress widgets
//!   (sharing, related posts, navigation) are stripped before sanitizing.
//!
//! ## Dependencies:
//! - `api::novel` – shared HTTP client and HTML utilities

use scraper::{Html, Selector};

use super::{
    absolutize, extract_content, host_of, looks_like_chapter_text, sanitize_to_xhtml,
    ChapterContent, ChapterRef, NovelInfo, NovelSource, PoliteClient,
};
use crate::error::{Result, VaultError};

/// Content selectors for chapter pages, in priority order.
const CHAPTER_CONTENT_SELECTORS: [&str; 3] =
    ["div.entry-content", "article .post-content", "article"];

/// WordPress widget/cruft selectors removed from chapter content.
const CRUFT_SELECTORS: [&str; 4] = [
    ".sharedaddy",
    ".jp-relatedposts",
    "nav.post-navigation",
    ".wp-block-buttons",
];

/// WordPress translation-site adapter.
pub struct WordPressSource;

impl NovelSource for WordPressSource {
    fn id(&self) -> &'static str {
        "wordpress"
    }

    fn fetch_novel_info(&self, client: &PoliteClient, url: &str) -> Result<NovelInfo> {
        let (final_url, body) = client.get_text(url)?;
        parse_novel_info(&final_url, &body)
    }

    fn fetch_chapter(&self, client: &PoliteClient, chapter: &ChapterRef) -> Result<ChapterContent> {
        let (_final_url, body) = client.get_text(&chapter.url)?;
        let content = extract_chapter_content(&body).ok_or_else(|| {
            VaultError::ExternalApi(format!(
                "WordPress-Kapitelinhalt nicht gefunden: {}",
                chapter.url
            ))
        })?;
        Ok(ChapterContent {
            title: chapter.title.clone(),
            xhtml: content,
        })
    }
}

/// Extracts and sanitizes the chapter body from a WordPress post page.
fn extract_chapter_content(body: &str) -> Option<String> {
    let html = Html::parse_document(body);
    let fragment = extract_content(&html, &CHAPTER_CONTENT_SELECTORS)?;
    Some(sanitize_to_xhtml(&strip_cruft(&fragment)))
}

/// Removes known WordPress widget markup from a fragment before sanitizing.
///
/// Re-parses the fragment and re-serializes everything that does not match a
/// cruft selector.  This runs before the whitelist sanitizer, which would
/// otherwise unwrap widget containers and keep their text.
fn strip_cruft(fragment: &str) -> String {
    let parsed = Html::parse_fragment(fragment);
    let cruft: Vec<Selector> = CRUFT_SELECTORS
        .iter()
        .filter_map(|raw| Selector::parse(raw).ok())
        .collect();

    let cruft_ids: std::collections::HashSet<_> = cruft
        .iter()
        .flat_map(|selector| parsed.select(selector).map(|element| element.id()))
        .collect();

    let mut out = String::with_capacity(fragment.len());
    for child in parsed.tree.root().children() {
        serialize_without(child, &cruft_ids, &mut out);
    }
    out
}

fn serialize_without(
    node: scraper::ego_tree::NodeRef<'_, scraper::Node>,
    skip: &std::collections::HashSet<scraper::ego_tree::NodeId>,
    out: &mut String,
) {
    if skip.contains(&node.id()) {
        return;
    }
    match node.value() {
        scraper::Node::Text(text) => out.push_str(&crate::core::epub::escape_xml(text)),
        scraper::Node::Element(element) => {
            let name = element.name();
            out.push_str(&format!("<{name}>"));
            for child in node.children() {
                serialize_without(child, skip, out);
            }
            out.push_str(&format!("</{name}>"));
        }
        _ => {
            for child in node.children() {
                serialize_without(child, skip, out);
            }
        }
    }
}

/// Parses a WordPress novel overview page into metadata plus ToC.
fn parse_novel_info(page_url: &str, body: &str) -> Result<NovelInfo> {
    let html = Html::parse_document(body);
    let page_host = host_of(page_url).unwrap_or_default();

    let title = first_text(&html, "h1.entry-title")
        .or_else(|| first_text(&html, "h1"))
        .ok_or_else(|| {
            VaultError::ExternalApi(format!("Novel-Titel nicht gefunden: {page_url}"))
        })?;

    let link_selector = Selector::parse("div.entry-content a[href]")
        .map_err(|e| VaultError::ExternalApi(format!("selector parse error: {e}")))?;

    let mut chapters = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for link in html.select(&link_selector) {
        let Some(href) = link.value().attr("href") else {
            continue;
        };
        let text = link.text().collect::<Vec<_>>().join(" ");
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
        if !looks_like_chapter_text(&text) {
            continue;
        }
        let url = absolutize(page_url, href);
        // Only same-host links are chapters; external links are ads or socials.
        if host_of(&url).unwrap_or_default() != page_host {
            continue;
        }
        if seen.insert(url.clone()) {
            chapters.push(ChapterRef { title: text, url });
        }
    }

    if chapters.is_empty() {
        return Err(VaultError::ExternalApi(format!(
            "Keine Kapitel-Links auf der Seite gefunden: {page_url}"
        )));
    }

    Ok(NovelInfo {
        title,
        author: None,
        cover_url: None,
        description: None,
        completed_hint: None,
        chapters,
    })
}

fn first_text(html: &Html, raw_selector: &str) -> Option<String> {
    let selector = Selector::parse(raw_selector).ok()?;
    let element = html.select(&selector).next()?;
    let text = element.text().collect::<Vec<_>>().join(" ");
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const NOVEL_PAGE: &str = r#"
    <html><body>
      <h1 class="entry-title">Martial Test God</h1>
      <div class="entry-content">
        <p>Synopsis text here.</p>
        <ul>
          <li><a href="https://www.example-wp.com/mtg-chapter-1/">Chapter 1 – Awakening</a></li>
          <li><a href="https://www.example-wp.com/mtg-chapter-2/">Chapter 2 – Rise</a></li>
        </ul>
        <p><a href="https://twitter.com/share">Share on Twitter</a></p>
        <p><a href="https://www.example-wp.com/about/">About the Author</a></p>
      </div>
    </body></html>"#;

    #[test]
    fn parses_chapter_links_same_host_only() {
        let info = parse_novel_info("https://www.example-wp.com/mtg/", NOVEL_PAGE)
            .expect("page should parse");
        assert_eq!(info.title, "Martial Test God");
        assert_eq!(info.chapters.len(), 2);
        assert_eq!(info.chapters[0].title, "Chapter 1 – Awakening");
        assert_eq!(info.chapters[1].url, "https://www.example-wp.com/mtg-chapter-2/");
    }

    #[test]
    fn strips_wordpress_cruft() {
        let content = r#"<div class="entry-content">
            <p>Story text.</p>
            <div class="sharedaddy"><p>Share this!</p></div>
            <p>More story.</p></div>"#;
        let html = format!("<html><body><article>{content}</article></body></html>");
        let extracted = extract_chapter_content(&html).expect("content should extract");
        assert!(extracted.contains("Story text."));
        assert!(extracted.contains("More story."));
        assert!(!extracted.contains("Share this!"));
    }
}

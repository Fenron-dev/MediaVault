//! # core::cbz
//!
//! CBZ (comic book zip) reader and writer.
//!
//! The subscription engine writes archives here; the manga viewer reads page
//! images back out of them one at a time, so a 50-page chapter never has to be
//! unpacked to disk or pushed into the WebView as a whole.
//!
//! ## Why hand-rolled?
//! A CBZ is nothing but a zip archive of page images that readers display in
//! file-name order, plus an optional `ComicInfo.xml` metadata entry.  Writing
//! it directly keeps the dependency surface at the `zip` crate that
//! [`crate::core::epub`] already uses, and gives full control over page
//! ordering — the one thing a comic reader cannot recover on its own.
//!
//! ## Page ordering
//! Pages are stored as `0001.jpg`, `0002.png`, … in the order supplied by the
//! caller.  Zero-padding to four digits is what makes the plain lexicographic
//! sort every reader applies match reading order; three digits would break at
//! chapter 1000 for long-running series.
//!
//! ## Metadata
//! `ComicInfo.xml` follows the ComicRack schema, which Komga, Kavita, Mihon
//! and YACReader all read.  It is written first so readers that stream the
//! archive find it without scanning to the end.
//!
//! ## Responsibilities:
//! - Assemble downloaded page images into a valid CBZ
//! - Escape all metadata for XML safety
//!
//! ## Dependencies:
//! - `zip` – container writing
//! - `core::epub` – shared XML escaping

use std::fs::File;
use std::io::Write;
use std::path::Path;

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::core::epub::escape_xml;
use crate::error::{Result, VaultError};

/// Width of the zero-padded page number in archive entry names.
const PAGE_NUMBER_WIDTH: usize = 4;

/// One page image to embed into the archive.
#[derive(Debug, Clone)]
pub struct CbzPage {
    /// Image MIME type (`image/jpeg`, `image/png`, `image/webp`, `image/gif`).
    pub media_type: String,
    /// Raw image bytes as downloaded.
    pub bytes: Vec<u8>,
}

impl CbzPage {
    /// File extension matching the media type.
    fn extension(&self) -> &'static str {
        match self.media_type.as_str() {
            "image/png" => "png",
            "image/webp" => "webp",
            "image/gif" => "gif",
            // JPEG dominates scanlation output; unknown types are still
            // written, readers sniff the actual bytes anyway.
            _ => "jpg",
        }
    }
}

/// Chapter-level metadata written into `ComicInfo.xml`.
#[derive(Debug, Clone, Default)]
pub struct CbzMeta {
    /// Series name, e.g. `Barakamon`.
    pub series: String,
    /// Chapter title, when the source names chapters beyond their number.
    pub title: Option<String>,
    /// Chapter number as shown by the source (kept as text: sources use
    /// `10.5` for extras, which is not an integer).
    pub number: Option<String>,
    /// Volume label, when the source groups chapters into volumes.
    pub volume: Option<String>,
    /// Series synopsis.
    pub summary: Option<String>,
    /// Writer/author.
    pub writer: Option<String>,
    /// Illustrator, where the source separates it from the writer.
    pub penciller: Option<String>,
    /// Genre names.
    pub genres: Vec<String>,
    /// Source URL of the chapter, for provenance.
    pub web: Option<String>,
    /// ISO language code of this release, e.g. `en`.
    pub language: Option<String>,
    /// Right-to-left reading direction (true for Japanese manga).
    pub right_to_left: bool,
}

/// Writes a CBZ file to `target`, replacing any existing file.
///
/// # Parameters
/// - `target` – Destination path for the `.cbz` file
/// - `meta` – Chapter metadata for `ComicInfo.xml`
/// - `pages` – Page images in reading order
///
/// # Returns
/// - `Ok(())` – Archive written and flushed
///
/// # Errors
/// - `VaultError::InvalidProperty` if `pages` is empty
/// - `VaultError::Io` on filesystem or zip write failures
pub fn write_cbz(target: &Path, meta: &CbzMeta, pages: &[CbzPage]) -> Result<()> {
    if pages.is_empty() {
        return Err(VaultError::InvalidProperty(
            "cannot write a CBZ without pages".to_string(),
        ));
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(VaultError::from)?;
    }

    let file = File::create(target).map_err(VaultError::from)?;
    let mut zip = ZipWriter::new(file);

    let deflated = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    // Page images arrive already compressed (JPEG/PNG/WebP); deflating them
    // again costs CPU and grows the archive, so they are stored verbatim.
    let stored = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

    zip.start_file("ComicInfo.xml", deflated)
        .map_err(zip_error)?;
    zip.write_all(render_comic_info(meta, pages.len()).as_bytes())
        .map_err(VaultError::from)?;

    for (index, page) in pages.iter().enumerate() {
        zip.start_file(page_file_name(index, page), stored)
            .map_err(zip_error)?;
        zip.write_all(&page.bytes).map_err(VaultError::from)?;
    }

    zip.finish().map_err(zip_error)?;
    Ok(())
}

/// Archive entry name for the page at `index` (0-based).
fn page_file_name(index: usize, page: &CbzPage) -> String {
    format!(
        "{:0width$}.{}",
        index + 1,
        page.extension(),
        width = PAGE_NUMBER_WIDTH
    )
}

/// Renders the `ComicInfo.xml` document.
fn render_comic_info(meta: &CbzMeta, page_count: usize) -> String {
    let mut fields = String::new();

    let mut push = |tag: &str, value: &str| {
        if !value.is_empty() {
            fields.push_str(&format!("  <{tag}>{}</{tag}>\n", escape_xml(value)));
        }
    };

    push("Series", &meta.series);
    if let Some(title) = &meta.title {
        push("Title", title);
    }
    if let Some(number) = &meta.number {
        push("Number", number);
    }
    if let Some(volume) = &meta.volume {
        push("Volume", volume);
    }
    if let Some(summary) = &meta.summary {
        push("Summary", summary);
    }
    if let Some(writer) = &meta.writer {
        push("Writer", writer);
    }
    if let Some(penciller) = &meta.penciller {
        push("Penciller", penciller);
    }
    if !meta.genres.is_empty() {
        push("Genre", &meta.genres.join(", "));
    }
    if let Some(web) = &meta.web {
        push("Web", web);
    }
    if let Some(language) = &meta.language {
        push("LanguageISO", language);
    }
    push("PageCount", &page_count.to_string());
    // ComicRack encodes reading direction in the `Manga` field; readers use it
    // to decide whether page 2 goes left or right of page 1 in spread view.
    push(
        "Manga",
        if meta.right_to_left {
            "YesAndRightToLeft"
        } else {
            "Yes"
        },
    );

    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n\
         <ComicInfo xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
         xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">\n\
         {fields}</ComicInfo>\n"
    )
}

/// Converts a zip error into the shared vault error type.
fn zip_error(error: zip::result::ZipError) -> VaultError {
    VaultError::Io(format!("cbz write failed: {error}"))
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

/// File extensions treated as page images when reading an archive.
const PAGE_EXTENSIONS: [&str; 6] = ["jpg", "jpeg", "png", "webp", "gif", "avif"];

/// What a viewer needs to know about an archive before showing page one.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CbzInfo {
    /// Page entry names in reading order.
    pub pages: Vec<String>,
    /// Series name from `ComicInfo.xml`.
    pub series: Option<String>,
    /// Chapter title from `ComicInfo.xml`.
    pub title: Option<String>,
    /// Chapter number from `ComicInfo.xml`.
    pub number: Option<String>,
    /// Right-to-left reading order — drives page order and spread pairing.
    pub right_to_left: bool,
}

/// Reads an archive's page list and `ComicInfo.xml` metadata.
///
/// Page order is the archive's entry names sorted **naturally**, so `2.jpg`
/// precedes `10.jpg`.  Archives written by this app zero-pad and would sort
/// correctly either way, but files from other tools routinely do not.
///
/// # Parameters
/// - `path` – Absolute path to the `.cbz` file
///
/// # Returns
/// - `Ok(CbzInfo)` – Page names in reading order plus available metadata
///
/// # Errors
/// - `VaultError::Io` if the file cannot be opened or is not a zip
/// - `VaultError::InvalidProperty` if the archive holds no page images
pub fn read_cbz_info(path: &Path) -> Result<CbzInfo> {
    let file = File::open(path).map_err(VaultError::from)?;
    let mut archive = zip::ZipArchive::new(file).map_err(read_error)?;

    let mut pages: Vec<String> = Vec::new();
    let mut comic_info: Option<String> = None;
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(read_error)?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().to_string();
        if is_comic_info(&name) {
            comic_info = Some(name);
            continue;
        }
        if is_page_entry(&name) {
            pages.push(name);
        }
    }

    if pages.is_empty() {
        return Err(VaultError::InvalidProperty(format!(
            "Archiv enthält keine Seitenbilder: {}",
            path.display()
        )));
    }
    pages.sort_by(|left, right| natural_compare(left, right));

    let mut info = CbzInfo {
        pages,
        ..CbzInfo::default()
    };
    if let Some(entry_name) = comic_info {
        let mut xml = String::new();
        if let Ok(mut entry) = archive.by_name(&entry_name) {
            use std::io::Read;
            if entry.read_to_string(&mut xml).is_ok() {
                info.series = xml_tag_value(&xml, "Series");
                info.title = xml_tag_value(&xml, "Title");
                info.number = xml_tag_value(&xml, "Number");
                // ComicRack encodes direction in `Manga`; only the explicit
                // right-to-left value flips the reading order.
                info.right_to_left = xml_tag_value(&xml, "Manga")
                    .map(|value| value.eq_ignore_ascii_case("YesAndRightToLeft"))
                    .unwrap_or(false);
            }
        }
    }
    Ok(info)
}

/// Reads a single page image out of an archive.
///
/// # Parameters
/// - `path` – Absolute path to the `.cbz` file
/// - `index` – 0-based page position in [`read_cbz_info`] order
///
/// # Returns
/// - `Ok((media_type, bytes))` – Image MIME type and raw bytes
///
/// # Errors
/// - `VaultError::InvalidProperty` if `index` is past the last page
/// - `VaultError::Io` if the archive or entry cannot be read
pub fn read_cbz_page(path: &Path, index: usize) -> Result<(String, Vec<u8>)> {
    use std::io::Read;

    let info = read_cbz_info(path)?;
    let Some(name) = info.pages.get(index) else {
        return Err(VaultError::InvalidProperty(format!(
            "Seite {index} existiert nicht ({} Seiten)",
            info.pages.len()
        )));
    };

    let file = File::open(path).map_err(VaultError::from)?;
    let mut archive = zip::ZipArchive::new(file).map_err(read_error)?;
    let mut entry = archive.by_name(name).map_err(read_error)?;
    let mut bytes = Vec::with_capacity(entry.size() as usize);
    entry.read_to_end(&mut bytes).map_err(VaultError::from)?;

    // Sniff the real bytes rather than trusting the extension: a mislabelled
    // entry would otherwise reach the viewer with the wrong content type.
    let media_type = crate::api::novel::detect_image_media_type(&bytes)
        .map(str::to_string)
        .unwrap_or_else(|| media_type_for_name(name).to_string());
    Ok((media_type, bytes))
}

/// Whether an entry name is the metadata document.
fn is_comic_info(name: &str) -> bool {
    name.rsplit('/')
        .next()
        .map(|file| file.eq_ignore_ascii_case("ComicInfo.xml"))
        .unwrap_or(false)
}

/// Whether an entry name looks like a page image.
///
/// Skips macOS resource-fork entries, which carry image extensions but hold
/// no usable image data.
fn is_page_entry(name: &str) -> bool {
    if name.starts_with("__MACOSX/") {
        return false;
    }
    let file = name.rsplit('/').next().unwrap_or(name);
    if file.starts_with('.') {
        return false;
    }
    extension_of(file)
        .map(|ext| PAGE_EXTENSIONS.contains(&ext.as_str()))
        .unwrap_or(false)
}

fn extension_of(name: &str) -> Option<String> {
    name.rsplit_once('.')
        .map(|(_, ext)| ext.trim().to_lowercase())
}

fn media_type_for_name(name: &str) -> &'static str {
    match extension_of(name).as_deref() {
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("avif") => "image/avif",
        _ => "image/jpeg",
    }
}

/// Reads the text content of the first `<tag>` in an XML document.
///
/// `ComicInfo.xml` is a flat element list, so this avoids an XML dependency.
fn xml_tag_value(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    let value = unescape_xml(xml[start..end].trim());
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Reverses [`escape_xml`] for the entities it produces.
fn unescape_xml(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        // Ampersand last: doing it first would re-expand the entities above.
        .replace("&amp;", "&")
}

/// Compares names so embedded numbers order numerically (`2` before `10`).
fn natural_compare(left: &str, right: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let mut left_chars = left.chars().peekable();
    let mut right_chars = right.chars().peekable();

    loop {
        match (left_chars.peek().copied(), right_chars.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(left_char), Some(right_char)) => {
                if left_char.is_ascii_digit() && right_char.is_ascii_digit() {
                    let left_number = take_number(&mut left_chars);
                    let right_number = take_number(&mut right_chars);
                    match left_number.cmp(&right_number) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                let left_key = left_char.to_ascii_lowercase();
                let right_key = right_char.to_ascii_lowercase();
                match left_key.cmp(&right_key) {
                    Ordering::Equal => {
                        left_chars.next();
                        right_chars.next();
                    }
                    other => return other,
                }
            }
        }
    }
}

/// Consumes a run of digits and returns its numeric value.
///
/// Saturates rather than overflowing: an absurdly long digit run in a crafted
/// entry name must not panic the viewer.
fn take_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> u128 {
    let mut value: u128 = 0;
    while let Some(digit) = chars.peek().and_then(|ch| ch.to_digit(10)) {
        value = value.saturating_mul(10).saturating_add(u128::from(digit));
        chars.next();
    }
    value
}

/// Converts a zip error from the reading path into the shared error type.
fn read_error(error: zip::result::ZipError) -> VaultError {
    VaultError::Io(format!("cbz read failed: {error}"))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn page(media_type: &str) -> CbzPage {
        CbzPage {
            media_type: media_type.to_string(),
            bytes: vec![0xFF, 0xD8, 0xFF, 0x00],
        }
    }

    fn temp_target(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cbz-test-{label}-{}.cbz", std::process::id()))
    }

    #[test]
    fn pages_are_named_in_sortable_reading_order() {
        assert_eq!(page_file_name(0, &page("image/jpeg")), "0001.jpg");
        assert_eq!(page_file_name(9, &page("image/png")), "0010.png");
        assert_eq!(page_file_name(1233, &page("image/webp")), "1234.webp");

        // The padding must keep lexicographic order equal to reading order.
        let mut names: Vec<String> = (0..12)
            .map(|index| page_file_name(index, &page("image/jpeg")))
            .collect();
        let expected = names.clone();
        names.sort();
        assert_eq!(names, expected);
    }

    #[test]
    fn empty_page_list_is_rejected() {
        let target = temp_target("empty");
        let result = write_cbz(&target, &CbzMeta::default(), &[]);
        assert!(matches!(result, Err(VaultError::InvalidProperty(_))));
        assert!(!target.exists());
    }

    #[test]
    fn write_then_read_round_trips_pages_and_metadata() {
        let target = temp_target("roundtrip");
        let meta = CbzMeta {
            series: "Test & Serie".to_string(),
            title: Some("Kapitel <1>".to_string()),
            number: Some("10.5".to_string()),
            right_to_left: true,
            ..CbzMeta::default()
        };
        write_cbz(&target, &meta, &[page("image/jpeg"), page("image/png")])
            .expect("write should succeed");

        let info = read_cbz_info(&target).expect("read should succeed");
        assert_eq!(
            info.pages,
            vec!["0001.jpg".to_string(), "0002.png".to_string()]
        );
        // Escaped metadata must come back as it went in.
        assert_eq!(info.series.as_deref(), Some("Test & Serie"));
        assert_eq!(info.title.as_deref(), Some("Kapitel <1>"));
        assert_eq!(info.number.as_deref(), Some("10.5"));
        assert!(info.right_to_left);

        let (media_type, bytes) = read_cbz_page(&target, 0).expect("page should read");
        assert_eq!(media_type, "image/jpeg");
        assert_eq!(bytes, vec![0xFF, 0xD8, 0xFF, 0x00]);

        // Past the last page is an error, not a panic.
        assert!(read_cbz_page(&target, 2).is_err());

        std::fs::remove_file(&target).ok();
    }

    #[test]
    fn left_to_right_archives_report_no_flip() {
        let target = temp_target("ltr");
        write_cbz(
            &target,
            &CbzMeta {
                series: "Webtoon".to_string(),
                right_to_left: false,
                ..CbzMeta::default()
            },
            &[page("image/jpeg")],
        )
        .expect("write should succeed");

        assert!(!read_cbz_info(&target).expect("read").right_to_left);
        std::fs::remove_file(&target).ok();
    }

    #[test]
    fn pages_sort_naturally_not_lexicographically() {
        // Archives from other tools rarely zero-pad; "10" must follow "2".
        let mut names = vec![
            "page10.jpg".to_string(),
            "page2.jpg".to_string(),
            "page1.jpg".to_string(),
            "page20.jpg".to_string(),
        ];
        names.sort_by(|left, right| natural_compare(left, right));
        assert_eq!(
            names,
            ["page1.jpg", "page2.jpg", "page10.jpg", "page20.jpg"]
        );

        // A digit run far beyond u64 must not panic or wrap.
        let huge = format!("p{}.jpg", "9".repeat(60));
        assert_eq!(natural_compare(&huge, &huge), std::cmp::Ordering::Equal);
    }

    #[test]
    fn skips_metadata_and_platform_junk_entries() {
        assert!(is_page_entry("0001.jpg"));
        assert!(is_page_entry("sub/folder/0002.png"));
        assert!(!is_page_entry("ComicInfo.xml"));
        // macOS resource forks carry image extensions but hold no image.
        assert!(!is_page_entry("__MACOSX/._0001.jpg"));
        assert!(!is_page_entry(".hidden.jpg"));
        assert!(!is_page_entry("readme.txt"));

        assert!(is_comic_info("ComicInfo.xml"));
        assert!(is_comic_info("nested/comicinfo.xml"));
        assert!(!is_comic_info("0001.jpg"));
    }

    #[test]
    fn archive_without_pages_is_rejected() {
        let target = temp_target("nopages");
        // A zip holding only metadata is not a readable comic.
        let file = File::create(&target).expect("create");
        let mut zip = ZipWriter::new(file);
        zip.start_file("ComicInfo.xml", SimpleFileOptions::default())
            .expect("entry");
        zip.write_all(b"<ComicInfo/>").expect("write");
        zip.finish().expect("finish");

        assert!(matches!(
            read_cbz_info(&target),
            Err(VaultError::InvalidProperty(_))
        ));
        std::fs::remove_file(&target).ok();
    }

    #[test]
    fn writes_a_readable_archive_with_metadata() {
        let target = temp_target("write");
        let meta = CbzMeta {
            series: "Test & Serie".to_string(),
            title: Some("Kapitel <1>".to_string()),
            number: Some("10.5".to_string()),
            volume: Some("v02".to_string()),
            writer: Some("Autor".to_string()),
            genres: vec!["Action".to_string(), "Drama".to_string()],
            right_to_left: true,
            ..CbzMeta::default()
        };
        let pages = vec![page("image/jpeg"), page("image/png")];

        write_cbz(&target, &meta, &pages).expect("write should succeed");

        let file = File::open(&target).expect("archive should open");
        let mut archive = zip::ZipArchive::new(file).expect("archive should parse");
        let names: Vec<String> = archive.file_names().map(str::to_string).collect();
        assert!(names.contains(&"ComicInfo.xml".to_string()));
        assert!(names.contains(&"0001.jpg".to_string()));
        assert!(names.contains(&"0002.png".to_string()));

        let mut xml = String::new();
        archive
            .by_name("ComicInfo.xml")
            .expect("metadata should exist")
            .read_to_string(&mut xml)
            .expect("metadata should read");
        // Metadata reaching the XML comes from scraped pages, so escaping is
        // what keeps a title with markup from breaking the document.
        assert!(xml.contains("<Series>Test &amp; Serie</Series>"));
        assert!(xml.contains("<Title>Kapitel &lt;1&gt;</Title>"));
        assert!(xml.contains("<Number>10.5</Number>"));
        assert!(xml.contains("<PageCount>2</PageCount>"));
        assert!(xml.contains("<Manga>YesAndRightToLeft</Manga>"));
        assert!(xml.contains("<Genre>Action, Drama</Genre>"));

        std::fs::remove_file(&target).ok();
    }
}

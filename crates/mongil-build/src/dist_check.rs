//! Built-output checks: gate G12's enforceable core and gate G14's
//! dist half (plan §B.7, T2.10).
//!
//! G12 core, run when `validate --dist <dir>` is passed: every HTML
//! page in `dist/` contains the disclaimer string; no page exists for
//! a note whose status is `draft`; every internal link resolves to a
//! file; each page's total JavaScript is ≤ 6 KB gzipped.
//! G14 dist half: every image file in `dist/` lives under
//! `img/own/` (and is allowlisted) or `img/generated/`.

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;
use std::path::{Path, PathBuf};

/// The disclaimer string every page must carry (plan §B.5 footer;
/// fixed here so T3.1's `Disclaimer.astro` has exactly one spelling to
/// use).
pub const DISCLAIMER: &str = "Fan-made site. Not an official MONGIL: STAR DIVE or Netmarble site.";

pub const JS_LIMIT_BYTES: usize = 6 * 1024;

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else {
                out.push(path);
            }
        }
    }
}

fn rel(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Draft note dates (YYYY-MM-DD directory names) from the data dir.
fn draft_note_dates(data_dir: &Path) -> Vec<String> {
    let mut dates = Vec::new();
    let notes_dir = data_dir.join("notes");
    if let Ok(entries) = std::fs::read_dir(&notes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            let is_draft = text.lines().take(20).any(|l| l.trim() == "status: draft");
            if is_draft {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    dates.push(stem.to_string());
                }
            }
        }
    }
    dates
}

fn gzip_len(bytes: &[u8]) -> usize {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    if encoder.write_all(bytes).is_err() {
        return usize::MAX;
    }
    match encoder.finish() {
        Ok(v) => v.len(),
        Err(_) => usize::MAX,
    }
}

/// Run the G12 core against a built `dist/` (needs the data dir for
/// draft-note detection). Returns one error string per violation.
pub fn check_dist(data_dir: &Path, dist_dir: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let mut files = Vec::new();
    walk(dist_dir, &mut files);

    let html_files: Vec<&PathBuf> = files
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("html"))
        .collect();

    let draft_dates = draft_note_dates(data_dir);

    for page in &html_files {
        let rel_path = rel(dist_dir, page);
        let Ok(html) = std::fs::read_to_string(page) else {
            errors.push(format!("G12: cannot read dist page {rel_path}"));
            continue;
        };
        if !html.contains(DISCLAIMER) {
            errors.push(format!(
                "G12: dist page {rel_path} does not contain the disclaimer string"
            ));
        }
        for date in &draft_dates {
            if rel_path == format!("notes/{date}/index.html") {
                errors.push(format!(
                    "G12: dist renders notes/{date}, whose source note is still draft"
                ));
            }
        }
        // Internal links must resolve.
        for href in extract_hrefs(&html) {
            if !resolves(dist_dir, &href) {
                errors.push(format!(
                    "G12: dist page {rel_path} links to {href}, which does not resolve in dist"
                ));
            }
        }
        // Per-page JS budget (external src files + inline scripts).
        let mut js: Vec<u8> = Vec::new();
        for src in extract_script_srcs(&html) {
            let target = dist_dir.join(src.trim_start_matches('/'));
            if let Ok(bytes) = std::fs::read(&target) {
                js.extend_from_slice(&bytes);
            }
        }
        for inline in extract_inline_scripts(&html) {
            js.extend_from_slice(inline.as_bytes());
        }
        if !js.is_empty() {
            let gz = gzip_len(&js);
            if gz > JS_LIMIT_BYTES {
                errors.push(format!(
                    "G12: dist page {rel_path} ships {gz} bytes of gzipped JS (limit {JS_LIMIT_BYTES})"
                ));
            }
        }
    }

    // G14, dist half: images only under img/own/ (allowlisted) or
    // img/generated/.
    let allowlist = read_allowlist_from_dist(dist_dir);
    for file in &files {
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if !matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "avif" | "svg" | "ico"
        ) {
            continue;
        }
        let rel_path = rel(dist_dir, file);
        if let Some(rest) = rel_path.strip_prefix("img/own/") {
            if let Some(list) = &allowlist {
                if !list.iter().any(|a| a == rest) {
                    errors.push(format!(
                        "G14: dist image img/own/{rest} is not in ALLOWLIST.txt"
                    ));
                }
            }
        } else if !rel_path.starts_with("img/generated/") {
            errors.push(format!(
                "G14: dist image {rel_path} is outside img/own/ and img/generated/"
            ));
        }
    }

    errors
}

fn read_allowlist_from_dist(dist_dir: &Path) -> Option<Vec<String>> {
    let path = dist_dir.join("img/own/ALLOWLIST.txt");
    let text = std::fs::read_to_string(path).ok()?;
    Some(parse_allowlist(&text))
}

/// Parse an ALLOWLIST.txt body: one path per line, `#` comments and
/// blank lines ignored. Paths are relative to `img/own/`.
pub fn parse_allowlist(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.to_string())
        .collect()
}

/// Gate G14's source half: images under `site/public/img/` may only
/// live in `own/` (and must be listed in `own/ALLOWLIST.txt` when that
/// file exists) or in `generated/`.
pub fn check_site_images(site_dir: &Path) -> Vec<String> {
    let mut errors = Vec::new();
    let img_dir = site_dir.join("public/img");
    if !img_dir.is_dir() {
        return errors;
    }
    let mut files = Vec::new();
    walk(&img_dir, &mut files);
    let allowlist_path = img_dir.join("own/ALLOWLIST.txt");
    let allowlist = std::fs::read_to_string(&allowlist_path)
        .ok()
        .map(|t| parse_allowlist(&t));
    for file in &files {
        let rel_path = rel(&img_dir, file);
        if rel_path == "own/ALLOWLIST.txt" {
            continue;
        }
        if let Some(rest) = rel_path.strip_prefix("own/") {
            if let Some(list) = &allowlist {
                if !list.iter().any(|a| a == rest) {
                    errors.push(format!(
                        "G14: site/public/img/own/{rest} is not listed in ALLOWLIST.txt"
                    ));
                }
            }
        } else if !rel_path.starts_with("generated/") {
            errors.push(format!(
                "G14: site/public/img/{rel_path} is outside img/own/ and img/generated/"
            ));
        }
    }
    errors
}

fn extract_hrefs(html: &str) -> Vec<String> {
    extract_attr(html, "href=\"")
}

fn extract_script_srcs(html: &str) -> Vec<String> {
    // Only src attributes on <script> tags.
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(idx) = rest.find("<script") {
        let tag_end = rest[idx..].find('>').map(|e| idx + e).unwrap_or(rest.len());
        let tag = &rest[idx..tag_end];
        if let Some(src) = extract_attr(tag, "src=\"").into_iter().next() {
            out.push(src);
        }
        rest = &rest[tag_end..];
    }
    out
}

fn extract_inline_scripts(html: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(start) = rest.find("<script") {
        let Some(open_end) = rest[start..].find('>') else {
            break;
        };
        let tag = &rest[start..start + open_end];
        let body_start = start + open_end + 1;
        if tag.contains("src=") {
            rest = &rest[body_start..];
            continue;
        }
        let Some(close) = rest[body_start..].find("</script>") else {
            break;
        };
        out.push(rest[body_start..body_start + close].to_string());
        rest = &rest[body_start + close..];
    }
    out
}

fn extract_attr(html: &str, attr: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = html;
    while let Some(idx) = rest.find(attr) {
        let value_start = idx + attr.len();
        let Some(end) = rest[value_start..].find('"') else {
            break;
        };
        out.push(rest[value_start..value_start + end].to_string());
        rest = &rest[value_start + end..];
    }
    out
}

fn resolves(dist_dir: &Path, href: &str) -> bool {
    if !href.starts_with('/') || href.starts_with("//") {
        return true; // external or relative: not this gate's problem
    }
    let clean = href.split(['#', '?']).next().unwrap_or(href);
    if clean.is_empty() || clean == "/" {
        return dist_dir.join("index.html").is_file();
    }
    let trimmed = clean.trim_matches('/');
    let direct = dist_dir.join(trimmed);
    direct.is_file()
        || direct.join("index.html").is_file()
        || dist_dir.join(format!("{trimmed}.html")).is_file()
}

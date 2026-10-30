//! Acceptance tests for `mongil-build harvest-diff` (plan T2.1).

mod common;

use common::{run_bin, temp_dir, write_files};
use std::path::PathBuf;

const A: &str = r#"
[meta]
source = "kaiden"
harvest = "2026-11-03-am"
worker = "A"
read_mode = "text"

[[placement]]
character = "mina"
page_updated = "2026-11-01"
low = { tier = "T1", raw = "T1" }
high = { tier = "T0.5", raw = "T0.5" }
max = { tier = "T0.5", raw = "T0.5" }

[[placement]]
character = "narae"
low = { tier = "T0", raw = "T0" }
high = { tier = "T0", raw = "T0" }
max = { tier = "T0", raw = "T0" }

[[placement]]
character = "chloe"
low = { tier = "T1", raw = "T1" }
high = { tier = "T1", raw = "T1" }
max = { tier = "T1", raw = "T1" }
"#;

fn write_pair(name: &str, b_content: &str) -> (PathBuf, PathBuf) {
    let dir = temp_dir(name);
    let a = dir.join("kaiden.a.toml");
    let b = dir.join("kaiden.b.toml");
    write_files(&dir, &[("kaiden.a.toml", A), ("kaiden.b.toml", b_content)]);
    (a, b)
}

/// B identical except the meta bookkeeping that differs by design
/// (worker, read mode, read time) → clean, exit 0.
#[test]
fn harvest_diff_identical_ok() {
    let b = A
        .replace("worker = \"A\"", "worker = \"B\"")
        .replace("read_mode = \"text\"", "read_mode = \"screenshot\"");
    let (a, b) = write_pair("identical", &b);
    let out = run_bin(
        &["harvest-diff", a.to_str().unwrap(), b.to_str().unwrap()],
        None,
    );
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    assert!(out.stdout.contains("harvest-diff kaiden 2026-11-03-am"));
    assert!(out.stdout.contains("0 blocking"));
    assert!(out.stdout.contains("clean"));
}

/// One structured tier differs → blocking MISMATCH, exit 1.
#[test]
fn harvest_diff_tier_mismatch_blocks() {
    let b = A.replace(
        "high = { tier = \"T0.5\", raw = \"T0.5\" }\nmax = { tier = \"T0.5\", raw = \"T0.5\" }\n\n[[placement]]\ncharacter = \"narae\"",
        "high = { tier = \"T1\", raw = \"T0.5\" }\nmax = { tier = \"T0.5\", raw = \"T0.5\" }\n\n[[placement]]\ncharacter = \"narae\"",
    );
    assert_ne!(b, A, "test fixture replacement must change B");
    let (a, b) = write_pair("tier-mismatch", &b);
    let out = run_bin(
        &["harvest-diff", a.to_str().unwrap(), b.to_str().unwrap()],
        None,
    );
    assert_eq!(out.code, Some(1), "output: {}", out.combined());
    assert!(out.stdout.contains("MISMATCH"));
    assert!(out.stdout.contains("mina.high.tier"));
    assert!(out.stdout.contains("1 blocking"));
    assert!(out.stdout.contains("update blocked"));
}

/// A character present in A but absent in B → blocking MISSING, exit 1.
#[test]
fn harvest_diff_missing_character_blocks() {
    let b = A
        .split("[[placement]]\ncharacter = \"chloe\"")
        .next()
        .unwrap()
        .to_string();
    let (a, b) = write_pair("missing", &b);
    let out = run_bin(
        &["harvest-diff", a.to_str().unwrap(), b.to_str().unwrap()],
        None,
    );
    assert_eq!(out.code, Some(1), "output: {}", out.combined());
    assert!(out.stdout.contains("MISSING"));
    assert!(out.stdout.contains("chloe"));
    assert!(out.stdout.contains("A=present B=absent"));
    assert!(out.stdout.contains("update blocked"));
}

/// Only a raw string differs (trailing space) → warning, exit 0.
#[test]
fn harvest_diff_raw_only_warns() {
    let b = A.replace(
        "character = \"narae\"\nlow = { tier = \"T0\", raw = \"T0\" }",
        "character = \"narae\"\nlow = { tier = \"T0\", raw = \"T0 \" }",
    );
    assert_ne!(b, A, "test fixture replacement must change B");
    let (a, b) = write_pair("raw-warn", &b);
    let out = run_bin(
        &["harvest-diff", a.to_str().unwrap(), b.to_str().unwrap()],
        None,
    );
    assert_eq!(out.code, Some(0), "output: {}", out.combined());
    assert!(out.stdout.contains("WARN"));
    assert!(out.stdout.contains("narae.low.raw"));
    assert!(out.stdout.contains("(normalised equal)"));
    assert!(out.stdout.contains("0 blocking"));
    assert!(out.stdout.contains("clean"));
}

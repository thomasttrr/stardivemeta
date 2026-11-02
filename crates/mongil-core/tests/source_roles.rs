//! A voting source casts votes only while the registry says it may: its
//! `role` must be `voting` and its `status` must not be
//! `stale_check_only` / `unavailable` (method.rs `source_counts`).

use mongil_core::load::{load_dataset, Dataset};
use mongil_core::method::compute;
use mongil_core::model::{Level, SourceRole, SourceStatus};
use std::path::PathBuf;

fn fixture() -> Dataset {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/dataset-2026-10-29");
    load_dataset(&dir).expect("fixture loads")
}

/// How many characters carry a vote from `source` at Low.
fn votes_from(ds: &Dataset, source: &str) -> usize {
    compute(ds, &ds.method, Level::Low)
        .characters
        .iter()
        .filter(|c| c.votes.iter().any(|v| v.source == source))
        .count()
}

fn with_pg(edit: impl FnOnce(&mut mongil_core::model::Source)) -> Dataset {
    let mut ds = fixture();
    let pg = ds
        .sources
        .sources
        .iter_mut()
        .find(|s| s.id == "pg")
        .expect("fixture registers pg");
    edit(pg);
    ds
}

#[test]
fn voting_current_source_votes() {
    assert!(votes_from(&fixture(), "pg") > 0);
}

#[test]
fn excluded_source_casts_no_votes() {
    let ds = with_pg(|s| {
        s.role = SourceRole::Excluded;
        s.status = None;
    });
    assert_eq!(votes_from(&ds, "pg"), 0);
}

#[test]
fn check_only_source_casts_no_votes() {
    let ds = with_pg(|s| {
        s.role = SourceRole::CheckOnly;
        s.status = Some(SourceStatus::Current);
    });
    assert_eq!(votes_from(&ds, "pg"), 0);
}

#[test]
fn unregistered_source_casts_no_votes() {
    let mut ds = fixture();
    ds.sources.sources.retain(|s| s.id != "pg");
    assert_eq!(votes_from(&ds, "pg"), 0);
}

#[test]
fn stale_source_casts_no_votes() {
    let ds = with_pg(|s| s.status = Some(SourceStatus::StaleCheckOnly));
    assert_eq!(votes_from(&ds, "pg"), 0);
}

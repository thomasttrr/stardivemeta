//! mongil-core: data model, Method v1.0 engine, derived lists, codes,
//! snapshots and diffs for StarDiveMeta.
//!
//! Landed so far: `model` + `load` (T1.2), the 2026-10-29 fixture
//! dataset (T1.3), and `method` (T1.4). Contract types, codes,
//! snapshots and diffs land in later tasks.

pub mod load;
pub mod method;
pub mod model;

pub use load::{load_dataset, Dataset, LoadError};
pub use model::MethodConfig;

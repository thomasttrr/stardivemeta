//! mongil-core: data model, Method v1.0 engine, derived lists, codes,
//! snapshots and diffs for StarDiveMeta.
//!
//! Landed so far: `model` + `load` (T1.2). The method engine, contract
//! types, codes, snapshots and diffs land in later tasks.

pub mod load;
pub mod model;

pub use load::{load_dataset, Dataset, LoadError};
pub use model::MethodConfig;

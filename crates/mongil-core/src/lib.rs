//! mongil-core: data model, Method v1.0 engine, derived lists, codes,
//! snapshots and diffs for StarDiveMeta.
//!
//! Landed so far: `model` + `load` (T1.2), the 2026-10-29 fixture dataset
//! (T1.3), `method` (T1.4), the B.6 regression suite (T1.5), `derive`
//! (the T3.7 Beginner lists, pulled forward because §B.6 tests them),
//! `contract` (T1.6, the §B.2 JSON wire types), and `snapshot` (T2.9:
//! frozen snapshots + diffs). The codes status engine (T2.7, §B.3
//! rules) lives in `mongil-build`'s `emit`, next to the codes.json it
//! produces.

pub mod contract;
pub mod derive;
pub mod load;
pub mod method;
pub mod model;
pub mod snapshot;

pub use load::{load_dataset, Dataset, LoadError};
pub use model::MethodConfig;

//! mongil-build CLI (plan T1.6): one binary does everything the
//! pipeline needs — `validate`, `emit`, `snapshot`, `diff`,
//! `harvest-diff`, `png`, `lane`, `tripwire`, `note`.
//!
//! Status: `validate`/`emit` (T1.6–T1.7), `harvest-diff` (T2.1),
//! `snapshot`/`diff` (T2.9), the gate cores (T2.10), the `lane`
//! path-classifier core (T4.6's classification) and `png` (T3.9)
//! are implemented. Still stubbed with explicit "not yet
//! implemented" errors: `note` (T4.4), `tripwire` (T4.5).

mod dist_check;
mod emit;
mod harvest_diff;
mod lane;
mod png;
mod snapshots;
mod validate;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "mongil-build", version, about = "StarDiveMeta build pipeline")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the §B.7 validation gates against a data dir.
    Validate {
        /// Dataset directory (e.g. `data`).
        #[arg(long)]
        data: PathBuf,
        /// Site directory (e.g. `site`): enables gate G14's image checks.
        #[arg(long)]
        site: Option<PathBuf>,
        /// Built output directory (e.g. `site/dist`): enables gate
        /// G12's dist checks and G14's dist half.
        #[arg(long)]
        dist: Option<PathBuf>,
    },
    /// Emit the §B.2 contract JSON (tierlist/codes/history/meta).
    Emit {
        /// Dataset directory (e.g. `data`).
        #[arg(long)]
        data: PathBuf,
        /// Output directory (e.g. `site/src/generated`).
        #[arg(long)]
        out: PathBuf,
    },
    /// Write a snapshot of the current dataset (T2.9).
    Snapshot {
        #[arg(long)]
        data: PathBuf,
        #[arg(long)]
        id: Option<String>,
    },
    /// Diff two snapshots, or a snapshot against the working data (T2.9).
    Diff {
        #[arg(long)]
        data: PathBuf,
        #[arg(long)]
        from: String,
        /// A snapshot id, or `working`.
        #[arg(long)]
        to: String,
        #[arg(long, default_value = "md", value_parser = ["md", "json"])]
        format: String,
    },
    /// Compare the A and B reads of one source harvest (T2.1).
    HarvestDiff {
        /// Worker A's harvest file.
        a: PathBuf,
        /// Worker B's harvest file.
        b: PathBuf,
    },
    /// Render the official list PNGs (T3.9).
    Png {
        #[arg(long)]
        data: PathBuf,
        #[arg(long)]
        out: PathBuf,
    },
    /// Classify a change set as `data-only` or `needs-tom` (T4.6).
    Lane {
        #[arg(long, default_value = "origin/main")]
        base: String,
    },
    /// Evaluate the accuracy tripwire over the errata record (T4.5).
    Tripwire {
        #[arg(long)]
        data: PathBuf,
    },
    /// Generate a "what moved" note from a snapshot diff (T4.4).
    Note {
        #[arg(long)]
        data: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        date: String,
    },
}

fn not_yet_implemented(command: &str, task: &str) -> ExitCode {
    eprintln!("mongil-build {command}: not yet implemented (lands in {task})");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Command::Validate { data, site, dist } => {
            let outcome = validate::run(&data, site.as_deref(), dist.as_deref());
            validate::print_and_exit(&outcome)
        }
        Command::Emit { data, out } => match emit::run(&data, &out) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("emit: {e}");
                ExitCode::FAILURE
            }
        },
        Command::Snapshot { data, id } => snapshots::run_snapshot(&data, id.as_deref()),
        Command::Diff {
            data,
            from,
            to,
            format,
        } => snapshots::run_diff(&data, &from, &to, &format),
        Command::HarvestDiff { a, b } => harvest_diff::run(&a, &b),
        Command::Png { data, out } => png::run(&data, &out),
        Command::Lane { base } => lane::run(&base),
        Command::Tripwire { .. } => not_yet_implemented("tripwire", "T4.5"),
        Command::Note { .. } => not_yet_implemented("note", "T4.4"),
    }
}

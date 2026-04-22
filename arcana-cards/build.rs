//! Bootstrap script: writes a minimal stub for
//! `src/generated/_scratch/candidate.rs` if the file is missing.
//!
//! Why: `candidate.rs` is the volatile scratch file that
//! `arcana-gen::verify` overwrites per run. It's gitignored, so on
//! a fresh clone the file doesn't exist, and arcana-cards would
//! fail to compile (because `_scratch/mod.rs` declares
//! `pub mod candidate;`). This script makes arcana-cards
//! self-bootstrapping: first build creates the stub, verify
//! overwrites it at runtime.

use std::path::Path;

const CANDIDATE_PATH: &str = "src/generated/_scratch/candidate.rs";

const STUB: &str = "//! Volatile scratch — overwritten by `arcana-gen::verify`.\n\
                    //! This committed-as-placeholder state is what `arcana-cards/build.rs`\n\
                    //! writes on fresh clones so the crate compiles before verify runs.\n\
                    \n\
                    pub fn _noop() {}\n";

fn main() {
    // build.rs reruns on its own changes and on the candidate path
    // changes. We deliberately do NOT rerun-if-changed on
    // candidate.rs itself, because verify overwrites it and we
    // don't want every verify run to force a rebuild of everything
    // downstream of the script.
    println!("cargo:rerun-if-changed=build.rs");

    let path = Path::new(CANDIDATE_PATH);
    if !path.exists() {
        std::fs::write(path, STUB).expect("writing candidate stub");
    }
}

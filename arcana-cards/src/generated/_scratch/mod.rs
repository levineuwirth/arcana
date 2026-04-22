//! Volatile scratch space for `arcana-gen::verify`.
//!
//! [`candidate`] is overwritten every verify run. The file is
//! gitignored; the committed bootstrap is the stub written by
//! `arcana-cards/build.rs` on first build, ensuring the crate
//! compiles out-of-the-box before verify has ever run.
//!
//! v1 declares exactly one candidate slot. The parallelism path is
//! to preallocate `candidate_0` through `candidate_N` here and
//! teach verify to lease slots. No callers today use a non-default
//! `scratch_slug`, and the config field panics at `cargo check`
//! time if they try (missing module).

pub mod candidate;

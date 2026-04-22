//! Generated card code.
//!
//! This is the staging area for cards produced by `arcana-gen`'s
//! card-generation pipeline. It has two responsibilities:
//!
//! * [`_scratch`] — a single volatile candidate file that
//!   `arcana-gen::verify` overwrites per run, used to compile-check
//!   model-generated card source.
//! * `<set>/<slug>.rs` — promoted generations, one file per card,
//!   set-grouped to mirror the hand-written seed layout. These get
//!   added here after verify passes **and** a human spot-checks
//!   semantic correctness; they move out to `arcana-cards/src/<set>/`
//!   in a follow-up pass.
//!
//! Everything in this subtree is treated as intermediate — no
//! downstream code should take a long-lived dependency on a specific
//! path within `generated/`.

pub mod _scratch;

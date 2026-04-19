//! Engine-wide collection type aliases.
//!
//! Every `HashMap` and `HashSet` in `arcana-core` resolves to the
//! `rustc-hash` variants via the aliases in this module, not to the
//! standard library's. Two reasons:
//!
//! 1. **Determinism.** `std::collections::HashMap` seeds each instance
//!    with a fresh [`RandomState`], so two games built from the same
//!    seed in two separate processes iterate their object arenas in
//!    different orders. Any replay recorded in one process and played
//!    back in another would diverge the moment a code path depends
//!    on hash iteration order (legal-action enumeration, SBA
//!    ordering, trigger collection). Determinism is the top-level
//!    principle P5 in the engine spec — it has to hold across
//!    processes, not just within one.
//! 2. **Speed.** `FxHasher` is the hasher `rustc` itself uses. For
//!    small integer and tuple keys (the shape of every engine key)
//!    it's meaningfully faster than SipHash, which matters in the
//!    hot paths `arcana-ai` hammers during self-play.
//!
//! `FxHasher` is **not cryptographic**. Using it for anything exposed
//! to adversarial input (networked session keys, user-provided data
//! hashed into a map) would be a hazard. Inside the engine all keys
//! are engine-generated integer ids, so the trade is clean.
//!
//! Tests that only hash locally to dedupe test inputs — and whose
//! assertions are order-independent — can keep using `std::collections`
//! if it reads more clearly there, but default to these aliases.
//!
//! [`RandomState`]: std::collections::hash_map::RandomState

pub use rustc_hash::FxHashMap as HashMap;
pub use rustc_hash::FxHashSet as HashSet;

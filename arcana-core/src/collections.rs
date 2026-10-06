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
//! No file in this crate names `std::collections::HashMap` or
//! `std::collections::HashSet`, tests included; the `no_std_hash_maps`
//! test below reads the source tree and fails on the first one. The
//! rule became a test on 2026-10-06, after `state.rs` had carried a
//! std map for the last-known-information table — exactly the
//! trigger-collection path the determinism argument above names.
//!
//! [`RandomState`]: std::collections::hash_map::RandomState

pub use rustc_hash::FxHashMap as HashMap;
pub use rustc_hash::FxHashSet as HashSet;

#[cfg(test)]
mod tests {
    /// Every `.rs` file under `src/` except this one is free of the
    /// standard-library hash collections. A grep-level guard, because a
    /// behavioral test for cross-process iteration order needs two
    /// processes and a game that reaches simultaneous LKI-sourced
    /// triggers; this catches the import before it matters.
    #[test]
    fn no_std_hash_maps() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut offenders = Vec::new();
        let mut stack = vec![root];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).expect("read src dir") {
                let path = entry.expect("dir entry").path();
                if path.is_dir() { stack.push(path); continue; }
                if path.extension().and_then(|e| e.to_str()) != Some("rs") { continue; }
                if path.file_name().and_then(|n| n.to_str()) == Some("collections.rs") { continue; }
                let text = std::fs::read_to_string(&path).expect("read source file");
                for (i, line) in text.lines().enumerate() {
                    if line.contains("std::collections::HashMap")
                        || line.contains("std::collections::HashSet")
                    {
                        offenders.push(format!("{}:{}: {}", path.display(), i + 1, line.trim()));
                    }
                }
            }
        }
        assert!(
            offenders.is_empty(),
            "std hash collections in arcana-core (use crate::collections):\n{}",
            offenders.join("\n")
        );
    }
}

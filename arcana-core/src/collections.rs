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

/// Byte offsets at which `text` names a standard-library hash collection
/// through `std::collections`: a direct path (`std::collections::HashMap`),
/// a brace group that includes one (`std::collections::{HashMap, HashSet}`,
/// across lines or not), or a glob (`std::collections::*`). Paths into
/// `hash_map`/`hash_set` submodules (`DefaultHasher`, `Entry`) are not
/// collections and are allowed.
#[cfg(test)]
fn std_hash_collection_offsets(text: &str) -> Vec<usize> {
    const PREFIX: &str = "std::collections::";
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(rel) = text[from..].find(PREFIX) {
        let at = from + rel;
        let rest = &text[at + PREFIX.len()..];
        let offending = if rest.starts_with("HashMap") || rest.starts_with("HashSet") || rest.starts_with('*') {
            true
        } else if let Some(body) = rest.strip_prefix('{') {
            // Take the brace group, nested braces included, and look for
            // the collection names as whole tokens inside it.
            let mut depth = 1usize;
            let mut end = body.len();
            for (i, c) in body.char_indices() {
                match c {
                    '{' => depth += 1,
                    '}' => { depth -= 1; if depth == 0 { end = i; break; } }
                    _ => {}
                }
            }
            body[..end]
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .any(|tok| tok == "HashMap" || tok == "HashSet")
        } else {
            false
        };
        if offending { out.push(at); }
        from = at + PREFIX.len();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::std_hash_collection_offsets;

    fn line_of(text: &str, offset: usize) -> usize {
        text[..offset].matches('\n').count() + 1
    }

    #[test]
    fn scanner_catches_every_import_shape() {
        let direct = "let m: std::collections::HashMap<u8, u8> = Default::default();";
        assert_eq!(std_hash_collection_offsets(direct).len(), 1, "direct path");
        let grouped = "use std::collections::{HashMap, HashSet};";
        assert_eq!(std_hash_collection_offsets(grouped).len(), 1, "grouped import, the state.rs shape");
        let multiline = "use std::collections::{\n    BTreeMap,\n    HashSet,\n};";
        assert_eq!(std_hash_collection_offsets(multiline).len(), 1, "multiline group");
        let nested = "use std::collections::{hash_map::{Entry, HashMap}, VecDeque};";
        assert_eq!(std_hash_collection_offsets(nested).len(), 1, "nested group");
        let glob = "use std::collections::*;";
        assert_eq!(std_hash_collection_offsets(glob).len(), 1, "glob");
    }

    #[test]
    fn scanner_allows_non_collection_paths() {
        let hasher = "use std::collections::hash_map::DefaultHasher;";
        assert!(std_hash_collection_offsets(hasher).is_empty(), "a hasher is not a map");
        let others = "use std::collections::{BTreeMap, VecDeque, BinaryHeap};";
        assert!(std_hash_collection_offsets(others).is_empty(), "ordered collections are fine");
        let alias = "use crate::collections::{HashMap, HashSet};";
        assert!(std_hash_collection_offsets(alias).is_empty(), "the alias is the point");
    }

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
                for at in std_hash_collection_offsets(&text) {
                    let line = text[at..].lines().next().unwrap_or("").trim();
                    offenders.push(format!("{}:{}: {}", path.display(), line_of(&text, at), line));
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

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

/// Byte offsets of every path in `text` that reaches a standard-library
/// hash collection through `std`. Each `std` token that starts a path is
/// expanded as a use-tree (`std::{collections::{HashMap, hash_map::Entry}}`
/// becomes two full paths) and each full path is judged by its segments:
/// `std::collections::HashMap`, `::HashSet`, `::*`, the `hash_map` and
/// `hash_set` submodules' `HashMap`/`HashSet`/`*`, and any import that
/// stops at the `collections`, `hash_map` or `hash_set` module itself
/// (which would let a later `hash_map::HashMap` escape) are offenders;
/// `std::*` is too. `hash_map::DefaultHasher`, `::Entry`, `::RandomState`
/// and the ordered collections are not.
#[cfg(test)]
fn std_hash_collection_offsets(text: &str) -> Vec<usize> {
    #[derive(Clone, Copy, PartialEq)]
    enum Tok<'a> { Ident(&'a str), PathSep, Open, Close, Comma, Star, Other }
    let bytes = text.as_bytes();
    let mut toks: Vec<(Tok, usize)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_alphabetic() || c == '_' {
            let st = i;
            while i < bytes.len() && ((bytes[i] as char).is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            toks.push((Tok::Ident(&text[st..i]), st));
            continue;
        }
        if c.is_ascii_digit() {
            // a digit run glued to a preceding identifier was consumed above;
            // a bare number is not part of any path
            while i < bytes.len() && (bytes[i] as char).is_ascii_alphanumeric() { i += 1; }
            toks.push((Tok::Other, i));
            continue;
        }
        if c.is_whitespace() { i += 1; continue; }
        if bytes[i..].starts_with(b"::") { toks.push((Tok::PathSep, i)); i += 2; continue; }
        toks.push((match c { '{' => Tok::Open, '}' => Tok::Close, ',' => Tok::Comma, '*' => Tok::Star, _ => Tok::Other }, i));
        i += 1;
    }

    // Parse a path starting at `at`; return its flattened segment lists
    // and the index after it. A brace group ends the path.
    fn parse_path<'a>(toks: &[(Tok<'a>, usize)], mut at: usize) -> (Vec<Vec<&'a str>>, usize) {
        let mut prefix: Vec<&'a str> = Vec::new();
        loop {
            match toks.get(at).map(|t| t.0) {
                Some(Tok::Ident(s)) => { prefix.push(s); at += 1; }
                Some(Tok::Star) => { prefix.push("*"); at += 1; return (vec![prefix], at); }
                Some(Tok::Open) => {
                    at += 1;
                    let mut out = Vec::new();
                    loop {
                        match toks.get(at).map(|t| t.0) {
                            Some(Tok::Close) => { at += 1; break; }
                            Some(Tok::Comma) => { at += 1; }
                            Some(_) => {
                                let (subs, next) = parse_path(toks, at);
                                if next == at { at += 1; continue; } // unparseable token: skip it
                                at = next;
                                for sub in subs {
                                    let mut full = prefix.clone();
                                    full.extend(sub);
                                    out.push(full);
                                }
                            }
                            None => break,
                        }
                    }
                    return (out, at);
                }
                _ => return (if prefix.is_empty() { Vec::new() } else { vec![prefix] }, at),
            }
            if toks.get(at).map(|t| t.0) == Some(Tok::PathSep) { at += 1; } else { return (vec![prefix], at); }
        }
    }

    fn offends(path: &[&str]) -> bool {
        if path.first() != Some(&"std") { return false; }
        let is_set = |s: &str| s == "HashMap" || s == "HashSet" || s == "*";
        match path.get(1) {
            Some(&"*") => true,
            Some(&"collections") => match path.get(2) {
                None => true,                                  // `use std::collections;`
                Some(s) if is_set(s) => true,
                Some(&"hash_map") | Some(&"hash_set") => match path.get(3) {
                    None => true,                              // `use std::collections::hash_map;`
                    Some(s) => is_set(s),
                },
                Some(_) => false,                              // BTreeMap, VecDeque, ...
            },
            _ => false,
        }
    }

    let mut out = Vec::new();
    let mut at = 0;
    while at < toks.len() {
        if toks[at].0 == Tok::Ident("std") && toks.get(at + 1).map(|t| t.0) == Some(Tok::PathSep) {
            let (paths, next) = parse_path(&toks, at);
            if paths.iter().any(|p| offends(p)) { out.push(toks[at].1); }
            at = next.max(at + 1);
        } else {
            at += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::std_hash_collection_offsets;

    fn line_of(text: &str, offset: usize) -> usize {
        text[..offset].matches('\n').count() + 1
    }

    fn offends(src: &str) -> bool { !std_hash_collection_offsets(src).is_empty() }

    #[test]
    fn scanner_catches_every_import_shape() {
        for (src, why) in [
            ("let m: std::collections::HashMap<u8, u8> = Default::default();", "direct path"),
            ("use std::collections::{HashMap, HashSet};", "grouped import, the state.rs shape"),
            ("use std::collections::{\n    BTreeMap,\n    HashSet,\n};", "multiline group"),
            ("use std::collections::{hash_map::{Entry, HashMap}, VecDeque};", "nested group"),
            ("use std::collections::*;", "glob"),
            ("use std::collections::{*};", "braced glob"),
            ("use std::collections::hash_map::HashMap;", "submodule re-export"),
            ("use std::collections::hash_set::HashSet;", "hash_set re-export"),
            ("use std::collections::hash_map::*;", "submodule glob"),
            ("use std::{collections::{HashMap, HashSet}};", "group above collections"),
            ("use std::{io, collections::hash_map::HashMap};", "mixed group above collections"),
            ("use std::collections::HashMap as Map;", "rename"),
            ("use std::collections;", "module import, lets `collections::HashMap` escape"),
            ("use std::collections::hash_map;", "submodule import, lets `hash_map::HashMap` escape"),
            ("use std::collections::hash_map as hm;", "renamed submodule import"),
            ("let m = ::std::collections::HashMap::<u8, u8>::default();", "leading :: and turbofish"),
            ("use std::*;", "std glob"),
        ] {
            assert!(offends(src), "missed: {why}: {src}");
        }
    }

    #[test]
    fn scanner_allows_non_collection_paths() {
        for (src, why) in [
            ("use std::collections::hash_map::DefaultHasher;", "a hasher is not a map"),
            ("use std::collections::hash_map::{DefaultHasher, Entry, RandomState};", "hasher, entry, state"),
            ("use std::collections::{BTreeMap, VecDeque, BinaryHeap};", "ordered collections"),
            ("use std::collections::{BTreeMap, hash_map::Entry};", "mixed, nothing hashed"),
            ("use crate::collections::{HashMap, HashSet};", "the alias is the point"),
            ("use std::{io, fmt};", "unrelated std group"),
            ("let my_std = 1; my_std::x", "not the std token"),
            ("use rustc_hash::FxHashMap as HashMap;", "the alias definition itself"),
        ] {
            assert!(!offends(src), "false positive: {why}: {src}");
        }
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

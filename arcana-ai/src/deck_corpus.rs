//! Load real decklists into the gauntlet and report catalog coverage.
//!
//! The bridge from an external decklist corpus (MTGTop8 / Topdeck / any source
//! that can emit `N CardName` lines) to the [`crate::deckeval_runner`] gauntlet.
//! It is deliberately SOURCE-AGNOSTIC: every source reduces to Arena/MTGO list
//! text, which [`arcana_core::deck::parse_deck_text`] already parses (set-code
//! stripping, `4x` counts, DFC/split `A // B` fallback), collecting names it
//! can't resolve in [`arcana_core::deck::ParsedDeck::unresolved`].
//!
//! That `unresolved` field is the whole point: our catalog implements ~20.5k of
//! ~30.9k real cards, so a real list is only playable if EVERY maindeck name
//! resolves. [`coverage_report`] tallies the resolve rate per format so we can
//! pick the best-covered formats (eternal/older formats fare worse — reserved-
//! list and unimplemented staples), and [`playable_decks`] hands the fully-
//! covered, size-valid lists to the gauntlet as [`Deck`]s.
//!
//! NOTE — maindeck-only / coverage ≠ fidelity: today's source (the MTGTop8
//! converter) emits MAINDECKS only, and `unresolved` is tracked over the whole
//! list. A future source that includes sideboards would need unresolved tracked
//! per section, so a sideboard miss can't disqualify a playable maindeck. Also,
//! "resolves" is not "faithful" — some registered cards are approximated (see
//! `docs/gauntlet-results/approximated-cards.txt`); a per-deck fidelity score is
//! a planned addition.
//!
//! A source-specific adapter (e.g. Kaggle MTGTop8 CSV → `(name, format, text)`)
//! slots in on top; this module is everything downstream of that.

use std::collections::HashMap;

use arcana_core::deck::{parse_deck_text, ParsedDeck};
use arcana_core::registry::CardRegistry;

use crate::deckeval::Deck;

// =============================================================================
// Loaded deck
// =============================================================================

/// A real decklist parsed against the catalog, retaining its format tag and the
/// names that didn't resolve (the per-deck coverage signal).
#[derive(Clone, Debug)]
pub struct LoadedDeck {
    pub name: String,
    pub format: String,
    pub parsed: ParsedDeck,
}

impl LoadedDeck {
    /// Maindeck cards counting copies (resolved only — `parse_deck_text` drops
    /// unresolved names from `main`).
    pub fn main_count(&self) -> u32 {
        self.parsed.main.iter().map(|(_, c)| c).sum()
    }

    /// Unresolved (missing-from-catalog) maindeck names, counting copies.
    pub fn unresolved_count(&self) -> u32 {
        self.parsed.unresolved.iter().map(|(_, c)| c).sum()
    }

    /// True when every maindeck name resolved against the catalog.
    pub fn fully_covered(&self) -> bool {
        self.parsed.unresolved.is_empty()
    }

    /// Coverage fraction by card count: resolved / (resolved + unresolved), in
    /// `[0, 1]` (1.0 for an empty list).
    pub fn coverage(&self) -> f32 {
        let resolved = self.main_count() as f32;
        let total = resolved + self.unresolved_count() as f32;
        if total == 0.0 {
            1.0
        } else {
            resolved / total
        }
    }

    /// True when fully covered AND maindeck size in `[min_main, max_main]`.
    pub fn is_playable(&self, min_main: u32, max_main: u32) -> bool {
        self.fully_covered() && (min_main..=max_main).contains(&self.main_count())
    }

    /// Flatten the maindeck into the [`Deck`] shape the gauntlet wants (a flat
    /// multiset of card ids). Only meaningful when [`Self::fully_covered`].
    pub fn to_deck(&self) -> Deck {
        let mut cards = Vec::with_capacity(self.main_count() as usize);
        for (id, n) in &self.parsed.main {
            for _ in 0..*n {
                cards.push(*id);
            }
        }
        Deck {
            name: self.name.clone(),
            cards,
        }
    }
}

/// Parse one real decklist (Arena/MTGO `N CardName` text) against the catalog.
pub fn load_deck(
    name: impl Into<String>,
    format: impl Into<String>,
    list_text: &str,
    reg: &CardRegistry,
) -> LoadedDeck {
    let parsed = parse_deck_text(list_text, reg);
    // An `About`/`Name` header (e.g. the archetype) wins; the passed `name`
    // (file stem) is the fallback when the list carries no name.
    let name = if parsed.name.is_empty() {
        name.into()
    } else {
        parsed.name.clone()
    };
    LoadedDeck { name, format: format.into(), parsed }
}

/// Convenience: load every `*.txt` decklist in `dir` under one format tag (the
/// file stem is the deck name). Used by the `#[ignore]` gauntlet template; a
/// CSV/JSON source adapter calls [`load_deck`] directly instead.
pub fn decks_from_dir(
    dir: impl AsRef<std::path::Path>,
    format: &str,
    reg: &CardRegistry,
) -> std::io::Result<Vec<LoadedDeck>> {
    let mut out = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false))
        .collect();
    entries.sort(); // deterministic order
    for path in entries {
        let text = std::fs::read_to_string(&path)?;
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("deck")
            .to_string();
        out.push(load_deck(name, format, &text, reg));
    }
    Ok(out)
}

// =============================================================================
// Coverage report
// =============================================================================

/// Per-format coverage of a corpus.
#[derive(Clone, Debug)]
pub struct FormatCoverage {
    pub format: String,
    pub decks: u32,
    /// Decks where every maindeck name resolved.
    pub fully_covered: u32,
    /// Mean per-deck coverage fraction (by card count).
    pub mean_coverage: f32,
    /// Decks that are fully covered AND maindeck size in `[min_main, max_main]`.
    pub playable: u32,
}

/// A per-format coverage roll-up plus the most-frequently-missing cards.
pub struct CoverageReport {
    pub formats: Vec<FormatCoverage>,
    /// Most-frequently-missing card names across the whole corpus, as
    /// `(name, total copies missing, #decks affected)`, sorted by #decks then
    /// copies, descending.
    pub top_missing: Vec<(String, u32, u32)>,
    pub min_main: u32,
    pub max_main: u32,
}

/// Roll up coverage over a corpus. `min_main`/`max_main` define a valid
/// maindeck size for the "playable" tally (e.g. `60..=60` for constructed).
pub fn coverage_report(decks: &[LoadedDeck], min_main: u32, max_main: u32) -> CoverageReport {
    // Per-format aggregation, insertion-ordered.
    let mut order: Vec<String> = Vec::new();
    let mut by_fmt: HashMap<String, (u32, u32, f32, u32)> = HashMap::new(); // decks, full, cov_sum, playable
    for d in decks {
        let e = by_fmt.entry(d.format.clone()).or_insert_with(|| {
            order.push(d.format.clone());
            (0, 0, 0.0, 0)
        });
        e.0 += 1;
        if d.fully_covered() {
            e.1 += 1;
        }
        e.2 += d.coverage();
        if d.is_playable(min_main, max_main) {
            e.3 += 1;
        }
    }
    let mut formats: Vec<FormatCoverage> = order
        .into_iter()
        .map(|f| {
            let (decks, full, cov_sum, playable) = by_fmt[&f];
            FormatCoverage {
                format: f,
                decks,
                fully_covered: full,
                mean_coverage: if decks == 0 { 0.0 } else { cov_sum / decks as f32 },
                playable,
            }
        })
        .collect();
    formats.sort_by(|a, b| b.decks.cmp(&a.decks));

    // Most-frequently-missing cards across the corpus.
    let mut missing: HashMap<String, (u32, u32)> = HashMap::new(); // copies, deck_count
    for d in decks {
        for (name, copies) in &d.parsed.unresolved {
            let e = missing.entry(name.clone()).or_insert((0, 0));
            e.0 += copies;
            e.1 += 1;
        }
    }
    let mut top_missing: Vec<(String, u32, u32)> =
        missing.into_iter().map(|(n, (c, d))| (n, c, d)).collect();
    top_missing.sort_by(|a, b| b.2.cmp(&a.2).then(b.1.cmp(&a.1)).then(a.0.cmp(&b.0)));
    top_missing.truncate(25);

    CoverageReport {
        formats,
        top_missing,
        min_main,
        max_main,
    }
}

impl CoverageReport {
    /// Human-readable coverage table.
    pub fn format_table(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "coverage (playable = fully-covered & main in {}..={}):\n",
            self.min_main, self.max_main
        ));
        s.push_str(&format!(
            "{:>12} | {:>6} {:>10} {:>8} {:>9}\n",
            "format", "decks", "full-cover", "mean-cov", "playable"
        ));
        for f in &self.formats {
            s.push_str(&format!(
                "{:>12} | {:>6} {:>10} {:>7.1}% {:>9}\n",
                f.format,
                f.decks,
                f.fully_covered,
                f.mean_coverage * 100.0,
                f.playable
            ));
        }
        if !self.top_missing.is_empty() {
            s.push_str("top missing (name: copies / #decks):\n");
            for (name, copies, ndecks) in self.top_missing.iter().take(15) {
                s.push_str(&format!("  {name}: {copies} / {ndecks}\n"));
            }
        }
        s
    }
}

/// Filter a corpus to the gauntlet-ready decks: fully covered AND maindeck size
/// in `[min_main, max_main]`, converted to [`Deck`].
pub fn playable_decks(decks: &[LoadedDeck], min_main: u32, max_main: u32) -> Vec<Deck> {
    decks
        .iter()
        .filter(|d| d.is_playable(min_main, max_main))
        .map(|d| d.to_deck())
        .collect()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Cards confirmed present in the catalog (resolve via card_id_by_name).
    fn corpus(reg: &CardRegistry) -> Vec<LoadedDeck> {
        vec![
            load_deck(
                "burn",
                "modern",
                "Deck\n10 Lightning Bolt\n10 Counterspell\n40 Mountain\n",
                reg,
            ),
            load_deck(
                "elves",
                "modern",
                "Deck\n4 Llanowar Elves\n4 Thoughtseize\n52 Forest\n",
                reg,
            ),
            // Has a card our catalog lacks → not fully covered.
            load_deck(
                "fake",
                "legacy",
                "Deck\n4 Totally Fake Xyz\n56 Island\n",
                reg,
            ),
        ]
    }

    #[test]
    fn loads_and_reports_coverage() {
        let reg = arcana_cards::build_catalog();
        let decks = corpus(&reg);

        // Per-deck coverage.
        assert!(decks[0].fully_covered() && decks[1].fully_covered());
        assert!(!decks[2].fully_covered(), "fake-card deck should miss");
        assert_eq!(decks[0].main_count(), 60);
        assert_eq!(decks[2].unresolved_count(), 4);
        assert!((decks[2].coverage() - 56.0 / 60.0).abs() < 1e-4);

        let report = coverage_report(&decks, 60, 60);
        // modern (2 decks) sorts before legacy (1 deck).
        assert_eq!(report.formats[0].format, "modern");
        assert_eq!(report.formats[0].decks, 2);
        assert_eq!(report.formats[0].fully_covered, 2);
        assert_eq!(report.formats[0].playable, 2);
        let legacy = report.formats.iter().find(|f| f.format == "legacy").unwrap();
        assert_eq!(legacy.fully_covered, 0);
        assert_eq!(legacy.playable, 0);
        // The fake card tops the missing list.
        assert_eq!(report.top_missing[0].0, "Totally Fake Xyz");
        assert_eq!(report.top_missing[0].1, 4); // copies
        assert_eq!(report.top_missing[0].2, 1); // decks
        assert!(report.format_table().contains("modern"));
    }

    #[test]
    fn playable_decks_filters_and_flattens() {
        let reg = arcana_cards::build_catalog();
        let decks = corpus(&reg);
        let playable = playable_decks(&decks, 60, 60);
        // Only the two fully-covered 60-card decks survive.
        assert_eq!(playable.len(), 2);
        for d in &playable {
            assert_eq!(d.cards.len(), 60, "deck {} flattened to 60 cards", d.name);
        }
        // Size filter excludes a fully-covered but wrong-size deck.
        let small = load_deck("tiny", "modern", "Deck\n4 Lightning Bolt\n", &reg);
        assert!(small.fully_covered() && !small.is_playable(60, 60));
        assert_eq!(playable_decks(&[small], 60, 60).len(), 0);
    }

    /// Scan a converted corpus (base dir with per-format subdirs of `*.txt`)
    /// and print catalog coverage per format — the "which format is viable"
    /// measurement. Run:
    /// `KAGGLE_DECKS=/path cargo test -p arcana-ai --release coverage_scan -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn coverage_scan() {
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS to the corpus base dir");
        let reg = arcana_cards::build_catalog();
        let mut all = Vec::new();
        let mut subdirs: Vec<_> = std::fs::read_dir(&base)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        subdirs.sort();
        for sub in subdirs {
            let fmt = sub.file_name().unwrap().to_str().unwrap().to_string();
            let decks = decks_from_dir(&sub, &fmt, &reg).unwrap();
            all.extend(decks);
        }
        let report = coverage_report(&all, 60, 60);
        println!("\n{}", report.format_table());
    }

    /// Load a real decklist corpus (`KAGGLE_DECKS/<CORPUS_FORMAT>/*.txt`), report
    /// coverage, then run the gauntlet on the fully-covered subset. Env knobs:
    /// `CORPUS_FORMAT` (default PI), `CORPUS_REFEREE` (random|vmc|pimc, default
    /// vmc), `CORPUS_DUELS` (paired duels/pair, default 5). Run:
    /// `KAGGLE_DECKS=/path CORPUS_FORMAT=PI cargo test -p arcana-ai --release corpus_gauntlet -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn corpus_gauntlet() {
        use crate::deckeval_runner::{run_gauntlet, ExperimentConfig, Referee};
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let format = std::env::var("CORPUS_FORMAT").unwrap_or_else(|_| "PI".into());
        let referee = match std::env::var("CORPUS_REFEREE").as_deref() {
            Ok("random") => Referee::Random,
            Ok("pimc") => Referee::Pimc,
            _ => Referee::VmcMaterial,
        };
        let duels: u32 = std::env::var("CORPUS_DUELS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);

        let reg = arcana_cards::build_catalog();
        let dir = std::path::Path::new(&base).join(&format);
        let loaded = decks_from_dir(&dir, &format, &reg).expect("read corpus dir");
        let report = coverage_report(&loaded, 60, 60);
        println!("\n{}", report.format_table());

        let decks = playable_decks(&loaded, 60, 60);
        println!("\nplayable {format} decks: {}", decks.len());
        if decks.len() < 2 {
            println!("not enough playable decks to run a gauntlet");
            return;
        }
        let cfg = ExperimentConfig {
            referee,
            paired_duels_per_pair: duels,
            max_steps: 4000,
            base_seed: 0,
            bootstrap_samples: 2000,
        };
        let gauntlet = run_gauntlet(&decks, &reg, &cfg);
        println!("\n{}\n", gauntlet.format_table());
        println!("{}", gauntlet.to_csv());
    }

    /// Build FIDELITY-CONTROLLED Pioneer subsets to separate referee-bias from
    /// card-fidelity-bias: fully-covered maindecks with NO GAP-approximated cards
    /// (`strict`), plus a relaxed tier that allows only the low-impact shockland
    /// "pay 2 life or tapped" payment approximation. Coverage is authoritative
    /// (`parse_deck_text().unresolved.is_empty()`); the GAP list mirrors
    /// `docs/gauntlet-results/approximated-cards.txt` with `(shocklands)` expanded
    /// to the ten exact names. Copies passing decklists to
    /// `FIDELITY_OUT/{strict,relaxed}/PI` and prints a survival report (counts +
    /// archetype buckets) BEFORE any scoring.
    /// `KAGGLE_DECKS=<dir> FIDELITY_OUT=<dir> cargo test -p arcana-ai --release build_fidelity_subset -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn build_fidelity_subset() {
        use std::collections::{BTreeMap, HashSet};
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let out = std::env::var("FIDELITY_OUT").expect("set FIDELITY_OUT");
        let reg = arcana_cards::build_catalog();

        // The low-impact "pay 2 life or enters tapped" payment approximation.
        let shocklands: HashSet<&str> = [
            "Blood Crypt", "Breeding Pool", "Godless Shrine", "Hallowed Fountain",
            "Overgrown Tomb", "Sacred Foundry", "Steam Vents", "Stomping Ground",
            "Temple Garden", "Watery Grave",
        ].into_iter().collect();
        // Every GAP-approximated card (approximated-cards.txt), shocklands expanded.
        let mut gap: HashSet<String> = [
            // NB: keep in sync with docs/gauntlet-results/approximated-cards.txt.
            // Done (fidelity grind, removed): Empyrean Eagle, Supreme Phantom,
            // Hangarback Walker, Winding Constrictor.
            "Tarmogoyf", "Steel Leaf Champion", "Soul-Scar Mage",
            "Torbran, Thane of Red Fell",
            "Smuggler's Copter", "Heart of Kiran", "Aethersphere Harvester",
            "Dig Through Time", "Wild Slash", "Fatal Push", "Boros Charm", "Censor",
            "Syncopate", "Stubborn Denial", "Once Upon a Time",
            "Castle Garenbrig", "Raging Ravine", "Creeping Tar Pit",
            "Field of the Dead", "Blast Zone",
        ].into_iter().map(String::from).collect();
        for s in &shocklands { gap.insert(s.to_string()); }

        let dir = std::path::Path::new(&base).join("PI");
        let mut files: Vec<_> = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false)).collect();
        files.sort();

        let strict_dir = std::path::Path::new(&out).join("strict").join("PI");
        let relaxed_dir = std::path::Path::new(&out).join("relaxed").join("PI");
        std::fs::create_dir_all(&strict_dir).unwrap();
        std::fs::create_dir_all(&relaxed_dir).unwrap();

        let (mut n_total, mut n_covered, mut n_strict, mut n_relaxed) = (0u32, 0u32, 0u32, 0u32);
        let mut strict_arch: BTreeMap<String, u32> = BTreeMap::new();
        let mut relaxed_arch: BTreeMap<String, u32> = BTreeMap::new();
        // How many COVERED decks each GAP card blocks (fidelity worklist by leverage).
        let mut gap_blocks: BTreeMap<String, u32> = BTreeMap::new();
        for p in &files {
            n_total += 1;
            let txt = std::fs::read_to_string(p).unwrap();
            let parsed = parse_deck_text(&txt, &reg);
            if !parsed.unresolved.is_empty() { continue; } // not fully covered
            n_covered += 1;
            let names: Vec<String> = parsed.main.iter()
                .filter_map(|(cid, _)| reg.get(*cid)
                    .and_then(|d| reg.interner().resolve(d.name)).map(String::from))
                .collect();
            for n in names.iter().collect::<HashSet<_>>() {
                if gap.contains(n) { *gap_blocks.entry(n.clone()).or_insert(0) += 1; }
            }
            let strict_ok = names.iter().all(|n| !gap.contains(n));
            let relaxed_ok = names.iter().all(|n| !gap.contains(n) || shocklands.contains(n.as_str()));
            let arch = if parsed.name.is_empty() { "?".into() } else { parsed.name.clone() };
            let fname = p.file_name().unwrap();
            if relaxed_ok {
                n_relaxed += 1;
                *relaxed_arch.entry(arch.clone()).or_insert(0) += 1;
                std::fs::copy(p, relaxed_dir.join(fname)).unwrap();
            }
            if strict_ok {
                n_strict += 1;
                *strict_arch.entry(arch.clone()).or_insert(0) += 1;
                std::fs::copy(p, strict_dir.join(fname)).unwrap();
            }
        }
        println!("PI fidelity survival: total={n_total} covered={n_covered} \
                  strict(zero-GAP)={n_strict} relaxed(+shocklands)={n_relaxed}");
        println!("\nstrict archetypes ({} distinct):", strict_arch.len());
        for (a, n) in &strict_arch { println!("  {n:>3}  {a}"); }
        println!("\nrelaxed archetypes ({} distinct):", relaxed_arch.len());
        for (a, n) in &relaxed_arch { println!("  {n:>3}  {a}"); }
        let mut blocks: Vec<(&String, &u32)> = gap_blocks.iter().collect();
        blocks.sort_by(|a, b| b.1.cmp(a.1));
        println!("\nGAP cards by # of covered decks blocked (of {n_covered}):");
        for (card, n) in blocks { println!("  {n:>3}  {card}"); }
    }
}

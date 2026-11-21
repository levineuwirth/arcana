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
    /// Stable source identifier, usually the decklist file stem.
    pub source_id: String,
    /// Human-facing deck name, usually the archetype from an `About/Name` header.
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
        let name = if self.source_id == self.name {
            self.name.clone()
        } else {
            format!("{} [{}]", self.name, self.source_id)
        };
        let mut cards = Vec::with_capacity(self.main_count() as usize);
        for (id, n) in &self.parsed.main {
            for _ in 0..*n {
                cards.push(*id);
            }
        }
        Deck { name, cards }
    }
}

/// Parse one real decklist (Arena/MTGO `N CardName` text) against the catalog.
pub fn load_deck(
    name: impl Into<String>,
    format: impl Into<String>,
    list_text: &str,
    reg: &CardRegistry,
) -> LoadedDeck {
    let source_id = name.into();
    let parsed = parse_deck_text(list_text, reg);
    // An `About`/`Name` header (e.g. the archetype) wins; the passed `name`
    // (file stem) is the fallback when the list carries no name.
    let name = if parsed.name.is_empty() {
        source_id.clone()
    } else {
        parsed.name.clone()
    };
    LoadedDeck {
        source_id,
        name,
        format: format.into(),
        parsed,
    }
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

/// Why `decks` are not exactly the field a manifest pins (A2.2's
/// `docs/a2-field/manifest-PI.csv`: `index,name,source,sha256`), or `None` when
/// they are: the same decks by index and name, in order, none extra and none
/// missing. The field shards check this before any game, so a directory with
/// one playable deck more or less refuses to run rather than shifting every
/// index. (The decklists' digests are checked by `field.lsf`.)
pub fn field_manifest_mismatch(decks: &[Deck], manifest_csv: &str) -> Option<String> {
    let mut want = Vec::new();
    for (n, line) in manifest_csv.lines().enumerate().skip(1) {
        let fields: Vec<&str> = line.split(',').collect();
        let [index, name, _source, _sha] = fields[..] else {
            return Some(format!("manifest line {}: {} fields, not 4", n + 1, fields.len()));
        };
        want.push((index.to_string(), name.to_string()));
    }
    let got: Vec<(String, String)> = decks.iter().enumerate().map(|(i, d)| (i.to_string(), d.name.clone())).collect();
    if got == want {
        return None;
    }
    let first = (0..got.len().max(want.len())).find(|&k| got.get(k) != want.get(k)).unwrap_or(0);
    Some(format!("{} playable decks against the manifest's {}; first difference at index {first}", got.len(), want.len()))
}

/// The field shards' gate: with `CORPUS_MANIFEST` set, panic before any game
/// unless `decks` are exactly the manifest's.
fn require_manifest(decks: &[Deck]) {
    if let Ok(path) = std::env::var("CORPUS_MANIFEST") {
        let text = std::fs::read_to_string(&path).expect("read CORPUS_MANIFEST");
        if let Some(why) = field_manifest_mismatch(decks, &text) {
            panic!("the playable decks are not the manifest's field, so no game is played: {why}");
        }
    }
}

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

    #[test]
    fn deck_names_keep_source_id_when_header_renames_deck() {
        let reg = arcana_cards::build_catalog();
        let loaded = load_deck(
            "24148_368536",
            "pioneer",
            "About\nName Red Deck Wins\nDeck\n10 Lightning Bolt\n50 Mountain\n",
            &reg,
        );
        assert_eq!(loaded.source_id, "24148_368536");
        assert_eq!(loaded.name, "Red Deck Wins");
        assert_eq!(loaded.to_deck().name, "Red Deck Wins [24148_368536]");
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
    /// vmc), `CORPUS_DUELS` (paired duels/pair, default 5), `CORPUS_BASE_SEED`
    /// (root seed, default 0 — vary it to get an INDEPENDENT re-measurement of the
    /// same decks under the same referee, i.e. the regression-to-the-mean control
    /// null for a referee arm). Run:
    /// `KAGGLE_DECKS=/path CORPUS_FORMAT=PI cargo test -p arcana-ai --release corpus_gauntlet -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn corpus_gauntlet() {
        use crate::deckeval_runner::{run_gauntlet, ExperimentConfig, Referee, RefereeBudgets};
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
        let base_seed: u64 = std::env::var("CORPUS_BASE_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

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
            base_seed,
            bootstrap_samples: 2000,
            budgets: RefereeBudgets::from_env(),
        };
        let gauntlet = run_gauntlet(&decks, &reg, &cfg);
        println!("\n{}\n", gauntlet.format_table());
        println!("{}", gauntlet.to_csv());
    }

    /// One shard of [`corpus_gauntlet`] for a cluster array: the duels of the
    /// pairs whose index in [`crate::deckeval_runner::gauntlet_pairs`] is `k`
    /// modulo `K` (`CORPUS_SHARD=k/K`, default `0/1`, the whole gauntlet), each
    /// played by [`crate::deckeval_runner::play_duel`] exactly as
    /// `run_gauntlet` plays it. No aggregation: it prints the playable decks
    /// and one row per duel, which merge across shards.
    ///
    ///   `config,<referee>,<budgets>,<duels>,<base seed>,<k>/<K>`
    ///   `deck,<index>,<name>`
    ///   `duel,<i>,<j>,<d>,<a_i>,<a_j>,<b_i>,<b_j>,<seconds>`
    ///
    /// where game A seats deck i first and game B swaps seats. Same variables
    /// as `corpus_gauntlet`: `CORPUS_POLICIES=capsule` gives A2.1's capsule
    /// policies instead of the field's ([`crate::deckeval_runner::RefereeBudgets`]),
    /// and `CORPUS_PIMC_SAMPLES`/`CORPUS_PIMC_CAP` set PIMC's budget;
    /// `CORPUS_DUELS` defaults to 1 here.
    /// `KAGGLE_DECKS=<dir> CORPUS_SHARD=0/8 cargo test -p arcana-ai --release corpus_gauntlet_shard -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn corpus_gauntlet_shard() {
        use crate::deckeval_runner::{gauntlet_pairs, play_duel, ExperimentConfig, Referee, RefereeBudgets};
        use std::time::Instant;
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let format = std::env::var("CORPUS_FORMAT").unwrap_or_else(|_| "PI".into());
        let referee = match std::env::var("CORPUS_REFEREE").as_deref() {
            Ok("random") => Referee::Random,
            Ok("pimc") => Referee::Pimc,
            _ => Referee::VmcMaterial,
        };
        let env = |k: &str, default: u64| -> u64 {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
        };
        let duels = env("CORPUS_DUELS", 1) as u32;
        let (k, shards) = std::env::var("CORPUS_SHARD")
            .ok()
            .map(|s| {
                let (a, b) = s.split_once('/').expect("CORPUS_SHARD is k/K");
                (a.parse::<usize>().expect("k"), b.parse::<usize>().expect("K"))
            })
            .unwrap_or((0, 1));
        assert!(k < shards, "CORPUS_SHARD {k}/{shards}: k must be below K");

        let reg = arcana_cards::build_catalog();
        let dir = std::path::Path::new(&base).join(&format);
        let loaded = decks_from_dir(&dir, &format, &reg).expect("read corpus dir");
        let decks = playable_decks(&loaded, 60, 60);
        require_manifest(&decks);
        for (i, d) in decks.iter().enumerate() {
            println!("deck,{i},{}", d.name);
        }
        let cfg = ExperimentConfig {
            referee,
            paired_duels_per_pair: duels,
            max_steps: 4000,
            base_seed: env("CORPUS_BASE_SEED", 0),
            bootstrap_samples: 0,
            budgets: RefereeBudgets::from_env(),
        };
        println!(
            "config,{:?},{:?},{duels},{},{k}/{shards}",
            cfg.referee, cfg.budgets, cfg.base_seed
        );
        for (p, &(i, j)) in gauntlet_pairs(decks.len()).iter().enumerate() {
            if p % shards != k {
                continue;
            }
            for d in 0..duels {
                let t = Instant::now();
                let r = play_duel(&decks, &reg, &cfg, i, j, d);
                println!(
                    "duel,{i},{j},{d},{},{},{},{},{:.2}",
                    r.a_i, r.a_j, r.b_i, r.b_j, t.elapsed().as_secs_f64()
                );
            }
        }
    }

    /// A1.5ci's head-to-heads as shards for a cluster array: one policy on
    /// both seats, differing only in the pool its search takes its candidates
    /// from ([`crate::search::TopRule`]: `plays`, the shipped repair; `every`,
    /// the selection before it; `filter`, handoff 14's probe filter). The five
    /// seed decks of A2.1 (playable indices 6, 9, 19, 41 and 43) against each
    /// other, both seat orders, and each arm on each side of the same shuffle:
    /// 40 games a seed, the design and seeds of handoff 14's probe
    /// (`h2h` mode), so `TOP_ARMS=every,every TOP_FLIPS=0` replays its
    /// `verify-mf-*` games, the shipped policies at `e276894a`, by trace.
    /// The games are numbered in that order and shard `k/K` plays those whose
    /// number is `k` modulo `K`. It prints, each on a line of its own:
    ///
    ///   `config,top,<policy>,<budget>,<arm A>,<arm B>,<first seed>,<seeds>,<flips>,<k>/<K>`
    ///   `deck,<index>,<name>`
    ///   `game,<seed>,<i>,<j>,<ab>,<flip>,<s0>,<s1>,<arm0>,<arm1>,<ending>,<actions>,<turns>,
    ///    <casts0>,<casts1>,<opp_turn_casts0>,<opp_turn_casts1>,<taps0>,<taps1>,
    ///    <taps_upkeep0>,<taps_upkeep1>,<taps_off_main0>,<taps_off_main1>,
    ///    <lost_ends0>,<lost_ends1>,<lost_units0>,<lost_units1>,<scored>,<trace>,<seconds>`
    ///
    /// Seat 0 plays deck `s0` under `arm0`. `ending` is `win0`, `win1`,
    /// `draw`, or a bound: `action_cap` (20,000 actions), `board_cap` (200
    /// permanents) or `wall_cap` (`TOP_GAME_SECONDS`, default 1,800). A cast
    /// on the opponent's turn is one made while the other seat is active; a
    /// mana ability off main is one activated outside a main phase of the
    /// seat's own turn and outside a choice that asks for mana; mana is lost
    /// at a step end when an action ends the step and the seat's pool holds
    /// fewer units after it. `trace` hashes every action as the probe did.
    ///
    /// `TOP_POLICY` `pimc` (default) or `vmc`; `TOP_BUDGET` samples or
    /// rollouts, step cap, candidates (default 16,160,10 for PIMC, 10,30,16
    /// for `vmc-material`); `TOP_ARMS` (default `plays,every`); `TOP_SEEDS`
    /// first,count (default 20261111,1); `TOP_FLIPS` (default `0,1`).
    /// `KAGGLE_DECKS=<dir> CORPUS_SHARD=0/40 TOP_POLICY=pimc cargo test -p arcana-ai --release top_of_search_shard -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn top_of_search_shard() {
        use crate::search::{MaterialValue, PimcPolicy, StatePolicy, TopRule, ValueMcPolicy};
        use arcana_core::actions::Action;
        use arcana_core::engine::{new_game, step, EngineYield};
        use arcana_core::legal_actions::{is_mana_activation, pending_choice_asks_mana};
        use std::hash::{Hash, Hasher};
        use std::time::Instant;
        const SEED_DECKS: [usize; 5] = [6, 9, 19, 41, 43];
        // Handoff 14's seed mix, verbatim.
        fn mix4(a: u64, b: u64, c: u64, d: u64) -> u64 {
            let mut x = a.wrapping_add(0x9E37_79B9_7F4A_7C15);
            for v in [b, c, d] {
                x ^= v.wrapping_add(0x9E37_79B9_7F4A_7C15);
                x = x.wrapping_mul(0xBF58_476D_1CE4_E5B9);
                x ^= x >> 27;
            }
            x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^= x >> 31;
            x
        }
        let arm = |s: &str| match s {
            "plays" => TopRule::Plays,
            "every" => TopRule::Every,
            "filter" => TopRule::Filter,
            _ => panic!("an arm is plays, every or filter: {s}"),
        };
        let var = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.to_string());
        let pimc = match var("TOP_POLICY", "pimc").as_str() {
            "pimc" => true,
            "vmc" => false,
            p => panic!("TOP_POLICY is pimc or vmc: {p}"),
        };
        let budget: Vec<u64> = var("TOP_BUDGET", if pimc { "16,160,10" } else { "10,30,16" })
            .split(',').map(|x| x.parse().expect("TOP_BUDGET is three numbers")).collect();
        assert_eq!(budget.len(), 3, "TOP_BUDGET is samples or rollouts, step cap, candidates");
        let arms_text = var("TOP_ARMS", "plays,every");
        let (a, b) = arms_text.split_once(',').expect("TOP_ARMS is two arms");
        let (arm_a, arm_b) = (arm(a), arm(b));
        let seeds: Vec<u64> = var("TOP_SEEDS", "20261111,1")
            .split(',').map(|x| x.parse().expect("TOP_SEEDS is first,count")).collect();
        let flips: Vec<u8> = var("TOP_FLIPS", "0,1")
            .split(',').map(|x| x.parse().expect("TOP_FLIPS lists 0 and/or 1")).collect();
        let wall: f64 = var("TOP_GAME_SECONDS", "1800").parse().expect("TOP_GAME_SECONDS");
        let (k, shards) = std::env::var("CORPUS_SHARD")
            .ok()
            .map(|s| {
                let (a, b) = s.split_once('/').expect("CORPUS_SHARD is k/K");
                (a.parse::<usize>().expect("k"), b.parse::<usize>().expect("K"))
            })
            .unwrap_or((0, 1));
        assert!(k < shards, "CORPUS_SHARD {k}/{shards}: k must be below K");

        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let reg = arcana_cards::build_catalog();
        let loaded = decks_from_dir(std::path::Path::new(&base).join("PI"), "PI", &reg)
            .expect("read corpus dir");
        let decks = playable_decks(&loaded, 60, 60);
        require_manifest(&decks);
        // A line of its own first: a single-threaded libtest prints the
        // test's name before its output, on the same line.
        println!();
        println!("config,top,{},{},{a},{b},{},{},{},{k}/{shards}",
            if pimc { "pimc" } else { "vmc" },
            budget.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("/"),
            seeds[0], seeds[1],
            flips.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("/"));
        for &d in &SEED_DECKS {
            println!("deck,{d},{}", decks[d].name);
        }
        let mut jobs = Vec::new();
        for seed in seeds[0]..seeds[0] + seeds[1] {
            for (x, &i) in SEED_DECKS.iter().enumerate() {
                for &j in &SEED_DECKS[x + 1..] {
                    for ab in ["A", "B"] {
                        for &flip in &flips {
                            jobs.push((seed, i, j, ab, flip));
                        }
                    }
                }
            }
        }
        let name = |t: TopRule| match t {
            TopRule::Plays => "plays",
            TopRule::Every => "every",
            TopRule::Filter => "filter",
        };
        for (n, &(seed, i, j, ab, flip)) in jobs.iter().enumerate() {
            if n % shards != k { continue; }
            let t0 = Instant::now();
            let engine_seed = mix4(seed, i as u64, j as u64, 0);
            let (s0, s1, policy_seed) =
                if ab == "A" { (i, j, engine_seed ^ 0xF00D) } else { (j, i, engine_seed ^ 0xBEEF) };
            let seats = vec![decks[s0].cards.clone(), decks[s1].cards.clone()];
            let tops = if flip == 0 { [arm_a, arm_b] } else { [arm_b, arm_a] };
            let bot = |seat: usize| -> Box<dyn StatePolicy> {
                let seed = policy_seed ^ if seat == 0 { 0xA } else { 0xB };
                if pimc {
                    let mut p = PimcPolicy::with_budget(seed, seats.clone(), budget[0] as u32,
                        budget[1] as u32, budget[2] as usize);
                    p.top = tops[seat];
                    Box::new(p)
                } else {
                    let mut p = ValueMcPolicy::with_budget(Box::new(MaterialValue), seed,
                        budget[0] as u32, budget[1] as u32, budget[2] as usize);
                    p.top = tops[seat];
                    Box::new(p)
                }
            };
            let mut bots = [bot(0), bot(1)];
            let (mut state, mut yld) = new_game(seats.clone(), &reg, engine_seed);
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            let (mut actions, mut scored) = (0u32, 0u32);
            let mut casts = [0u32; 2];
            let mut opp_casts = [0u32; 2];
            let mut taps = [0u32; 2];
            let mut taps_upkeep = [0u32; 2];
            let mut taps_off = [0u32; 2];
            let mut lost_ends = [0u32; 2];
            let mut lost_units = [0u32; 2];
            let ending = loop {
                let (player, legal) = match &yld {
                    EngineYield::GameOver(r) => break match r {
                        arcana_core::state::GameResult::Win(0) => "win0",
                        arcana_core::state::GameResult::Win(_) => "win1",
                        _ => "draw",
                    },
                    EngineYield::PendingDecision { player, legal_actions, .. } =>
                        (*player, legal_actions.clone()),
                };
                if actions >= 20_000 { break "action_cap"; }
                if t0.elapsed().as_secs_f64() > wall { break "wall_cap"; }
                if legal.len() > 1 { scored += 1; }
                let action = bots[player as usize].choose(&state, &reg, player, &legal);
                format!("{action:?}").hash(&mut hasher);
                let p = player as usize;
                match &action {
                    Action::CastSpell { .. } => {
                        casts[p] += 1;
                        if state.turn.active_player != player { opp_casts[p] += 1; }
                    }
                    a @ Action::ActivateAbility { .. } if is_mana_activation(&state, a, &reg) => {
                        taps[p] += 1;
                        if state.turn.step == arcana_core::turn::Step::Upkeep { taps_upkeep[p] += 1; }
                        if !pending_choice_asks_mana(&state, &reg)
                            && !(state.turn.active_player == player && state.turn.phase.is_main())
                        {
                            taps_off[p] += 1;
                        }
                    }
                    _ => {}
                }
                let at = (state.turn.turn_number, state.turn.phase, state.turn.step);
                let before: Vec<usize> = (0..2u8).map(|q| state.player(q).mana_pool.iter().count()).collect();
                let (s, y) = step(state, action, &reg);
                state = s; yld = y; actions += 1;
                if (state.turn.turn_number, state.turn.phase, state.turn.step) != at {
                    for q in 0..2usize {
                        let after = state.player(q as u8).mana_pool.iter().count();
                        if after < before[q] {
                            lost_ends[q] += 1;
                            lost_units[q] += (before[q] - after) as u32;
                        }
                    }
                }
                let size = state.objects.objects_in_zone(arcana_core::zones::Zone::Battlefield).count();
                if size > 200 { break "board_cap"; }
            };
            println!(
                "game,{seed},{i},{j},{ab},{flip},{s0},{s1},{},{},{ending},{actions},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{scored},{:016x},{:.2}",
                name(tops[0]), name(tops[1]), state.turn.turn_number,
                casts[0], casts[1], opp_casts[0], opp_casts[1], taps[0], taps[1],
                taps_upkeep[0], taps_upkeep[1], taps_off[0], taps_off[1],
                lost_ends[0], lost_ends[1], lost_units[0], lost_units[1],
                hasher.finish(), t0.elapsed().as_secs_f64());
        }
    }

    /// The field gate refuses a directory whose playable decks differ from the
    /// manifest's in membership or order, before any game; a review found
    /// that one extra playable deck kept every digest and shifted indices.
    #[test]
    fn field_manifest_gate_requires_exact_membership_and_order() {
        let deck = |n: &str| Deck { name: n.to_string(), cards: Vec::new() };
        let manifest = "index,name,source,sha256\n0,A [1],1,x\n1,B [2],2,y\n";
        assert_eq!(field_manifest_mismatch(&[deck("A [1]"), deck("B [2]")], manifest), None);
        for (decks, why) in [
            (vec![deck("A [1]"), deck("B [2]"), deck("C [3]")], "an extra deck"),
            (vec![deck("A [1]")], "a missing deck"),
            (vec![deck("B [2]"), deck("A [1]")], "another order"),
            (vec![deck("A [1]"), deck("B [9]")], "another deck"),
        ] {
            assert!(field_manifest_mismatch(&decks, manifest).is_some(), "{why} must be refused");
        }
        assert!(field_manifest_mismatch(&[deck("A [1]")], "index,name\n0,A [1]\n").is_some(),
            "a malformed manifest must be refused");
    }

    /// A2.2's arm X as shards for a cluster array: [`crate::deckeval_runner::play_block`]
    /// for the pairs whose index in [`crate::deckeval_runner::gauntlet_pairs`]
    /// is `k` modulo `K` (`CORPUS_SHARD=k/K`, default `0/1`), `vmc-material`
    /// (pilot A) against PIMC (pilot B) at the budgets [`corpus_gauntlet_shard`]
    /// reads (`CORPUS_POLICIES=capsule`, `CORPUS_PIMC_SAMPLES`, `CORPUS_PIMC_CAP`).
    /// `CORPUS_BLOCKS` blocks a pair (default 1). It prints the playable decks
    /// and one row per block, which merge across shards:
    ///
    ///   `config,block,<budgets>,<blocks>,<base seed>,<k>/<K>`
    ///   `deck,<index>,<name>`
    ///   `block,<i>,<j>,<d>,<g1>,<g2>,<g3>,<g4>,<seconds>`
    ///
    /// where `g1` to `g4` are `vmc-material`'s points in the block's four games
    /// in `BLOCK_GAMES` order.
    /// `KAGGLE_DECKS=<dir> CORPUS_SHARD=0/8 CORPUS_POLICIES=capsule CORPUS_PIMC_SAMPLES=32 CORPUS_PIMC_CAP=320 cargo test -p arcana-ai --release corpus_block_shard -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn corpus_block_shard() {
        use crate::deckeval_runner::{gauntlet_pairs, play_block, ExperimentConfig, Referee, RefereeBudgets};
        use std::time::Instant;
        let base = std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS");
        let format = std::env::var("CORPUS_FORMAT").unwrap_or_else(|_| "PI".into());
        let env = |k: &str, default: u64| -> u64 {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
        };
        let blocks = env("CORPUS_BLOCKS", 1) as u32;
        let (k, shards) = std::env::var("CORPUS_SHARD")
            .ok()
            .map(|s| {
                let (a, b) = s.split_once('/').expect("CORPUS_SHARD is k/K");
                (a.parse::<usize>().expect("k"), b.parse::<usize>().expect("K"))
            })
            .unwrap_or((0, 1));
        assert!(k < shards, "CORPUS_SHARD {k}/{shards}: k must be below K");

        let reg = arcana_cards::build_catalog();
        let dir = std::path::Path::new(&base).join(&format);
        let loaded = decks_from_dir(&dir, &format, &reg).expect("read corpus dir");
        let decks = playable_decks(&loaded, 60, 60);
        require_manifest(&decks);
        for (i, d) in decks.iter().enumerate() {
            println!("deck,{i},{}", d.name);
        }
        let cfg = ExperimentConfig {
            referee: Referee::VmcMaterial,
            paired_duels_per_pair: blocks,
            max_steps: 4000,
            base_seed: env("CORPUS_BASE_SEED", 0),
            bootstrap_samples: 0,
            budgets: RefereeBudgets::from_env(),
        };
        println!("config,block,{:?},{blocks},{},{k}/{shards}", cfg.budgets, cfg.base_seed);
        for (p, &(i, j)) in gauntlet_pairs(decks.len()).iter().enumerate() {
            if p % shards != k {
                continue;
            }
            for d in 0..blocks {
                let t = Instant::now();
                let [g1, g2, g3, g4] = play_block(&decks, &reg, &cfg, i, j, d);
                println!("block,{i},{j},{d},{g1},{g2},{g3},{g4},{:.2}", t.elapsed().as_secs_f64());
            }
        }
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
            // Hangarback Walker, Winding Constrictor, Smuggler's Copter, Fatal Push.
            "Tarmogoyf", "Steel Leaf Champion", "Soul-Scar Mage",
            "Torbran, Thane of Red Fell",
            "Heart of Kiran", "Aethersphere Harvester",
            "Dig Through Time", "Wild Slash", "Boros Charm", "Censor",
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

    /// Select a SOURCE-AUDITABLE spanning-N control subset from the fidelity-fixed
    /// relaxed field (`CONTROL_IN`, i.e. the `relaxed/PI` output of
    /// [`build_fidelity_subset`]), deterministically by source id, for the four
    /// referee-arm buckets — Gruul Aggro, UW Spirit Aggro, Golgari Scales, Sultai
    /// Control. Copies the picks to `CONTROL_OUT/PI` and prints a manifest mapping
    /// each chosen source file stem to its bucket + archetype.
    ///
    /// This closes the provenance gap the historical 8-deck subset had: the loader
    /// preserves the file stem as [`LoadedDeck::source_id`] and the gauntlet now
    /// appends `[source_id]` to the deck name (see `subset-provenance-PI.txt`), so
    /// a gauntlet run over `CONTROL_OUT` emits source-auditable CSV rows. Its
    /// intended use is the regression-to-the-mean CONTROL for a referee arm: run
    /// [`corpus_gauntlet`] over `CONTROL_OUT` under VMC at two `CORPUS_BASE_SEED`
    /// values to get the same-referee re-measurement null, then compare that null
    /// band to the VMC->PIMC bucket movement.
    ///
    /// Bucket substrings mirror the gauntlet side of `placement_vs_gauntlet.py`;
    /// the FIRST matching bucket in priority order wins (independent of fullness),
    /// so a deck never spills into a secondary bucket. Run AFTER
    /// `build_fidelity_subset`:
    /// `CONTROL_IN=<fidelity_out>/relaxed/PI CONTROL_OUT=<dir> cargo test -p arcana-ai --release build_control_subset -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn build_control_subset() {
        let in_dir = std::env::var("CONTROL_IN")
            .expect("set CONTROL_IN to the relaxed fidelity field (…/relaxed/PI)");
        let out = std::env::var("CONTROL_OUT").expect("set CONTROL_OUT");
        let per_bucket: usize = std::env::var("CONTROL_PER_BUCKET")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2);
        let reg = arcana_cards::build_catalog();

        // (bucket label, archetype-name substrings). First match by priority wins.
        let buckets: [(&str, &[&str]); 4] = [
            ("Gruul Aggro", &["gruul aggro", "rg aggro"]),
            ("UW Spirit Aggro", &["uw spirit aggro", "spirit", "spirits"]),
            (
                "Golgari Scales",
                &["golgari aggro", "gb hardened", "golgari scales", "hardened", "harden ", "scales", "snakes"],
            ),
            ("Sultai Control", &["sultai control"]),
        ];

        let mut files: Vec<_> = std::fs::read_dir(&in_dir)
            .expect("read CONTROL_IN")
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map(|x| x == "txt").unwrap_or(false))
            .collect();
        files.sort(); // deterministic by source-id file stem

        let out_dir = std::path::Path::new(&out).join("PI");
        std::fs::create_dir_all(&out_dir).unwrap();

        // (source_id, archetype) picks per bucket.
        let mut picked: Vec<Vec<(String, String)>> = vec![Vec::new(); buckets.len()];
        for p in &files {
            let stem = p
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("deck")
                .to_string();
            let txt = std::fs::read_to_string(p).unwrap();
            let loaded = load_deck(stem, "PI", &txt, &reg);
            // The relaxed field is already covered, but re-check so the control
            // set is self-validating and gauntlet-playable (60-card maindeck).
            if !loaded.is_playable(60, 60) {
                continue;
            }
            let arch = loaded.name.to_lowercase();
            // First matching bucket by priority, independent of fullness.
            let Some(bi) = buckets
                .iter()
                .position(|(_, subs)| subs.iter().any(|s| arch.contains(s)))
            else {
                continue;
            };
            if picked[bi].len() >= per_bucket {
                continue;
            }
            picked[bi].push((loaded.source_id.clone(), loaded.name.clone()));
            std::fs::copy(p, out_dir.join(p.file_name().unwrap())).unwrap();
        }

        println!("control subset manifest (CONTROL_OUT={out}, per_bucket={per_bucket}):");
        println!("  {:<16}  {:<16}  archetype", "bucket", "source_id");
        let mut total = 0;
        for (bi, (label, _)) in buckets.iter().enumerate() {
            for (sid, arch) in &picked[bi] {
                println!("  {label:<16}  {sid:<16}  {arch}");
                total += 1;
            }
            if picked[bi].len() < per_bucket {
                println!(
                    "  !! {label}: only {} of {per_bucket} found in CONTROL_IN",
                    picked[bi].len()
                );
            }
        }
        println!("total control decks: {total} (target {})", per_bucket * buckets.len());
    }

    /// Payment list-size probe for the play-with-payment design: how long the
    /// legal list would be if every cast or activation carried the mana
    /// abilities that pay for it. Outcome-blind, at `pair_game` seeds no arm
    /// uses (default 9301 on). `vmc-material` 6/25/10 plays both seats of
    /// field pairs (2p, 2p + 1); at every `PROBE_EVERY`-th priority decision a
    /// wrapper measures the decider's state before delegating, and changes
    /// nothing. The game's result is discarded unread.
    ///
    /// For each variant A1.5ci's planner finds (planned or unplanned), the
    /// decider's legal mana activations are grouped by source, and sources
    /// into classes: same card, controller, status, counters, attachment,
    /// damage, computed characteristics and offered mana abilities, and no
    /// mention of the source's id anywhere in the stack, continuous,
    /// replacement or delayed effects, pending triggers or the variant
    /// itself (any number equal to the id counts as a mention, which only
    /// splits classes further). Candidate plans are count vectors over
    /// (class, ability); each is checked by the engine: activate a
    /// representative on a clone through `step`, as `replay` does, and look
    /// for the variant in the resulting legal list. A plan counts if it is
    /// legal and no plan one unit smaller is (local minimality). Its flat
    /// count is the multinomial over class members; a second, different
    /// representative is checked once per state to test the merge. Rows:
    ///
    ///   `state,<game>,<seat>,<turn>,<battlefield>,<sources>,<classes>,<legal>,<mana acts>,<plays>,<unplanned>,<variants>,<flat>,<collapsed>,<max flat>,<max collapsed>,<checks>,<capped>,<pool mult max>,<merge checks>,<merge fails>,<secs>`
    ///   `variant,<game>,<n>,<kind>,<planner taps>,<classes>,<flat>,<collapsed>,<checks>,<capped>`
    ///
    /// `KAGGLE_DECKS=<dir> CORPUS_MANIFEST=docs/a2-field/manifest-PI.csv cargo test -p arcana-ai --release payment_list_probe -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn payment_list_probe() {
        use crate::plays::{plays, variant_key};
        use crate::search::{pair_game, MaterialValue, StatePolicy, ValueMcPolicy};
        use arcana_core::actions::Action;
        use arcana_core::engine::{step, EngineYield};
        use arcana_core::legal_actions::is_mana_activation;
        use arcana_core::objects::ObjectId;
        use arcana_core::registry::CardRegistry;
        use arcana_core::state::GameState;
        use arcana_core::types::PlayerId;
        use arcana_core::zones::Zone;
        use std::cell::Cell;
        use std::collections::{BTreeMap, HashSet};
        use std::hash::{Hash, Hasher};
        use std::rc::Rc;
        use std::time::Instant;

        let env = |k: &str, d: u64| -> u64 {
            std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d)
        };
        let first = env("PROBE_FIRST", 9301);
        let games = env("PROBE_GAMES", 32);
        let every = env("PROBE_EVERY", 3).max(1);
        let per_game = env("PROBE_PER_GAME", 40);
        let cap = env("PROBE_CAP", 3000);
        assert!(![1000u64, 2000, 3000, 4000].contains(&first), "a registered seed");

        let reg = arcana_cards::build_catalog();
        let format = std::env::var("CORPUS_FORMAT").unwrap_or_else(|_| "PI".into());
        let dir = std::path::Path::new(&std::env::var("KAGGLE_DECKS").expect("set KAGGLE_DECKS")).join(&format);
        let loaded = decks_from_dir(&dir, &format, &reg).expect("read corpus dir");
        let decks = playable_decks(&loaded, 60, 60);
        require_manifest(&decks);
        println!("config,payment-probe,{first},{games},{every},{per_game},{cap},{}", decks.len());

        fn hash_of<T: Hash>(t: &T) -> u64 {
            let mut h = std::collections::hash_map::DefaultHasher::new();
            t.hash(&mut h);
            h.finish()
        }
        /// Every run of digits in `s`, as numbers: a conservative mention test.
        fn numbers(s: &str, out: &mut HashSet<u64>) {
            let mut cur: Option<u64> = None;
            for ch in s.chars() {
                if let Some(d) = ch.to_digit(10) {
                    cur = Some(cur.unwrap_or(0).saturating_mul(10).saturating_add(d as u64));
                } else if let Some(n) = cur.take() {
                    out.insert(n);
                }
            }
            if let Some(n) = cur { out.insert(n); }
        }
        fn multinomial(size: u64, counts: &[u64]) -> f64 {
            // size! / (c1! … ck! (size − Σc)!)
            let mut r = 1.0f64;
            let mut left = size;
            for &c in counts {
                for t in 0..c { r *= (left - t) as f64 / (t + 1) as f64; }
                left -= c;
            }
            r
        }
        /// The decider's state after activating `acts` in order, each from the
        /// legal list of the moment with the decider deciding again; `None` if
        /// any step fails that.
        fn activate(state: &GameState, reg: &CardRegistry, decider: PlayerId, acts: &[Action])
            -> Option<Vec<Action>> {
            let mut sim = state.clone();
            let mut now = arcana_core::legal_actions::legal_actions(&sim, reg);
            for a in acts {
                if !now.contains(a) { return None; }
                let (s, y) = step(sim, a.clone(), reg);
                sim = s;
                match y {
                    EngineYield::PendingDecision { player, legal_actions, .. } if player == decider =>
                        now = legal_actions,
                    _ => return None,
                }
            }
            Some(now)
        }

        struct Class { members: Vec<ObjectId>, options: Vec<(String, Vec<Action>)> }
        struct Ctx {
            game: u64,
            every: u64,
            per_game: u64,
            cap: u64,
            seen: Rc<Cell<u64>>,
            taken: Rc<Cell<u64>>,
        }

        fn measure(c: &Ctx, state: &GameState, reg: &CardRegistry, decider: PlayerId, legal: &[Action]) {
            let t = Instant::now();
            let mana: Vec<&Action> = legal.iter().filter(|a| is_mana_activation(state, a, reg)).collect();
            let ps = plays(state, reg, decider, legal);
            // Variants and the planner's tap counts (unplanned: none known).
            let mut variants: Vec<(Action, Option<usize>)> = ps.plays.iter()
                .filter_map(|p| p.last().map(|a| (variant_key(a), Some(p.len() - 1))))
                .collect();
            variants.extend(ps.unplanned.iter().map(|a| (variant_key(a), None)));
            // Sources and the mana activations each offers, keyed by shape.
            let mut by_source: BTreeMap<ObjectId, Vec<(String, Action)>> = BTreeMap::new();
            for a in &mana {
                if let Action::ActivateAbility { source, ability_index, targets, mana_payment, additional_costs } = a {
                    let shape = format!("{ability_index}|{targets:?}|{mana_payment:?}|{additional_costs:?}");
                    by_source.entry(*source).or_default().push((shape, (*a).clone()));
                }
            }
            let mut global = HashSet::new();
            numbers(&format!("{:?}{:?}{:?}{:?}{:?}", state.stack, state.continuous_effects,
                state.replacement_effects, state.delayed_triggers, state.pending_trigger_queue), &mut global);
            let key_of = |id: ObjectId, opts: &[(String, Action)]| -> u64 {
                let o = state.objects.get(id);
                let chars = state.compute_characteristics(id);
                let shapes: Vec<&String> = opts.iter().map(|(s, _)| s).collect();
                hash_of(&format!("{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{shapes:?}",
                    o.map(|o| o.card_id), o.map(|o| o.controller), o.map(|o| &o.status),
                    o.map(|o| &o.counters), o.map(|o| &o.attachments), o.map(|o| o.attached_to),
                    o.map(|o| o.damage_marked), o.map(|o| o.is_token), chars))
            };

            let (mut flat_total, mut coll_total, mut max_flat, mut max_coll) = (0f64, 0f64, 0f64, 0f64);
            let (mut checks_total, mut capped, mut pool_mult, mut merge_checks, mut merge_fails) = (0u64, 0u64, 0usize, 0u64, 0u64);
            let mut classes_seen = 0usize;
            for (vn, (v, planner)) in variants.iter().enumerate() {
                let mut mentions = global.clone();
                numbers(&format!("{v:?}"), &mut mentions);
                // Classes for this variant.
                let mut classes: Vec<Class> = Vec::new();
                let mut index: BTreeMap<u64, usize> = BTreeMap::new();
                for (id, opts) in &by_source {
                    let k = if mentions.contains(&(*id as u64)) { hash_of(&("solo", *id)) } else { key_of(*id, opts) };
                    let ci = *index.entry(k).or_insert_with(|| {
                        classes.push(Class { members: Vec::new(), options: opts.iter().map(|(s, _)| (s.clone(), Vec::new())).collect() });
                        classes.len() - 1
                    });
                    classes[ci].members.push(*id);
                    for (s, a) in opts {
                        if let Some(slot) = classes[ci].options.iter_mut().find(|(t, _)| t == s) { slot.1.push(a.clone()); }
                    }
                }
                classes_seen = classes_seen.max(classes.len());
                let sources = by_source.len();
                let kmax = planner.map_or(sources.min(8), |k| (k + 1).min(sources));
                // Count vectors over (class, option), by total size.
                let units: Vec<(usize, usize)> = classes.iter().enumerate()
                    .flat_map(|(ci, cl)| (0..cl.options.len()).map(move |oi| (ci, oi))).collect();
                let mut legal_plans: HashSet<Vec<u64>> = HashSet::new();
                let (mut flat, mut coll, mut checks, mut hit) = (0f64, 0f64, 0u64, false);
                let mut frontier: Vec<Vec<u64>> = vec![vec![0; units.len()]];
                for size in 1..=kmax {
                    // Extend only plans that are not legal: a superset of a
                    // legal plan is never minimal (more mana, by assumption,
                    // never makes a play illegal).
                    let mut next: HashSet<Vec<u64>> = HashSet::new();
                    for base in frontier.iter().filter(|b| !legal_plans.contains(*b)) {
                        if next.len() as u64 > 8 * c.cap { hit = true; break; }
                        let start = base.iter().rposition(|&x| x > 0).unwrap_or(0);
                        for u in start..units.len() {
                            let (ci, _) = units[u];
                            let used: u64 = units.iter().enumerate().filter(|(_, (cj, _))| *cj == ci).map(|(w, _)| base[w]).sum();
                            if used as usize >= classes[ci].members.len() { continue; }
                            let mut p = base.clone();
                            p[u] += 1;
                            next.insert(p);
                        }
                    }
                    let mut next: Vec<Vec<u64>> = next.into_iter().collect();
                    next.sort();
                    for p in &next {
                        // Skip plans whose one-smaller sub-plan is already legal.
                        let dominated = (0..units.len()).any(|w| p[w] > 0 && {
                            let mut q = p.clone(); q[w] -= 1; legal_plans.contains(&q)
                        });
                        if dominated { legal_plans.insert(p.clone()); continue; }
                        if checks >= c.cap { hit = true; break; }
                        // Representative: each class's members in order, options in order.
                        let mut acts = Vec::new();
                        for (ci, cl) in classes.iter().enumerate() {
                            let mut m = 0usize;
                            for (oi, (_, by_member)) in cl.options.iter().enumerate() {
                                let w = units.iter().position(|&x| x == (ci, oi)).unwrap();
                                for _ in 0..p[w] {
                                    let id = cl.members[m];
                                    m += 1;
                                    if let Some(a) = by_member.iter().find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == id)) {
                                        acts.push(a.clone());
                                    }
                                }
                            }
                        }
                        checks += 1;
                        let after = activate(state, reg, decider, &acts);
                        let hits = after.as_ref().map_or(0, |l| l.iter().filter(|a| variant_key(a) == *v).count());
                        if hits == 0 { continue; }
                        legal_plans.insert(p.clone());
                        pool_mult = pool_mult.max(hits);
                        coll += 1.0;
                        let mut f = 1.0;
                        for (ci, cl) in classes.iter().enumerate() {
                            let counts: Vec<u64> = (0..cl.options.len())
                                .map(|oi| p[units.iter().position(|&x| x == (ci, oi)).unwrap()]).collect();
                            f *= multinomial(cl.members.len() as u64, &counts);
                        }
                        flat += f;
                        // Once per state: the same plan through the last members.
                        if merge_checks == 0 && classes.iter().any(|cl| cl.members.len() > 1) {
                            let mut alt = Vec::new();
                            for (ci, cl) in classes.iter().enumerate() {
                                let mut m = cl.members.len();
                                for (oi, (_, by_member)) in cl.options.iter().enumerate() {
                                    let w = units.iter().position(|&x| x == (ci, oi)).unwrap();
                                    for _ in 0..p[w] {
                                        m -= 1;
                                        let id = cl.members[m];
                                        if let Some(a) = by_member.iter().find(|a| matches!(a, Action::ActivateAbility { source, .. } if *source == id)) {
                                            alt.push(a.clone());
                                        }
                                    }
                                }
                            }
                            if alt != acts {
                                merge_checks += 1;
                                let ok = activate(state, reg, decider, &alt)
                                    .is_some_and(|l| l.iter().filter(|a| variant_key(a) == *v).count() == hits);
                                if !ok { merge_fails += 1; }
                            }
                        }
                    }
                    if hit { break; }
                    frontier = next;
                }
                checks_total += checks;
                if hit { capped += 1; }
                flat_total += flat;
                coll_total += coll;
                max_flat = max_flat.max(flat);
                max_coll = max_coll.max(coll);
                let kind = match v { Action::CastSpell { .. } => "cast", Action::ActivateAbility { .. } => "activate", _ => "other" };
                println!("variant,{},{vn},{kind},{},{},{flat},{coll},{checks},{}",
                    c.game, planner.map_or(-1, |k| k as i64), classes.len(), hit as u8);
            }
            let bf = state.objects.objects_in_zone(Zone::Battlefield).count();
            println!("state,{},{decider},{},{bf},{},{classes_seen},{},{},{},{},{},{flat_total},{coll_total},{max_flat},{max_coll},{checks_total},{capped},{pool_mult},{merge_checks},{merge_fails},{:.3}",
                c.game, state.turn.turn_number, by_source.len(), legal.len(), mana.len(),
                ps.plays.len(), ps.unplanned.len(), variants.len(), t.elapsed().as_secs_f64());
        }

        struct Probe { inner: Box<dyn StatePolicy>, ctx: Rc<Ctx> }
        impl StatePolicy for Probe {
            fn choose(&mut self, state: &GameState, registry: &CardRegistry,
                      decider: PlayerId, legal: &[Action]) -> Action {
                let c = &self.ctx;
                let at_priority = state.pending_choice.is_none() && legal.contains(&Action::PassPriority);
                if at_priority {
                    let n = c.seen.get();
                    c.seen.set(n + 1);
                    if n % c.every == 0 && c.taken.get() < c.per_game {
                        c.taken.set(c.taken.get() + 1);
                        measure(c, state, registry, decider, legal);
                    }
                }
                self.inner.choose(state, registry, decider, legal)
            }
        }

        for p in 0..games {
            let (i, j) = ((2 * p as usize) % decks.len(), (2 * p as usize + 1) % decks.len());
            let g = first + p;
            let ctx = Rc::new(Ctx { game: g, every, per_game, cap,
                seen: Rc::new(Cell::new(0)), taken: Rc::new(Cell::new(0)) });
            let mk = |s: u64| -> Box<dyn StatePolicy> {
                Box::new(Probe {
                    inner: Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 6, 25, 10)),
                    ctx: Rc::clone(&ctx),
                })
            };
            let t = Instant::now();
            let _ = pair_game(&decks[i].cards, &decks[j].cards, &reg, g, 4000, &mk, &mk);
            println!("game,{g},{i},{j},{},{:.2}", ctx.taken.get(), t.elapsed().as_secs_f64());
        }
    }
}

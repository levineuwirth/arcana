//! Controlled RL benchmark — the internally-valid successor to the
//! deckbuilding-as-evaluation arc.
//!
//! # Why this exists
//!
//! The deckbuild-eval arc tried to make deck-strength rankings track the *real
//! MTGTop8 metagame*; a skeptical review + a same-referee control
//! (`docs/gauntlet-results/control-arm-PI.txt`) showed that axis is blocked by a
//! top-8-**censored** target, N=4 buckets, and a narrow field — none cheap to
//! fix. This module drops "match reality" and measures something we *can* fully
//! control: **how strong is a policy on a fixed, source-auditable deck capsule**,
//! and **can we distill an expensive search (PIMC) into a cheap value leaf**.
//!
//! # The benchmark
//!
//! - **Domain:** the fixed Pioneer capsule (`docs/capsule-pioneer/seeds/`), five
//!   real archetype decks whose file stems are Kaggle source ids (auditable).
//! - **Metric:** policy strength = point-rate (win = 1, draw = ½) in a
//!   **mirror** round-robin — for each capsule deck, every contestant plays that
//!   same deck, so the only thing that varies is the *policy*. Averaging across
//!   the five archetypes gives a deck-robust policy score.
//! - **Held-out opponent discipline:** a learned leaf is trained on self-play
//!   data from a *teacher* policy; its strength is read primarily **vs a
//!   different, stronger opponent** (PIMC) it was not trained against, so we
//!   don't score a policy on the very distribution it was fit to (the Goodhart
//!   trap the capsule A/B already exposed). [`PolicyScore::held_out_vs`] carries
//!   which opponent is a given leaf's teacher.
//!
//! The first experiment ([`tests::capsule_pimc_distill`]) A/Bs a ValueMc leaf
//! learned from **PIMC** self-play against one learned from **random** self-play
//! (the parked pipeline's only data source) and the hand-tuned **material** leaf
//! — testing the pivot's core hypothesis that on-distribution / strong-teacher
//! data yields a better cheap leaf.

use arcana_core::registry::CardRegistry;
use arcana_core::types::CardId;

use crate::deckeval::Deck;
use crate::search::{round_robin, RoundRobin, StatePolicy};

/// One contestant's aggregated result across the capsule.
#[derive(Clone, Debug)]
pub struct PolicyScore {
    pub name: String,
    /// Points (win 1, draw ½) summed over all opponents and all capsule decks.
    pub points: f32,
    /// Games played across all opponents and all capsule decks.
    pub games: u32,
    /// Mean point-rate in `[0, 1]` = `points / games`.
    pub point_rate: f32,
    /// Point-rate against each other contestant by name (deck-aggregated).
    pub vs: Vec<(String, f32)>,
}

impl PolicyScore {
    /// Point-rate against one named opponent (the held-out readout when
    /// `opponent` is a strong policy this one was not trained against).
    pub fn held_out_vs(&self, opponent: &str) -> Option<f32> {
        self.vs.iter().find(|(n, _)| n == opponent).map(|(_, r)| *r)
    }
}

/// Capsule-benchmark result: one [`PolicyScore`] per contestant, plus the decks
/// the yardstick ran on (source-auditable names) for provenance.
pub struct CapsuleYardstick {
    pub deck_names: Vec<String>,
    pub scores: Vec<PolicyScore>,
}

impl CapsuleYardstick {
    /// Contestants sorted by point-rate descending.
    pub fn ranking(&self) -> Vec<&PolicyScore> {
        let mut v: Vec<&PolicyScore> = self.scores.iter().collect();
        v.sort_by(|a, b| b.point_rate.partial_cmp(&a.point_rate).unwrap_or(std::cmp::Ordering::Equal));
        v
    }

    /// Human-readable table: point-rate, games, and per-opponent point-rates.
    pub fn format_table(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "capsule yardstick — {} decks: {}\n",
            self.deck_names.len(),
            self.deck_names.join(", ")
        ));
        let w = self.scores.iter().map(|p| p.name.len()).max().unwrap_or(8).max(8);
        for p in self.ranking() {
            s.push_str(&format!(
                "  {:<w$}  pr={:>6.3}  ({} games)   ",
                p.name, p.point_rate, p.games, w = w
            ));
            let vs: Vec<String> = p.vs.iter().map(|(n, r)| format!("vs {n}={r:.2}")).collect();
            s.push_str(&vs.join("  "));
            s.push('\n');
        }
        s
    }
}

/// Aggregate one deck's [`RoundRobin`] into per-contestant (points, games) and
/// per-opponent (points, games) tallies, indexed by contestant position.
fn accumulate(
    rr: &RoundRobin,
    points: &mut [f32],
    games: &mut [u32],
    vs_points: &mut [Vec<f32>],
    vs_games: &mut [Vec<u32>],
) {
    let n = rr.names.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let w = rr.wins[i][j] as f32;
            let d = rr.draws[i][j] as f32;
            let g = rr.wins[i][j] + rr.wins[j][i] + rr.draws[i][j];
            points[i] += w + 0.5 * d;
            games[i] += g;
            vs_points[i][j] += w + 0.5 * d;
            vs_games[i][j] += g;
        }
    }
}

/// Run the capsule-mirror round-robin: for each deck in `decks`, every
/// contestant plays that same deck (a mirror), then aggregate point-rates across
/// decks. `contestants` are `(name, policy-maker)` where the maker may close over
/// the *current deck* — so callers rebuild PIMC/leaf makers per deck (PIMC needs
/// the deck for determinization). See [`tests::capsule_pimc_distill`].
pub fn run_capsule_yardstick(
    decks: &[Deck],
    registry: &CardRegistry,
    make_contestants: &dyn Fn(&[CardId]) -> Vec<(String, Box<dyn Fn(u64) -> Box<dyn StatePolicy>>)>,
    games_per_pair: u32,
    max_steps: u32,
) -> CapsuleYardstick {
    assert!(!decks.is_empty(), "need at least one capsule deck");

    // `make_contestants` is called EXACTLY once per deck (it may train learned
    // leaves — expensive). The roster (names/order) must be stable across decks;
    // the makers rebuild per deck because PIMC determinization needs the deck.
    let mut names: Vec<String> = Vec::new();
    let mut points: Vec<f32> = Vec::new();
    let mut games: Vec<u32> = Vec::new();
    let mut vs_points: Vec<Vec<f32>> = Vec::new();
    let mut vs_games: Vec<Vec<u32>> = Vec::new();

    for (di, deck) in decks.iter().enumerate() {
        let owned = make_contestants(&deck.cards);
        let contestants: Vec<(&str, &dyn Fn(u64) -> Box<dyn StatePolicy>)> =
            owned.iter().map(|(nm, mk)| (nm.as_str(), mk.as_ref())).collect();
        let rr = round_robin(&contestants, &deck.cards, registry, games_per_pair, max_steps);
        // Per-deck progress (long PIMC runs: preserves partial signal on a kill).
        let per_g = games_per_pair.max(1) * (rr.names.len().max(1) as u32 - 1);
        let line: Vec<String> = (0..rr.names.len())
            .map(|i| {
                let pts: f32 = (0..rr.names.len())
                    .map(|j| rr.wins[i][j] as f32 + 0.5 * rr.draws[i][j] as f32)
                    .sum();
                format!("{}={:.2}", rr.names[i], pts / per_g as f32)
            })
            .collect();
        println!("  [deck {}/{}] {}: {}", di + 1, decks.len(), deck.name, line.join(" "));

        if names.is_empty() {
            let n = rr.names.len();
            names = rr.names.clone();
            points = vec![0.0; n];
            games = vec![0; n];
            vs_points = vec![vec![0.0; n]; n];
            vs_games = vec![vec![0; n]; n];
        } else {
            debug_assert_eq!(rr.names, names, "contestant roster must be stable across decks");
        }
        accumulate(&rr, &mut points, &mut games, &mut vs_points, &mut vs_games);
    }

    let n = names.len();
    let scores = (0..n)
        .map(|i| {
            let vs = (0..n)
                .filter(|&j| j != i && vs_games[i][j] > 0)
                .map(|j| (names[j].clone(), vs_points[i][j] / vs_games[i][j] as f32))
                .collect();
            PolicyScore {
                name: names[i].clone(),
                points: points[i],
                games: games[i],
                point_rate: if games[i] == 0 { 0.0 } else { points[i] / games[i] as f32 },
                vs,
            }
        })
        .collect();

    CapsuleYardstick { deck_names: decks.iter().map(|d| d.name.clone()).collect(), scores }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck_corpus::decks_from_dir;
    use crate::deckeval::Deck;
    use crate::learn::learn_value;
    use crate::search::{
        MaterialValue, PimcPolicy, RandomStatePolicy, StatePolicy, ValueMcPolicy,
    };

    /// Load the source-auditable capsule decks from a seeds dir (each `*.txt` a
    /// Kaggle-source-id decklist). Only fully-covered 60-card lists are kept.
    fn load_capsule(dir: &str, reg: &CardRegistry) -> Vec<Deck> {
        decks_from_dir(dir, "PI", reg)
            .expect("read capsule seeds dir")
            .into_iter()
            .filter(|d| d.is_playable(60, 60))
            .map(|d| d.to_deck())
            .collect()
    }

    /// FAST non-ignored smoke: the capsule yardstick aggregation is well-formed
    /// on two sample-deck mirrors under cheap policies (no external files, no
    /// PIMC). Guards the harness against bit-rot.
    #[test]
    fn capsule_yardstick_is_well_formed() {
        let reg = arcana_cards::build_catalog();
        let decks = vec![
            Deck { name: "mirrorA".into(), cards: arcana_cards::sample_deck(&reg, 7) },
            Deck { name: "mirrorB".into(), cards: arcana_cards::sample_deck(&reg, 3) },
        ];
        let make = |_deck: &[CardId]| -> Vec<(String, Box<dyn Fn(u64) -> Box<dyn StatePolicy>>)> {
            vec![
                (
                    "random".into(),
                    Box::new(|s: u64| Box::new(RandomStatePolicy::new(s)) as Box<dyn StatePolicy>)
                        as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
                (
                    "vmc-material".into(),
                    Box::new(|s: u64| {
                        Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 4, 20, 8))
                            as Box<dyn StatePolicy>
                    }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
            ]
        };
        let y = run_capsule_yardstick(&decks, &reg, &make, 2, 4000);
        assert_eq!(y.scores.len(), 2);
        for p in &y.scores {
            assert!((0.0..=1.0).contains(&p.point_rate), "pr out of range: {}", p.point_rate);
            assert!(p.games > 0, "no games for {}", p.name);
            // one opponent
            assert_eq!(p.vs.len(), 1, "{} should face 1 opponent", p.name);
        }
        assert!(y.format_table().contains("capsule yardstick"));
    }

    /// EXPERIMENT 1 — distill PIMC's distribution into a cheap ValueMc leaf.
    /// A/Bs, on the capsule, ValueMc leaves learned from PIMC self-play vs random
    /// self-play vs hand-tuned material, with random + PIMC as reference
    /// opponents. Held-out read: `vmc-learned-rand`'s point-rate vs PIMC (it was
    /// NOT trained on PIMC), and both learned leaves vs material.
    /// Env: CAPSULE_DIR (seeds dir), CAP_GAMES (per pair, default 8), CAP_TRAIN
    /// (self-play games per leaf per deck, default 20), CAP_NDECKS (default all),
    /// CAP_PIMC_SAMPLES/CAP (PIMC budget, default 12/120). Run in release:
    ///   CAPSULE_DIR=$(pwd)/docs/capsule-pioneer/seeds \
    ///   cargo test -p arcana-ai --release benchmark::tests::capsule_pimc_distill -- --ignored --nocapture
    #[test]
    #[ignore]
    fn capsule_pimc_distill() {
        let reg = arcana_cards::build_catalog();
        let dir = std::env::var("CAPSULE_DIR")
            .expect("set CAPSULE_DIR to docs/capsule-pioneer/seeds");
        let games: u32 = envu("CAP_GAMES", 8);
        let train: u32 = envu("CAP_TRAIN", 20);
        let pimc_samples: u32 = envu("CAP_PIMC_SAMPLES", 12);
        let pimc_cap: u32 = envu("CAP_PIMC_CAP", 120);
        let ndecks: usize = envu("CAP_NDECKS", 999) as usize;
        let max_steps = 4000u32;

        let mut decks = load_capsule(&dir, &reg);
        // CAP_DECK selects ONE deck by index (for running the capsule as
        // independent parallel per-deck jobs — faster + kill-robust); else keep
        // the first CAP_NDECKS.
        if let Ok(idx) = std::env::var("CAP_DECK") {
            if let Ok(i) = idx.parse::<usize>() {
                assert!(i < decks.len(), "CAP_DECK {i} out of range ({} decks)", decks.len());
                decks = vec![decks[i].clone()];
            }
        } else {
            decks.truncate(ndecks.max(1));
        }
        println!(
            "capsule: {} decks — {}",
            decks.len(),
            decks.iter().map(|d| d.name.clone()).collect::<Vec<_>>().join(", ")
        );

        // Per capsule deck, build the contestant panel. The two learned leaves are
        // trained HERE (per deck) so each is on-distribution for its own mirror.
        // Capture the registry by shared reference (Copy) so `reg` itself is not
        // moved into the closure — the yardstick call below still borrows it.
        let reg_ref: &CardRegistry = &reg;
        let make = move |deck: &[CardId]| -> Vec<(String, Box<dyn Fn(u64) -> Box<dyn StatePolicy>>)> {
            let d = deck.to_vec();

            // PIMC maker for this deck (determinization needs the deck).
            let dp = d.clone();
            let pimc_mk = move |s: u64| -> Box<dyn StatePolicy> {
                Box::new(PimcPolicy::with_budget(
                    s, vec![dp.clone(), dp.clone()], pimc_samples, pimc_cap, 10,
                ))
            };

            // Leaf A: value learned from RANDOM self-play (the parked baseline).
            let lv_rand = learn_value(
                &d, reg_ref, train, max_steps,
                &|s| Box::new(RandomStatePolicy::new(s)),
                300, 0.3, 1e-4, 1,
            );
            // Leaf B: value learned from PIMC self-play (the pivot hypothesis).
            let dpp = d.clone();
            let lv_pimc = learn_value(
                &d, reg_ref, train, max_steps,
                &move |s| Box::new(PimcPolicy::with_budget(
                    s, vec![dpp.clone(), dpp.clone()], pimc_samples, pimc_cap, 10)),
                300, 0.3, 1e-4, 1,
            );

            let lv_rand_c = lv_rand.clone();
            let lv_pimc_c = lv_pimc.clone();
            let dp2 = d.clone();
            vec![
                (
                    "random".into(),
                    Box::new(|s: u64| Box::new(RandomStatePolicy::new(s)) as Box<dyn StatePolicy>)
                        as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
                (
                    "vmc-material".into(),
                    Box::new(|s: u64| {
                        Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 6, 25, 10))
                            as Box<dyn StatePolicy>
                    }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
                (
                    "vmc-learned-rand".into(),
                    Box::new(move |s: u64| {
                        Box::new(ValueMcPolicy::with_budget(Box::new(lv_rand_c.clone()), s, 6, 25, 10))
                            as Box<dyn StatePolicy>
                    }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
                (
                    "vmc-learned-pimc".into(),
                    Box::new(move |s: u64| {
                        Box::new(ValueMcPolicy::with_budget(Box::new(lv_pimc_c.clone()), s, 6, 25, 10))
                            as Box<dyn StatePolicy>
                    }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
                (
                    "pimc".into(),
                    Box::new(move |s: u64| pimc_mk(s)) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>,
                ),
            ]
        };

        let y = run_capsule_yardstick(&decks, &reg, &make, games, max_steps);
        println!("\n{}", y.format_table());
        println!("\n# HELD-OUT READ (leaf vs an opponent it was NOT trained against):");
        for name in ["vmc-learned-rand", "vmc-learned-pimc"] {
            if let Some(p) = y.scores.iter().find(|p| p.name == name) {
                let vp = p.held_out_vs("pimc").unwrap_or(f32::NAN);
                let vm = p.held_out_vs("vmc-material").unwrap_or(f32::NAN);
                let teacher = if name.ends_with("pimc") { "pimc (NOT held-out)" } else { "random" };
                println!("  {name:<17} vs pimc={vp:.2}  vs material={vm:.2}  (teacher={teacher})");
            }
        }
    }

    fn envu(k: &str, default: u32) -> u32 {
        std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(default)
    }
}

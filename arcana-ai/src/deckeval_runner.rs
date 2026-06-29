//! Deckeval experiment runner — a reproducible gauntlet harness that ranks
//! decks with PAIRED-SEED seat swaps and bootstrap confidence intervals. This
//! is the evaluation-rigor layer called for in `docs/rl-status.md` peer
//! feedback ("paired seeds + seat swaps + bootstrap CIs; ~100 games = a
//! screen, 400+ = a claim").
//!
//! # Why this exists next to [`crate::deckeval::rank_decks`]
//!
//! `rank_decks` answers "which deck wins more" with a raw win matrix. This
//! runner adds the STATISTICS needed to make a defensible *claim*:
//!
//! - **Paired seeds.** Each unordered deck pair plays in DUELS; one duel = two
//!   games that share a single engine seed AND per-deck policy seeds but swap
//!   seats. The shuffle/draw luck and policy RNG are therefore common to both
//!   seat orders, so the comparison isolates the deck/seat effect (antithetic
//!   variance reduction). `crate::search::win_rate` alternates seats but seeds
//!   each game independently — this runner pairs them.
//! - **Bootstrap CIs.** Per-deck win-rate comes with a percentile bootstrap
//!   95% CI over that deck's own per-game outcome vector, so "deck A > deck B"
//!   can be read against the interval overlap rather than a point estimate.
//! - **CSV emission.** [`GauntletReport::to_csv`] dumps a self-describing table
//!   for experiment logs / downstream analysis.
//!
//! # Referee knob
//!
//! The *referee* is the policy BOTH seats play under (we measure decks, not
//! policies). [`Referee::Random`] is cheapest (screening + tests),
//! [`Referee::VmcMaterial`] is the standard cheap referee (matches
//! [`crate::deckeval::fixed_policy`]), and [`Referee::Pimc`] is the strongest
//! but slowest (it samples determinizations from each matchup's decks).
//!
//! # Sample sizing
//!
//! A deck's game count is `2 * paired_duels_per_pair * (n_decks - 1)`. Per the
//! feedback, target ~100 games/deck for a screen and 400+ before a claim.

use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::CardId;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::deckeval::{fixed_policy, Deck};
use crate::information_set::DeckList;
use crate::search::{play_match, PimcPolicy, RandomStatePolicy, StatePolicy};

// PIMC referee budget — deliberately small (a gauntlet plays many full games).
const PIMC_SAMPLES: u32 = 8;
const PIMC_CAP: u32 = 80;
const PIMC_MAX_CANDIDATES: usize = 8;

// =============================================================================
// Referee
// =============================================================================

/// The policy BOTH seats play under during a gauntlet (we are measuring decks,
/// not policies, so the seats are symmetric).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Referee {
    /// Uniform-random legal action. Cheapest; for screening + fast tests.
    Random,
    /// `ValueMc(Material)` at the deckeval fixed budget — the standard cheap
    /// referee (see [`crate::deckeval::fixed_policy`]).
    VmcMaterial,
    /// Perfect-Information Monte-Carlo at a small budget. Strongest referee but
    /// the slowest; needs the matchup's decks for determinization sampling.
    Pimc,
}

impl Referee {
    /// Short stable label used in tables / CSV.
    pub fn label(self) -> &'static str {
        match self {
            Referee::Random => "random",
            Referee::VmcMaterial => "vmc-material",
            Referee::Pimc => "pimc",
        }
    }
}

/// Build a per-seat policy maker for ONE game. `seat_decks` are the two
/// decklists in physical seat order (PIMC samples determinizations from them;
/// the simpler referees ignore them). The returned closure builds a fresh
/// policy seeded by its `u64` argument.
fn maker_for(
    referee: Referee,
    seat_decks: &[Vec<CardId>],
) -> Box<dyn Fn(u64) -> Box<dyn StatePolicy>> {
    match referee {
        Referee::Random => {
            Box::new(|s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) })
                as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>
        }
        Referee::VmcMaterial => {
            Box::new(|s: u64| -> Box<dyn StatePolicy> { fixed_policy(s) })
                as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>
        }
        Referee::Pimc => {
            let decks: Vec<DeckList> = seat_decks.to_vec();
            Box::new(move |s: u64| -> Box<dyn StatePolicy> {
                Box::new(PimcPolicy::with_budget(
                    s,
                    decks.clone(),
                    PIMC_SAMPLES,
                    PIMC_CAP,
                    PIMC_MAX_CANDIDATES,
                ))
            }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>
        }
    }
}

// =============================================================================
// Config
// =============================================================================

/// One reproducible gauntlet's parameters. The whole run is deterministic in
/// `base_seed`.
#[derive(Clone, Debug)]
pub struct ExperimentConfig {
    /// Policy both seats play under.
    pub referee: Referee,
    /// Seed-paired duels per unordered deck pair. Each duel plays TWO games that
    /// share one engine + per-deck policy seed but swap seats, so shared luck
    /// cancels. A deck's total games = `2 * paired_duels_per_pair * (n_decks-1)`.
    pub paired_duels_per_pair: u32,
    /// Per-game step cap (hit cap = [`GameResult::Draw`]). ~4000 guarantees
    /// termination.
    pub max_steps: u32,
    /// Root seed — the entire gauntlet (engine RNG, policy RNG, bootstrap) is
    /// deterministic in it.
    pub base_seed: u64,
    /// Bootstrap resamples for each per-deck 95% CI (e.g. 2000; 0 collapses the
    /// CI to the point estimate).
    pub bootstrap_samples: u32,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            referee: Referee::VmcMaterial,
            paired_duels_per_pair: 25, // 50 games/pair
            max_steps: 4000,
            base_seed: 0,
            bootstrap_samples: 2000,
        }
    }
}

// =============================================================================
// Report
// =============================================================================

/// Per-deck summary line.
#[derive(Clone, Debug)]
pub struct DeckStat {
    pub name: String,
    pub games: u32,
    /// Points scored (win = 1.0, draw = 0.5), so `win_rate = score / games`.
    pub score: f32,
    pub win_rate: f32,
    /// Percentile bootstrap 95% CI bounds on `win_rate`.
    pub ci_lo: f32,
    pub ci_hi: f32,
}

/// The result of a [`run_gauntlet`].
pub struct GauntletReport {
    pub referee: &'static str,
    pub base_seed: u64,
    pub paired_duels_per_pair: u32,
    pub max_steps: u32,
    pub bootstrap_samples: u32,
    pub deck_stats: Vec<DeckStat>,
    /// `score_matrix[i][j]` = points deck `i` scored vs deck `j` (win 1, draw
    /// 0.5). Diagonal 0.
    pub score_matrix: Vec<Vec<f32>>,
    /// `games_matrix[i][j]` = games the pair played. Symmetric, diagonal 0.
    pub games_matrix: Vec<Vec<u32>>,
    /// Raw per-deck per-game outcomes (for re-bootstrapping / external stats).
    pub per_deck_outcomes: Vec<Vec<f32>>,
}

impl GauntletReport {
    /// Deck indices sorted by win-rate descending (ties broken by score).
    pub fn ranking(&self) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..self.deck_stats.len()).collect();
        idx.sort_by(|&a, &b| {
            self.deck_stats[b]
                .win_rate
                .partial_cmp(&self.deck_stats[a].win_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(
                    self.deck_stats[b]
                        .score
                        .partial_cmp(&self.deck_stats[a].score)
                        .unwrap_or(std::cmp::Ordering::Equal),
                )
        });
        idx
    }

    /// Win-rate of `i` vs `j` in `[0, 1]` (0 if they never met).
    fn pair_win_rate(&self, i: usize, j: usize) -> f32 {
        let g = self.games_matrix[i][j];
        if g == 0 {
            0.0
        } else {
            self.score_matrix[i][j] / g as f32
        }
    }

    /// Human-readable matrix + win-rate / 95%-CI summary + ranking line.
    pub fn format_table(&self) -> String {
        let n = self.deck_stats.len();
        let mut s = String::new();
        s.push_str(&format!(
            "gauntlet: referee={} base_seed={} duels/pair={} (x2 seat-swapped) bootstrap={}\n",
            self.referee, self.base_seed, self.paired_duels_per_pair, self.bootstrap_samples
        ));
        let w = self
            .deck_stats
            .iter()
            .map(|d| d.name.len())
            .max()
            .unwrap_or(6)
            .max(6);
        // Header.
        s.push_str(&format!("{:>w$} |", "", w = w));
        for d in &self.deck_stats {
            s.push_str(&format!(" {:>9}", d.name));
        }
        s.push_str("  |    win%        95% CI    games\n");
        // Rows.
        for i in 0..n {
            s.push_str(&format!("{:>w$} |", self.deck_stats[i].name, w = w));
            for j in 0..n {
                if i == j {
                    s.push_str(&format!(" {:>9}", "—"));
                } else {
                    s.push_str(&format!(" {:>9.1}", self.pair_win_rate(i, j) * 100.0));
                }
            }
            let d = &self.deck_stats[i];
            s.push_str(&format!(
                "  | {:>6.1}%  [{:>5.1},{:>5.1}] {:>6}\n",
                d.win_rate * 100.0,
                d.ci_lo * 100.0,
                d.ci_hi * 100.0,
                d.games
            ));
        }
        // Ranking line.
        s.push_str("ranking: ");
        let rank: Vec<String> = self
            .ranking()
            .iter()
            .map(|&i| {
                format!(
                    "{}({:.1}%)",
                    self.deck_stats[i].name,
                    self.deck_stats[i].win_rate * 100.0
                )
            })
            .collect();
        s.push_str(&rank.join(" > "));
        s
    }

    /// Self-describing CSV: a `#`-prefixed config comment line, then a header
    /// row, then one row per deck. Pandas reads it with `comment='#'`.
    pub fn to_csv(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "# gauntlet referee={} base_seed={} paired_duels_per_pair={} max_steps={} bootstrap_samples={}\n",
            self.referee,
            self.base_seed,
            self.paired_duels_per_pair,
            self.max_steps,
            self.bootstrap_samples
        ));
        s.push_str("deck,games,wins,win_rate,ci_lo,ci_hi\n");
        for d in &self.deck_stats {
            s.push_str(&format!(
                "{},{},{:.3},{:.4},{:.4},{:.4}\n",
                csv_escape(&d.name),
                d.games,
                d.score,
                d.win_rate,
                d.ci_lo,
                d.ci_hi
            ));
        }
        s
    }
}

/// Minimal CSV field escaping (quote if it contains a comma/quote/newline).
fn csv_escape(field: &str) -> String {
    if field.contains([',', '"', '\n']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

// =============================================================================
// The runner
// =============================================================================

/// Run a paired-seed, seat-swapped gauntlet over `decks` and return a
/// [`GauntletReport`] with per-deck win-rates + bootstrap 95% CIs.
pub fn run_gauntlet(
    decks: &[Deck],
    registry: &CardRegistry,
    cfg: &ExperimentConfig,
) -> GauntletReport {
    let n = decks.len();
    let mut score_matrix = vec![vec![0.0f32; n]; n];
    let mut games_matrix = vec![vec![0u32; n]; n];
    let mut per_deck_outcomes: Vec<Vec<f32>> = vec![Vec::new(); n];

    for i in 0..n {
        for j in (i + 1)..n {
            for d in 0..cfg.paired_duels_per_pair {
                let engine_seed = mix4(cfg.base_seed, i as u64, j as u64, d as u64);
                // Per-DECK policy seeds, constant across the seat swap so the
                // policy RNG is shared between both seatings (paired design).
                let seed_i = engine_seed ^ 0xA5A5_A5A5_A5A5_A5A5;
                let seed_j = engine_seed ^ 0x5A5A_5A5A_5A5A_5A5A;

                // Game A: deck i = seat 0, deck j = seat 1.
                let seat_a = vec![decks[i].cards.clone(), decks[j].cards.clone()];
                let mk_a = maker_for(cfg.referee, &seat_a);
                let ra = play_seated(seat_a, registry, engine_seed, &*mk_a, seed_i, seed_j, cfg.max_steps);
                let (s0, s1) = seat_scores(&ra);
                per_deck_outcomes[i].push(s0);
                per_deck_outcomes[j].push(s1);
                score_matrix[i][j] += s0;
                score_matrix[j][i] += s1;

                // Game B (paired): swap seats — deck j = seat 0, deck i = seat 1
                // — SAME engine seed, SAME per-deck policy seeds.
                let seat_b = vec![decks[j].cards.clone(), decks[i].cards.clone()];
                let mk_b = maker_for(cfg.referee, &seat_b);
                let rb = play_seated(seat_b, registry, engine_seed, &*mk_b, seed_j, seed_i, cfg.max_steps);
                let (t0, t1) = seat_scores(&rb);
                per_deck_outcomes[j].push(t0);
                per_deck_outcomes[i].push(t1);
                score_matrix[j][i] += t0;
                score_matrix[i][j] += t1;

                games_matrix[i][j] += 2;
                games_matrix[j][i] += 2;
            }
        }
    }

    // Bootstrap each deck's CI with an independent, deterministic RNG.
    let mut boot_rng = ChaCha8Rng::seed_from_u64(cfg.base_seed ^ 0xB007_5712_0000_0000);
    let deck_stats: Vec<DeckStat> = (0..n)
        .map(|i| {
            let outc = &per_deck_outcomes[i];
            let games = outc.len() as u32;
            let score: f32 = outc.iter().sum();
            let win_rate = if games == 0 { 0.0 } else { score / games as f32 };
            let (ci_lo, ci_hi) = bootstrap_ci(outc, cfg.bootstrap_samples, &mut boot_rng);
            DeckStat {
                name: decks[i].name.clone(),
                games,
                score,
                win_rate,
                ci_lo,
                ci_hi,
            }
        })
        .collect();

    GauntletReport {
        referee: cfg.referee.label(),
        base_seed: cfg.base_seed,
        paired_duels_per_pair: cfg.paired_duels_per_pair,
        max_steps: cfg.max_steps,
        bootstrap_samples: cfg.bootstrap_samples,
        deck_stats,
        score_matrix,
        games_matrix,
        per_deck_outcomes,
    }
}

/// Play one game with the two seat-ordered decks under `mk`, seeding seat 0's
/// policy from `policy_seed_0` and seat 1's from `policy_seed_1`.
fn play_seated(
    seat_decks: Vec<Vec<CardId>>,
    registry: &CardRegistry,
    engine_seed: u64,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
    policy_seed_0: u64,
    policy_seed_1: u64,
    max_steps: u32,
) -> GameResult {
    let mut p0 = mk(policy_seed_0);
    let mut p1 = mk(policy_seed_1);
    let mut slots: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];
    play_match(seat_decks, registry, engine_seed, &mut slots, max_steps)
}

/// Map a result to (seat-0 score, seat-1 score): win 1.0, loss 0.0, draw 0.5.
fn seat_scores(r: &GameResult) -> (f32, f32) {
    match r {
        GameResult::Win(0) | GameResult::Eliminated(0) => (1.0, 0.0),
        GameResult::Win(_) | GameResult::Eliminated(_) => (0.0, 1.0),
        GameResult::Draw => (0.5, 0.5),
    }
}

// =============================================================================
// Bootstrap
// =============================================================================

/// Percentile bootstrap 95% CI on the mean of `outcomes`. Resamples with
/// replacement `samples` times using `rng`. Empty input → `(0, 0)`; `samples ==
/// 0` → the point estimate for both bounds.
fn bootstrap_ci(outcomes: &[f32], samples: u32, rng: &mut ChaCha8Rng) -> (f32, f32) {
    if outcomes.is_empty() {
        return (0.0, 0.0);
    }
    let mean = outcomes.iter().sum::<f32>() / outcomes.len() as f32;
    if samples == 0 {
        return (mean, mean);
    }
    let n = outcomes.len();
    let mut means: Vec<f32> = Vec::with_capacity(samples as usize);
    for _ in 0..samples {
        let mut sum = 0.0f64;
        for _ in 0..n {
            let idx = (rng.gen::<u64>() % n as u64) as usize;
            sum += outcomes[idx] as f64;
        }
        means.push((sum / n as f64) as f32);
    }
    means.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    (percentile(&means, 2.5), percentile(&means, 97.5))
}

/// Linear-interpolated percentile of a pre-SORTED slice (`p` in `[0, 100]`).
fn percentile(sorted: &[f32], p: f32) -> f32 {
    if sorted.is_empty() {
        return 0.0;
    }
    if sorted.len() == 1 {
        return sorted[0];
    }
    let rank = (p / 100.0) * (sorted.len() as f32 - 1.0);
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        let frac = rank - lo as f32;
        sorted[lo] * (1.0 - frac) + sorted[hi] * frac
    }
}

/// SplitMix64-style deterministic mix of four `u64`s (gauntlet seed derivation).
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

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deckeval::{mono_color_creature_deck, standard_deck_set};

    /// Three tiny mono decks — fast to play under the random referee.
    fn tiny_decks(reg: &CardRegistry) -> Vec<Deck> {
        vec![
            mono_color_creature_deck(reg, 'R', 4, 8, 10, 4),
            mono_color_creature_deck(reg, 'G', 4, 8, 10, 4),
            mono_color_creature_deck(reg, 'U', 4, 8, 10, 4),
        ]
    }

    /// FAST end-to-end smoke: a tiny random-referee gauntlet produces a
    /// well-formed report — right deck count, conserved games/score, CIs that
    /// bracket the point estimate, and parseable CSV.
    #[test]
    fn gauntlet_runs_and_report_is_well_formed() {
        let reg = arcana_cards::build_catalog();
        let decks = tiny_decks(&reg);
        let n = decks.len();
        let cfg = ExperimentConfig {
            referee: Referee::Random,
            paired_duels_per_pair: 1, // 2 games/pair
            max_steps: 4000,
            base_seed: 7,
            bootstrap_samples: 200,
        };
        let report = run_gauntlet(&decks, &reg, &cfg);

        // One stat per deck.
        assert_eq!(report.deck_stats.len(), n);

        // Each deck plays 2 * duels * (n-1) games.
        let expected_games = 2 * cfg.paired_duels_per_pair * (n as u32 - 1);
        let mut total_games = 0u32;
        let mut total_score = 0.0f32;
        for d in &report.deck_stats {
            assert_eq!(d.games, expected_games, "deck {} game count", d.name);
            assert!((0.0..=1.0).contains(&d.win_rate), "win_rate {}", d.win_rate);
            assert!((0.0..=1.0).contains(&d.ci_lo) && (0.0..=1.0).contains(&d.ci_hi));
            assert!(d.ci_lo <= d.win_rate + 1e-4, "ci_lo above point est");
            assert!(d.ci_hi >= d.win_rate - 1e-4, "ci_hi below point est");
            total_games += d.games;
            total_score += d.score;
        }

        // Conservation: every physical game distributes exactly 1.0 point and is
        // counted by both of its decks.
        let physical_games = (n * (n - 1) / 2) as u32 * cfg.paired_duels_per_pair * 2;
        assert_eq!(total_games, 2 * physical_games);
        assert!(
            (total_score - physical_games as f32).abs() < 1e-2,
            "score not conserved: {total_score} vs {physical_games}"
        );

        // CSV: skip the `#` comment, then header + one row per deck, 6 cols each.
        let csv = report.to_csv();
        let lines: Vec<&str> = csv
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
            .collect();
        assert_eq!(lines.len(), n + 1, "header + {n} deck rows");
        assert_eq!(lines[0], "deck,games,wins,win_rate,ci_lo,ci_hi");
        for row in &lines[1..] {
            assert_eq!(row.split(',').count(), 6, "row has 6 columns: {row}");
        }

        // The table renders without panicking and names the ranking.
        let table = report.format_table();
        assert!(table.contains("ranking:"));
    }

    /// Determinism: same config + seed → identical win-rates.
    #[test]
    fn gauntlet_is_deterministic_in_seed() {
        let reg = arcana_cards::build_catalog();
        let decks = tiny_decks(&reg);
        let cfg = ExperimentConfig {
            referee: Referee::Random,
            paired_duels_per_pair: 1,
            max_steps: 4000,
            base_seed: 42,
            bootstrap_samples: 64,
        };
        let a = run_gauntlet(&decks, &reg, &cfg);
        let b = run_gauntlet(&decks, &reg, &cfg);
        for (x, y) in a.deck_stats.iter().zip(b.deck_stats.iter()) {
            assert_eq!(x.win_rate, y.win_rate, "non-deterministic win-rate");
            assert_eq!(x.ci_lo, y.ci_lo);
            assert_eq!(x.ci_hi, y.ci_hi);
        }
    }

    /// EXAMPLE TEMPLATE for a real measurement gauntlet (slow). Run in release:
    /// `cargo test -p arcana-ai --release gauntlet_measurement -- --ignored --nocapture`
    /// Swap in `Referee::Pimc` for the strongest (slowest) referee, raise
    /// `paired_duels_per_pair` toward 200+ for a claim-grade sample, and point
    /// `decks` at real Pro-Tour / meta decklists once those are sourced.
    #[test]
    #[ignore]
    fn gauntlet_measurement() {
        let reg = arcana_cards::build_catalog();
        let decks = standard_deck_set(&reg);
        let cfg = ExperimentConfig {
            referee: Referee::VmcMaterial,
            paired_duels_per_pair: 25, // 50 games/pair
            max_steps: 4000,
            base_seed: 0,
            bootstrap_samples: 2000,
        };
        let report = run_gauntlet(&decks, &reg, &cfg);
        println!("\n{}\n", report.format_table());
        println!("{}", report.to_csv());
    }
}

//! Deckeval experiment runner — a reproducible gauntlet harness that ranks
//! decks with seat-swapped common-seed DUELS and block bootstrap confidence
//! intervals. This is the evaluation-rigor layer called for in
//! `docs/rl-status.md` peer feedback ("paired seeds + seat swaps + bootstrap
//! CIs; ~100 games = a screen, 400+ = a claim").
//!
//! # Why this exists next to [`crate::deckeval::rank_decks`]
//!
//! `rank_decks` answers "which deck wins more" with a raw win matrix. This
//! runner adds the STATISTICS needed to make a defensible *claim*:
//!
//! - **Seat-swapped common seeds (duel blocks).** Each unordered deck pair
//!   plays in DUELS; one duel = two games that share a single engine seed and
//!   per-deck policy seeds but swap seats. NOTE the engine seeds each library
//!   shuffle by PHYSICAL seat (`GameState::shuffle_library` keys off
//!   `rng_seed + player`), so a deck does NOT draw the same shuffle in both
//!   games — but the duel's two shuffle streams are FIXED and seat-swapping
//!   balances which deck draws which stream, so seat/stream luck cancels at the
//!   DUEL (block) level rather than per game. Per-deck POLICY seeds ARE held
//!   constant across the swap, so policy RNG cancels per deck.
//!   (`crate::search::win_rate` alternates seats but seeds every game
//!   independently — this runner shares the seed within a duel.)
//! - **Block bootstrap CIs.** The experimental unit is the DUEL, not the game:
//!   a duel's two games deliberately share seeds and are correlated, so the CI
//!   resamples per-deck DUEL-block scores (each = the deck's mean points over
//!   its two games in that duel), NOT individual games. Resampling games as iid
//!   would treat correlated observations as independent and make the interval
//!   too optimistic.
//! - **CSV emission.** [`GauntletReport::to_csv`] dumps a self-describing table
//!   (true win/draw/loss counts + a points-based `point_rate`) for experiment
//!   logs / downstream analysis.
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
//! A deck's game count is `2 * paired_duels_per_pair * (n_decks - 1)` (and half
//! that many duel blocks). Per the feedback, target ~100 games/deck for a
//! screen and 400+ before a claim.

use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::CardId;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::deckeval::Deck;
use crate::information_set::DeckList;
use crate::search::{play_match, MaterialValue, PimcPolicy, RandomStatePolicy, StatePolicy, ValueMcPolicy};

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
    /// Progress-biased random legal action ([`RandomStatePolicy`]). Cheapest;
    /// for screening + fast tests.
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
    budgets: &RefereeBudgets,
    seat_decks: &[Vec<CardId>],
) -> Box<dyn Fn(u64) -> Box<dyn StatePolicy>> {
    let b = *budgets;
    match referee {
        Referee::Random => {
            Box::new(|s: u64| -> Box<dyn StatePolicy> { Box::new(RandomStatePolicy::new(s)) })
                as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>
        }
        Referee::VmcMaterial => {
            Box::new(move |s: u64| -> Box<dyn StatePolicy> {
                Box::new(ValueMcPolicy::with_budget(
                    Box::new(MaterialValue),
                    s,
                    b.vmc_rollouts,
                    b.vmc_depth,
                    b.vmc_candidates,
                ))
            }) as Box<dyn Fn(u64) -> Box<dyn StatePolicy>>
        }
        Referee::Pimc => {
            let decks: Vec<DeckList> = seat_decks.to_vec();
            Box::new(move |s: u64| -> Box<dyn StatePolicy> {
                Box::new(PimcPolicy::with_budget(
                    s,
                    decks.clone(),
                    b.pimc_samples,
                    b.pimc_cap,
                    b.pimc_candidates,
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
/// The referees' search budgets. [`Default`] is the field gauntlet's own, as
/// July ran it: material Monte Carlo at deckeval's fixed budget (4 rollouts,
/// depth 20, 8 candidates) and PIMC at 8 samples, cap 80, 8 candidates.
/// [`RefereeBudgets::capsule`] is A2.1's capsule policies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RefereeBudgets {
    pub vmc_rollouts: u32,
    pub vmc_depth: u32,
    pub vmc_candidates: usize,
    pub pimc_samples: u32,
    pub pimc_cap: u32,
    pub pimc_candidates: usize,
}

impl Default for RefereeBudgets {
    fn default() -> Self {
        Self {
            vmc_rollouts: crate::deckeval::FIXED_ROLLOUTS,
            vmc_depth: crate::deckeval::FIXED_ROLLOUT_CAP,
            vmc_candidates: crate::deckeval::FIXED_MAX_CANDIDATES,
            pimc_samples: PIMC_SAMPLES,
            pimc_cap: PIMC_CAP,
            pimc_candidates: PIMC_MAX_CANDIDATES,
        }
    }
}

impl RefereeBudgets {
    /// A2.1's capsule policies (`benchmark.rs`'s capsule tests): material
    /// Monte Carlo at 6 rollouts, depth 25, 10 candidates, and PIMC with 10
    /// candidates at the samples and cap given.
    pub fn capsule(pimc_samples: u32, pimc_cap: u32) -> Self {
        Self {
            vmc_rollouts: 6,
            vmc_depth: 25,
            vmc_candidates: 10,
            pimc_samples,
            pimc_cap,
            pimc_candidates: 10,
        }
    }

    /// As the field tests read the environment: `CORPUS_POLICIES=capsule`
    /// selects [`Self::capsule`], anything else the default, and
    /// `CORPUS_PIMC_SAMPLES` and `CORPUS_PIMC_CAP` set PIMC's budget either way.
    pub fn from_env() -> Self {
        let num = |k: &str, d: u32| std::env::var(k).ok().and_then(|s| s.parse().ok()).unwrap_or(d);
        let samples = num("CORPUS_PIMC_SAMPLES", PIMC_SAMPLES);
        let cap = num("CORPUS_PIMC_CAP", PIMC_CAP);
        if std::env::var("CORPUS_POLICIES").as_deref() == Ok("capsule") {
            Self::capsule(samples, cap)
        } else {
            Self { pimc_samples: samples, pimc_cap: cap, ..Self::default() }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExperimentConfig {
    /// Policy both seats play under.
    pub referee: Referee,
    /// Seed-paired duels per unordered deck pair. Each duel plays TWO games that
    /// share one engine seed + per-deck policy seeds but swap seats. A deck's
    /// total games = `2 * paired_duels_per_pair * (n_decks-1)`; its duel-block
    /// count is half that.
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
    /// The referee's search budgets.
    pub budgets: RefereeBudgets,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            referee: Referee::VmcMaterial,
            paired_duels_per_pair: 25, // 50 games/pair
            max_steps: 4000,
            base_seed: 0,
            bootstrap_samples: 2000,
            budgets: RefereeBudgets::default(),
        }
    }
}

// =============================================================================
// Report
// =============================================================================

/// Per-deck summary line. `wins`/`draws`/`losses` are TRUE game counts;
/// `point_rate` is the points-based rate (a draw is half a point), which is what
/// the bootstrap CI is computed on.
#[derive(Clone, Debug)]
pub struct DeckStat {
    pub name: String,
    pub games: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    /// Points scored (win = 1.0, draw = 0.5), so `point_rate = score / games`.
    pub score: f32,
    /// Match-win rate counting draws as half a point, in `[0, 1]`.
    pub point_rate: f32,
    /// Percentile bootstrap 95% CI bounds on `point_rate`, resampled over the
    /// deck's DUEL BLOCKS (not individual games).
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
    /// Raw per-deck per-game outcomes (1.0 win / 0.5 draw / 0.0 loss), for
    /// external analysis. NOTE the CI uses `per_deck_blocks`, not these.
    pub per_deck_outcomes: Vec<Vec<f32>>,
    /// Per-deck DUEL-BLOCK scores (each = the deck's mean points over a duel's
    /// two games). The bootstrap's experimental unit.
    pub per_deck_blocks: Vec<Vec<f32>>,
}

impl GauntletReport {
    /// Deck indices sorted by `point_rate` descending (ties broken by score).
    pub fn ranking(&self) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..self.deck_stats.len()).collect();
        idx.sort_by(|&a, &b| {
            self.deck_stats[b]
                .point_rate
                .partial_cmp(&self.deck_stats[a].point_rate)
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

    /// Point-rate of `i` vs `j` in `[0, 1]` (0 if they never met).
    fn pair_point_rate(&self, i: usize, j: usize) -> f32 {
        let g = self.games_matrix[i][j];
        if g == 0 {
            0.0
        } else {
            self.score_matrix[i][j] / g as f32
        }
    }

    /// Human-readable matrix + point-rate / 95%-CI / W-D-L summary + ranking.
    pub fn format_table(&self) -> String {
        let n = self.deck_stats.len();
        let mut s = String::new();
        s.push_str(&format!(
            "gauntlet: referee={} base_seed={} duels/pair={} (x2 seat-swapped) bootstrap={} (block)\n",
            self.referee, self.base_seed, self.paired_duels_per_pair, self.bootstrap_samples
        ));
        let w = self
            .deck_stats
            .iter()
            .map(|d| d.name.len())
            .max()
            .unwrap_or(6)
            .max(6);
        // Header (cells = row deck's point% vs column deck).
        s.push_str(&format!("{:>w$} |", "", w = w));
        for d in &self.deck_stats {
            s.push_str(&format!(" {:>9}", d.name));
        }
        s.push_str("  |  point%        95% CI       W-D-L\n");
        // Rows.
        for i in 0..n {
            s.push_str(&format!("{:>w$} |", self.deck_stats[i].name, w = w));
            for j in 0..n {
                if i == j {
                    s.push_str(&format!(" {:>9}", "—"));
                } else {
                    s.push_str(&format!(" {:>9.1}", self.pair_point_rate(i, j) * 100.0));
                }
            }
            let d = &self.deck_stats[i];
            s.push_str(&format!(
                "  | {:>6.1}%  [{:>5.1},{:>5.1}]  {}-{}-{}\n",
                d.point_rate * 100.0,
                d.ci_lo * 100.0,
                d.ci_hi * 100.0,
                d.wins,
                d.draws,
                d.losses
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
                    self.deck_stats[i].point_rate * 100.0
                )
            })
            .collect();
        s.push_str(&rank.join(" > "));
        s
    }

    /// Self-describing CSV: a `#`-prefixed config comment line, then a header
    /// row, then one row per deck. Pandas reads it with `comment='#'`.
    /// `score`/`point_rate` count a draw as half; `wins`/`draws`/`losses` are
    /// true game counts.
    pub fn to_csv(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!(
            "# gauntlet referee={} base_seed={} paired_duels_per_pair={} max_steps={} bootstrap_samples={} ci=block-bootstrap\n",
            self.referee,
            self.base_seed,
            self.paired_duels_per_pair,
            self.max_steps,
            self.bootstrap_samples
        ));
        s.push_str("deck,games,wins,draws,losses,score,point_rate,ci_lo,ci_hi\n");
        for d in &self.deck_stats {
            s.push_str(&format!(
                "{},{},{},{},{},{:.3},{:.4},{:.4},{:.4}\n",
                csv_escape(&d.name),
                d.games,
                d.wins,
                d.draws,
                d.losses,
                d.score,
                d.point_rate,
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

/// Run a seat-swapped, common-seed gauntlet over `decks` and return a
/// [`GauntletReport`] with per-deck point-rates + block bootstrap 95% CIs.
/// The unordered deck pairs of an `n`-deck gauntlet, in the order
/// [`run_gauntlet`] plays them: (0, 1), (0, 2), …, (n-2, n-1). A shard of a
/// gauntlet is the pairs whose index in this list is `k` modulo `K`.
pub fn gauntlet_pairs(n: usize) -> Vec<(usize, usize)> {
    (0..n).flat_map(|i| (i + 1..n).map(move |j| (i, j))).collect()
}

/// One duel's four scores: game A seats deck `i` first, game B swaps seats.
/// Each is a deck's points in that game (win 1, draw ½, loss 0).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DuelResult {
    pub a_i: f32,
    pub a_j: f32,
    pub b_i: f32,
    pub b_j: f32,
}

/// Duel `d` of the pair (`i`, `j`), as [`run_gauntlet`] plays it: both games
/// share the engine seed `mix4(base_seed, i, j, d)` and per-deck policy seeds,
/// and swap seats. Its seeds depend only on the pair and the duel, so any
/// subset of a gauntlet's duels played apart gives the same games.
pub fn play_duel(
    decks: &[Deck],
    registry: &CardRegistry,
    cfg: &ExperimentConfig,
    i: usize,
    j: usize,
    d: u32,
) -> DuelResult {
    let engine_seed = mix4(cfg.base_seed, i as u64, j as u64, d as u64);
    // Per-DECK policy seeds, constant across the seat swap so the
    // policy RNG is shared between both seatings (paired per deck).
    let seed_i = engine_seed ^ 0xA5A5_A5A5_A5A5_A5A5;
    let seed_j = engine_seed ^ 0x5A5A_5A5A_5A5A_5A5A;

    // Game A: deck i = seat 0, deck j = seat 1.
    let seat_a = vec![decks[i].cards.clone(), decks[j].cards.clone()];
    let mk_a = maker_for(cfg.referee, &cfg.budgets, &seat_a);
    let ra = play_seated(seat_a, registry, engine_seed, &*mk_a, seed_i, seed_j, cfg.max_steps);
    let (a_i, a_j) = seat_scores(&ra); // (deck i score, deck j score)

    // Game B (seat swap): deck j = seat 0, deck i = seat 1 — SAME
    // engine seed, SAME per-deck policy seeds.
    let seat_b = vec![decks[j].cards.clone(), decks[i].cards.clone()];
    let mk_b = maker_for(cfg.referee, &cfg.budgets, &seat_b);
    let rb = play_seated(seat_b, registry, engine_seed, &*mk_b, seed_j, seed_i, cfg.max_steps);
    let (b_j, b_i) = seat_scores(&rb); // (deck j score, deck i score)
    DuelResult { a_i, a_j, b_i, b_j }
}

pub fn run_gauntlet(
    decks: &[Deck],
    registry: &CardRegistry,
    cfg: &ExperimentConfig,
) -> GauntletReport {
    let n = decks.len();
    let mut score_matrix = vec![vec![0.0f32; n]; n];
    let mut games_matrix = vec![vec![0u32; n]; n];
    let mut per_deck_outcomes: Vec<Vec<f32>> = vec![Vec::new(); n];
    let mut per_deck_blocks: Vec<Vec<f32>> = vec![Vec::new(); n];

    for (i, j) in gauntlet_pairs(n) {
        for d in 0..cfg.paired_duels_per_pair {
            let DuelResult { a_i, a_j, b_i, b_j } = play_duel(decks, registry, cfg, i, j, d);

            // Per-game outcomes (for transparency / external stats).
            per_deck_outcomes[i].push(a_i);
            per_deck_outcomes[i].push(b_i);
            per_deck_outcomes[j].push(a_j);
            per_deck_outcomes[j].push(b_j);

            // The duel BLOCK = each deck's mean points over its two games.
            // This is the bootstrap's experimental unit (the two games share
            // seeds and are correlated).
            per_deck_blocks[i].push((a_i + b_i) / 2.0);
            per_deck_blocks[j].push((a_j + b_j) / 2.0);

            score_matrix[i][j] += a_i + b_i;
            score_matrix[j][i] += a_j + b_j;
            games_matrix[i][j] += 2;
            games_matrix[j][i] += 2;
        }
    }

    // Bootstrap each deck's CI over its DUEL BLOCKS with an independent,
    // deterministic RNG.
    let mut boot_rng = ChaCha8Rng::seed_from_u64(cfg.base_seed ^ 0xB007_5712_0000_0000);
    let deck_stats: Vec<DeckStat> = (0..n)
        .map(|i| {
            let outc = &per_deck_outcomes[i];
            let games = outc.len() as u32;
            let wins = outc.iter().filter(|&&x| x == 1.0).count() as u32;
            let draws = outc.iter().filter(|&&x| x == 0.5).count() as u32;
            let losses = outc.iter().filter(|&&x| x == 0.0).count() as u32;
            let score: f32 = outc.iter().sum();
            let point_rate = if games == 0 { 0.0 } else { score / games as f32 };
            let (ci_lo, ci_hi) =
                bootstrap_ci(&per_deck_blocks[i], cfg.bootstrap_samples, &mut boot_rng);
            DeckStat {
                name: decks[i].name.clone(),
                games,
                wins,
                draws,
                losses,
                score,
                point_rate,
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
        per_deck_blocks,
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
/// `Eliminated(p)` means player `p` LOST (matches [`crate::learn`] /
/// [`crate::reward`]), so the OTHER seat scores the win.
fn seat_scores(r: &GameResult) -> (f32, f32) {
    match r {
        GameResult::Win(0) => (1.0, 0.0),
        GameResult::Win(_) => (0.0, 1.0),
        GameResult::Eliminated(0) => (0.0, 1.0),
        GameResult::Eliminated(_) => (1.0, 0.0),
        GameResult::Draw => (0.5, 0.5),
    }
}

// =============================================================================
// Bootstrap
// =============================================================================

/// Percentile bootstrap 95% CI on the mean of `samples_in` (the per-deck DUEL
/// BLOCK scores). Resamples with replacement `n_boot` times using `rng`. Empty
/// input → `(0, 0)`; `n_boot == 0` → the point estimate for both bounds.
fn bootstrap_ci(samples_in: &[f32], n_boot: u32, rng: &mut ChaCha8Rng) -> (f32, f32) {
    if samples_in.is_empty() {
        return (0.0, 0.0);
    }
    let mean = samples_in.iter().sum::<f32>() / samples_in.len() as f32;
    if n_boot == 0 {
        return (mean, mean);
    }
    let n = samples_in.len();
    let mut means: Vec<f32> = Vec::with_capacity(n_boot as usize);
    for _ in 0..n_boot {
        let mut sum = 0.0f64;
        for _ in 0..n {
            let idx = (rng.gen::<u64>() % n as u64) as usize;
            sum += samples_in[idx] as f64;
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
    /// `RefereeBudgets::capsule` plays A2.1's capsule policies move for move:
    /// a game refereed through `maker_for` records the same actions as one
    /// between policies built directly at A2.1's budgets, for material Monte
    /// Carlo and for PIMC. For material, the field's default budgets record
    /// different actions, so that comparison can tell; for PIMC the test
    /// checks equality with the direct policy and pins both budget sets.
    #[test]
    fn capsule_budgets_play_a21s_policies() {
        use crate::search::play_match_recorded;
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        let seats = vec![deck.clone(), deck.clone()];
        let actions = |mk: &dyn Fn(u64) -> Box<dyn StatePolicy>| {
            let (mut p0, mut p1) = (mk(11), mk(12));
            let (_, rec) = play_match_recorded(seats.clone(), &reg, 5, &mut [p0.as_mut(), p1.as_mut()], 300);
            format!("{:?}", rec.actions)
        };
        let capsule = RefereeBudgets::capsule(2, 12);
        let vmc_direct = |s: u64| {
            Box::new(ValueMcPolicy::with_budget(Box::new(MaterialValue), s, 6, 25, 10)) as Box<dyn StatePolicy>
        };
        let vmc = actions(&*maker_for(Referee::VmcMaterial, &capsule, &seats));
        assert_eq!(vmc, actions(&vmc_direct));
        assert_ne!(vmc, actions(&*maker_for(Referee::VmcMaterial, &RefereeBudgets::default(), &seats)));
        let ds = seats.clone();
        let pimc_direct = move |s: u64| {
            Box::new(PimcPolicy::with_budget(s, ds.clone(), 2, 12, 10)) as Box<dyn StatePolicy>
        };
        assert_eq!(actions(&*maker_for(Referee::Pimc, &capsule, &seats)), actions(&pimc_direct));
        assert_eq!(RefereeBudgets::capsule(16, 160), RefereeBudgets {
            vmc_rollouts: 6, vmc_depth: 25, vmc_candidates: 10,
            pimc_samples: 16, pimc_cap: 160, pimc_candidates: 10,
        });
        assert_eq!(RefereeBudgets::default(), RefereeBudgets {
            vmc_rollouts: 4, vmc_depth: 20, vmc_candidates: 8,
            pimc_samples: 8, pimc_cap: 80, pimc_candidates: 8,
        });
    }

    /// Duels split into pair shards and played apart, in another order, sum
    /// exactly to `run_gauntlet`'s score matrix: no state crosses from one
    /// duel to the next, which is what lets a cluster run a gauntlet as shards.
    #[test]
    fn duel_shards_sum_to_run_gauntlet() {
        let reg = arcana_cards::build_catalog();
        let decks: Vec<Deck> = (1..=3)
            .map(|s| Deck { name: format!("d{s}"), cards: arcana_cards::sample_deck(&reg, s) })
            .collect();
        let cfg = ExperimentConfig {
            referee: Referee::Random,
            paired_duels_per_pair: 2,
            max_steps: 4000,
            base_seed: 7,
            bootstrap_samples: 10,
            budgets: RefereeBudgets::default(),
        };
        let whole = run_gauntlet(&decks, &reg, &cfg);
        let pairs = gauntlet_pairs(decks.len());
        assert_eq!(pairs, vec![(0, 1), (0, 2), (1, 2)]);
        let mut m = vec![vec![0.0f32; 3]; 3];
        for k in [1, 0] {
            for (p, &(i, j)) in pairs.iter().enumerate() {
                if p % 2 != k {
                    continue;
                }
                for d in (0..2).rev() {
                    let r = play_duel(&decks, &reg, &cfg, i, j, d);
                    m[i][j] += r.a_i + r.b_i;
                    m[j][i] += r.a_j + r.b_j;
                }
            }
        }
        assert_eq!(m, whole.score_matrix);
        assert!(
            pairs.iter().any(|&(i, j)| m[i][j] != 2.0),
            "every game a draw: the comparison would hold vacuously"
        );
    }

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
    /// well-formed report — right deck count, conserved games/score, true W-D-L
    /// that sums to games and reconstructs the score, CIs that bracket the point
    /// estimate, and parseable CSV.
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
            budgets: RefereeBudgets::default(),
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
            // True counts sum to games and reconstruct the points score.
            assert_eq!(d.wins + d.draws + d.losses, d.games, "W+D+L != games");
            assert!(
                (d.score - (d.wins as f32 + 0.5 * d.draws as f32)).abs() < 1e-3,
                "score {} != wins {} + 0.5*draws {}",
                d.score,
                d.wins,
                d.draws
            );
            assert!((0.0..=1.0).contains(&d.point_rate), "point_rate {}", d.point_rate);
            assert!((0.0..=1.0).contains(&d.ci_lo) && (0.0..=1.0).contains(&d.ci_hi));
            assert!(d.ci_lo <= d.point_rate + 1e-4, "ci_lo above point est");
            assert!(d.ci_hi >= d.point_rate - 1e-4, "ci_hi below point est");
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

        // CSV: skip the `#` comment, then header + one row per deck, 9 cols each.
        let csv = report.to_csv();
        let lines: Vec<&str> = csv
            .lines()
            .filter(|l| !l.starts_with('#') && !l.is_empty())
            .collect();
        assert_eq!(lines.len(), n + 1, "header + {n} deck rows");
        assert_eq!(
            lines[0],
            "deck,games,wins,draws,losses,score,point_rate,ci_lo,ci_hi"
        );
        for row in &lines[1..] {
            assert_eq!(row.split(',').count(), 9, "row has 9 columns: {row}");
        }

        // The table renders without panicking and names the ranking.
        let table = report.format_table();
        assert!(table.contains("ranking:"));
    }

    /// Determinism: same config + seed → identical point-rates + CIs.
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
            budgets: RefereeBudgets::default(),
        };
        let a = run_gauntlet(&decks, &reg, &cfg);
        let b = run_gauntlet(&decks, &reg, &cfg);
        for (x, y) in a.deck_stats.iter().zip(b.deck_stats.iter()) {
            assert_eq!(x.point_rate, y.point_rate, "non-deterministic point-rate");
            assert_eq!(x.ci_lo, y.ci_lo);
            assert_eq!(x.ci_hi, y.ci_hi);
        }
    }

    /// `Eliminated(p)` scores `p` as the LOSER (the other seat wins), matching
    /// `crate::learn` / `crate::reward`.
    #[test]
    fn eliminated_scores_the_eliminated_player_as_loser() {
        assert_eq!(seat_scores(&GameResult::Eliminated(0)), (0.0, 1.0));
        assert_eq!(seat_scores(&GameResult::Eliminated(1)), (1.0, 0.0));
        assert_eq!(seat_scores(&GameResult::Win(0)), (1.0, 0.0));
        assert_eq!(seat_scores(&GameResult::Win(1)), (0.0, 1.0));
        assert_eq!(seat_scores(&GameResult::Draw), (0.5, 0.5));
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
            budgets: RefereeBudgets::default(),
        };
        let report = run_gauntlet(&decks, &reg, &cfg);
        println!("\n{}\n", report.format_table());
        println!("{}", report.to_csv());
    }
}

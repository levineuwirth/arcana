//! Deck / card POWER-RANKING harness — the empirical data foundation for an
//! MTGA-style "suggested cards" feature and for deck testing.
//!
//! This is the INVERSE of [`crate::search::round_robin`]. That tournament holds
//! the DECK fixed (a mirror match) and varies the POLICY, to tell bots apart.
//! Here we hold the POLICY fixed and vary the DECK, to tell DECKS apart — and,
//! downstream, the cards that compose them. We fix the policy precisely because
//! we are measuring decks, not players: any per-game policy strength difference
//! would confound the deck signal.
//!
//! # The pieces
//!
//! * [`play_deck_match`] — one full deck-vs-deck game under the fixed policy
//!   (deckA = player 0, deckB = player 1), driven via the real engine to a
//!   [`GameResult`] with a `max_steps` cap so it always terminates.
//! * [`standard_deck_set`] — a small, deterministic, documented set of decks to
//!   rank (two `sample_deck` piles + one mono-color creature deck per color).
//! * [`rank_decks`] — a deck round-robin: every pair plays N games (seats
//!   alternated to cancel first-player bias), tallied into a [`DeckRanking`].
//! * [`card_contributions`] — a SIMPLE, honest first-cut per-card signal: the
//!   average win-rate of the decks that contain the card (co-occurrence
//!   weighting). See the confound note on [`card_contributions`].
//! * [`marginal_card_power`] / [`rank_cards_marginal`] — the CAUSAL upgrade that
//!   the [`card_contributions`] confound note asks for. Swap exactly ONE deck
//!   slot (a neutral `filler`) for the card under test, re-measure the deck's
//!   win-rate against a fixed gauntlet, and report the delta. Changing one card
//!   and re-measuring ISOLATES that card's marginal contribution (no
//!   co-occurrence confound), at the cost of a measurement that is relative to
//!   the chosen baseline + filler + gauntlet + referee policy. See its docs.
//!
//! # The fixed policy
//!
//! Both seats play [`ValueMcPolicy`]`(`[`MaterialValue`]`)` — the established
//! hand-tuned value-MC baseline — at a small ("snappy") budget
//! ([`FIXED_ROLLOUTS`] / [`FIXED_ROLLOUT_CAP`] / [`FIXED_MAX_CANDIDATES`]),
//! mirroring the budget `mlp.rs`'s value-MC leaf tournament uses. The
//! `*_with` variants accept any policy maker, so the cheap [`crate::search::
//! RandomStatePolicy`] can drive the fast unit tests while the headline
//! measurement uses the real fixed policy.

use std::collections::HashMap;

use arcana_core::catalog::{self, CardQuery};
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::CardId;

use crate::search::{play_match, win_rate, MaterialValue, StatePolicy, ValueMcPolicy};

// =============================================================================
// The fixed measurement policy
// =============================================================================

/// Rollouts per candidate action for the fixed measurement policy. Small on
/// purpose: a many-deck round-robin runs hundreds of full games.
pub const FIXED_ROLLOUTS: u32 = 4;
/// Steps before a fixed-policy rollout is cut off and scored heuristically.
pub const FIXED_ROLLOUT_CAP: u32 = 20;
/// Cap on candidate actions the fixed policy evaluates per decision.
pub const FIXED_MAX_CANDIDATES: usize = 8;

/// The single FIXED policy both seats play under:
/// [`ValueMcPolicy`]`(`[`MaterialValue`]`)` at a small budget. We hold the
/// policy constant because this harness measures DECKS, not policies (the
/// inverse of [`crate::search::round_robin`]).
pub fn fixed_policy(seed: u64) -> Box<dyn StatePolicy> {
    Box::new(ValueMcPolicy::with_budget(
        Box::new(MaterialValue),
        seed,
        FIXED_ROLLOUTS,
        FIXED_ROLLOUT_CAP,
        FIXED_MAX_CANDIDATES,
    ))
}

// =============================================================================
// Deck type
// =============================================================================

/// A named decklist (a flat multiset of card ids, MTG-style — repeats allowed).
#[derive(Clone, Debug)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<CardId>,
}

// =============================================================================
// Match primitive
// =============================================================================

/// Play one full game with `deck_a` as player 0 and `deck_b` as player 1 under
/// the fixed policy ([`fixed_policy`]) for BOTH seats, returning the engine's
/// [`GameResult`]. A `max_steps` cap (~4000) guarantees termination (a hit cap
/// yields [`GameResult::Draw`], per [`play_match`]). Both seats get the same
/// policy so the only thing that differs is the deck.
pub fn play_deck_match(
    deck_a: &[CardId],
    deck_b: &[CardId],
    registry: &CardRegistry,
    seed: u64,
    max_steps: u32,
) -> GameResult {
    play_deck_match_with(deck_a, deck_b, registry, seed, max_steps, &fixed_policy)
}

/// [`play_deck_match`] with a caller-supplied policy maker (used by the fast
/// unit tests, which swap in [`crate::search::RandomStatePolicy`]). `mk` builds
/// a fresh policy per seat, seeded off `seed` so the two seats differ.
pub fn play_deck_match_with(
    deck_a: &[CardId],
    deck_b: &[CardId],
    registry: &CardRegistry,
    seed: u64,
    max_steps: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> GameResult {
    let mut p0 = mk(seed.wrapping_mul(2).wrapping_add(1));
    let mut p1 = mk(seed.wrapping_mul(2).wrapping_add(2));
    let mut slots: Vec<&mut dyn StatePolicy> = vec![p0.as_mut(), p1.as_mut()];
    play_match(
        vec![deck_a.to_vec(), deck_b.to_vec()],
        registry,
        seed,
        &mut slots,
        max_steps,
    )
}

// =============================================================================
// Deck distribution
// =============================================================================

/// The basic land name for a WUBRG color letter (colorless → "Wastes").
fn basic_for(color: char) -> &'static str {
    match color.to_ascii_uppercase() {
        'W' => "Plains",
        'U' => "Island",
        'B' => "Swamp",
        'R' => "Mountain",
        'G' => "Forest",
        _ => "Wastes",
    }
}

/// Build a deterministic mono-color creature deck: `n_lands` basics of `color`
/// plus `n_spells` cheap creatures of `color`.
///
/// The creature pool is [`catalog::query`]'d — creatures of `color` with mana
/// value in `1..=cmc_max`, which `query` returns sorted by (mana value, name) —
/// then truncated to the `distinct` cheapest and CYCLED to fill the spell slots
/// (so each distinct creature appears ~equally, e.g. `distinct=11, n_spells=22`
/// gives ~2 copies each). Deterministic in the catalog contents. If `color` has
/// no creatures (never true for the real catalog) the deck is just lands.
pub fn mono_color_creature_deck(
    reg: &CardRegistry,
    color: char,
    distinct: usize,
    n_spells: usize,
    n_lands: usize,
    cmc_max: u32,
) -> Deck {
    let land_id = reg.card_id_by_name(basic_for(color));
    let pool: Vec<CardId> = catalog::query(
        reg,
        &CardQuery {
            colors: Some(vec![color]),
            types: Some(vec!["creature".into()]),
            cmc_min: Some(1),
            cmc_max: Some(cmc_max),
            limit: Some(distinct),
            ..Default::default()
        },
    )
    .into_iter()
    .map(|ci| ci.id)
    .collect();

    let mut cards = Vec::with_capacity(n_spells + n_lands);
    if !pool.is_empty() {
        for i in 0..n_spells {
            cards.push(pool[i % pool.len()]);
        }
    }
    if let Some(l) = land_id {
        for _ in 0..n_lands {
            cards.push(l);
        }
    }
    Deck {
        name: format!("mono-{}", color.to_ascii_uppercase()),
        cards,
    }
}

/// The fixed, documented deck distribution this harness ranks. Seven ~40-card
/// decks, all deterministic in the catalog:
///
/// * `sample-7`, `sample-13` — two [`arcana_cards::sample_deck`] piles (18 random
///   basics + 22 random catalog cards; the random-game harness deck shape).
/// * `mono-W/U/B/R/G` — one [`mono_color_creature_deck`] per color (22 cheap
///   creatures of the color = ~2 copies of 11 distinct + 18 basics).
pub fn standard_deck_set(reg: &CardRegistry) -> Vec<Deck> {
    let mut decks = vec![
        Deck {
            name: "sample-7".into(),
            cards: arcana_cards::sample_deck(reg, 7),
        },
        Deck {
            name: "sample-13".into(),
            cards: arcana_cards::sample_deck(reg, 13),
        },
    ];
    for color in ['W', 'U', 'B', 'R', 'G'] {
        decks.push(mono_color_creature_deck(reg, color, 11, 22, 18, 4));
    }
    decks
}

// =============================================================================
// Deck ranking (round-robin)
// =============================================================================

/// Result of a [`rank_decks`] deck round-robin: a win matrix plus per-deck
/// totals. `wins[i][j]` = games deck `i` won against deck `j`; `draws` is
/// symmetric; `played[i][j]` = games the pair played. Diagonals are 0.
pub struct DeckRanking {
    pub names: Vec<String>,
    pub wins: Vec<Vec<u32>>,
    pub draws: Vec<Vec<u32>>,
    pub played: Vec<Vec<u32>>,
    /// Total wins per deck across all opponents.
    pub total_wins: Vec<u32>,
    /// Total games per deck across all opponents.
    pub total_games: Vec<u32>,
}

impl DeckRanking {
    /// Win-rate of deck `i` in `[0, 1]` (0 if it played no games).
    pub fn win_pct(&self, i: usize) -> f32 {
        if self.total_games[i] == 0 {
            0.0
        } else {
            self.total_wins[i] as f32 / self.total_games[i] as f32
        }
    }

    /// Deck indices sorted by win-rate descending (ties broken by total wins).
    pub fn ranking(&self) -> Vec<usize> {
        let mut idx: Vec<usize> = (0..self.names.len()).collect();
        idx.sort_by(|&a, &b| {
            self.win_pct(b)
                .partial_cmp(&self.win_pct(a))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(self.total_wins[b].cmp(&self.total_wins[a]))
        });
        idx
    }

    /// A human-readable win matrix + win-rate column + ranking line.
    pub fn format_table(&self) -> String {
        let mut s = String::new();
        let w = self.names.iter().map(|n| n.len()).max().unwrap_or(6).max(6);
        // Header.
        s.push_str(&format!("{:>w$} |", "", w = w));
        for n in &self.names {
            s.push_str(&format!(" {:>9}", n));
        }
        s.push_str("  |   wins  games    win%\n");
        // Rows.
        for i in 0..self.names.len() {
            s.push_str(&format!("{:>w$} |", self.names[i], w = w));
            for j in 0..self.names.len() {
                if i == j {
                    s.push_str(&format!(" {:>9}", "—"));
                } else {
                    s.push_str(&format!(" {:>9}", self.wins[i][j]));
                }
            }
            s.push_str(&format!(
                "  | {:>6} {:>6} {:>6.1}%\n",
                self.total_wins[i],
                self.total_games[i],
                self.win_pct(i) * 100.0
            ));
        }
        // Ranking line.
        s.push_str("ranking: ");
        let rank: Vec<String> = self
            .ranking()
            .iter()
            .map(|&i| format!("{}({:.1}%)", self.names[i], self.win_pct(i) * 100.0))
            .collect();
        s.push_str(&rank.join(" > "));
        s
    }
}

/// Round-robin the deck set under the FIXED policy ([`fixed_policy`]): every
/// unordered pair plays `games_per_pair` games via [`win_rate`] (which
/// alternates seats to cancel first-player bias), tallied into a
/// [`DeckRanking`].
pub fn rank_decks(
    decks: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
) -> DeckRanking {
    rank_decks_with(decks, registry, games_per_pair, max_steps, &fixed_policy)
}

/// [`rank_decks`] with a caller-supplied policy maker (used by the fast unit
/// tests). Both seats use `mk`.
pub fn rank_decks_with(
    decks: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> DeckRanking {
    let n = decks.len();
    let mut wins = vec![vec![0u32; n]; n];
    let mut draws = vec![vec![0u32; n]; n];
    let mut played = vec![vec![0u32; n]; n];
    let mut total_wins = vec![0u32; n];
    let mut total_games = vec![0u32; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (iw, jw, d) = win_rate(
                &decks[i].cards,
                &decks[j].cards,
                registry,
                games_per_pair,
                max_steps,
                mk,
                mk,
            );
            wins[i][j] = iw;
            wins[j][i] = jw;
            draws[i][j] = d;
            draws[j][i] = d;
            played[i][j] = games_per_pair;
            played[j][i] = games_per_pair;
            total_wins[i] += iw;
            total_wins[j] += jw;
            total_games[i] += games_per_pair;
            total_games[j] += games_per_pair;
        }
    }
    DeckRanking {
        names: decks.iter().map(|d| d.name.clone()).collect(),
        wins,
        draws,
        played,
        total_wins,
        total_games,
    }
}

// =============================================================================
// Per-card contribution signal ("suggested cards" seed)
// =============================================================================

/// One card's contribution score from a ranked deck set.
#[derive(Clone, Debug)]
pub struct CardContribution {
    pub id: CardId,
    pub name: String,
    /// Mean win-rate (in `[0, 1]`) over the DISTINCT decks that contain this
    /// card. See [`card_contributions`] for the confound.
    pub score: f32,
    /// Number of decks in the set that contain at least one copy.
    pub n_decks: u32,
    /// Total copies across all decks (context only — it does NOT weight the
    /// score; a card in one strong deck and a card in one weak deck score by
    /// those decks' win-rates regardless of copy count).
    pub n_copies: u32,
}

/// A SIMPLE, HONEST first-cut per-card power signal: for each distinct card, the
/// average win-rate of the decks that contain it (co-occurrence weighting). The
/// result is sorted by score descending.
///
/// # Confound (read this)
///
/// This is a CO-OCCURRENCE statistic, NOT an isolated card-power measurement. A
/// card's score reflects the decks it HAPPENS to be in, not its marginal
/// contribution:
///
/// * A mediocre card that only ever appears in a strong deck inherits that
///   deck's high win-rate.
/// * A card spread across many decks (e.g. a basic land in several mono-color
///   decks) gets its score averaged toward the field mean, washing out signal.
/// * Because every card in a deck shares that deck's single win-rate, cards
///   within the same deck are indistinguishable from each other here.
///
/// An honest marginal estimate (swap card X for a baseline and re-measure the
/// win-rate delta, or regress win-rate on card presence across many decks) is a
/// strictly larger experiment; this function is the cheap co-occurrence seed it
/// would build on.
pub fn card_contributions(
    decks: &[Deck],
    ranking: &DeckRanking,
    reg: &CardRegistry,
) -> Vec<CardContribution> {
    // Per card: (sum of containing-deck win-rates over DISTINCT decks, #decks, #copies).
    let mut agg: HashMap<CardId, (f32, u32, u32)> = HashMap::new();
    for (di, deck) in decks.iter().enumerate() {
        let wr = ranking.win_pct(di);
        // Collapse to distinct cards + copy counts so each deck contributes its
        // win-rate exactly once to the mean (the co-occurrence weighting).
        let mut copies: HashMap<CardId, u32> = HashMap::new();
        for &c in &deck.cards {
            *copies.entry(c).or_default() += 1;
        }
        for (c, cnt) in copies {
            let e = agg.entry(c).or_insert((0.0, 0, 0));
            e.0 += wr;
            e.1 += 1;
            e.2 += cnt;
        }
    }
    let mut out: Vec<CardContribution> = agg
        .into_iter()
        .map(|(id, (sum, nd, nc))| {
            let name = catalog::card_info(reg, id)
                .map(|ci| ci.name)
                .unwrap_or_else(|| format!("#{id}"));
            CardContribution {
                id,
                name,
                score: sum / nd.max(1) as f32,
                n_decks: nd,
                n_copies: nc,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.n_decks.cmp(&a.n_decks))
            .then(a.name.cmp(&b.name))
    });
    out
}

// =============================================================================
// MARGINAL (causal) per-card power — the "real suggested cards" foundation
// =============================================================================
//
// `card_contributions` above is a co-occurrence statistic: a card inherits the
// win-rate of whatever decks it happens to be in, so cards inside one deck are
// indistinguishable and a card's number reflects its company, not itself. The
// functions below are the experiment its confound note calls for — an A/B swap.
//
// THE METHOD. Fix a `baseline` deck, a `filler` slot in it (a card we treat as
// near-neutral, e.g. a basic land), and a `gauntlet` of opponents. Build
// `deck_with` = baseline with ONE copy of `filler` replaced by the card under
// test, keeping the deck size constant. Measure both decks' win-rate against the
// whole gauntlet under the SAME referee policy, and report
// `win_rate(deck_with) − win_rate(baseline)`. Because exactly one slot changed,
// the delta is attributable to that one card: it is a marginal / causal estimate
// of "what does adding this card (in place of the filler) do to my win-rate?".
//
// HONEST LIMITATIONS (this is inherent to marginal measurement, not a bug):
//   * BASELINE-RELATIVE. The number answers "how much does this card help THIS
//     deck?", not "how good is this card in the abstract". A red bomb measured
//     against a mono-green baseline (whose Forests can't cast it) will look
//     terrible — correctly, for that baseline. Pick a baseline whose mana/curve
//     can actually support the candidates you compare, or compare candidates
//     that share the baseline's colors.
//   * FILLER-RELATIVE. The delta is "card MINUS filler". A basic-land filler
//     also means the swap removes one land, so the measured effect bundles the
//     card's value with losing a mana source. That is a legitimate question
//     ("is this card worth a land slot?") but it is a different question from a
//     spell-for-spell swap; choose the filler to match the question you mean.
//   * GAUNTLET- and REFEREE-RELATIVE. Same caveats as the rest of this harness
//     ([`fixed_policy`] is a small-budget ValueMc(Material) bot): a card that
//     shines only against control, or only under a stronger pilot, won't show
//     here. The gauntlet and policy define the metagame you are measuring in.
//   * VARIANCE + COST. Win-rate is noisy; small deltas are noise. This is the
//     expensive measurement (O(candidates × gauntlet × games_per_pair) FULL
//     games), so keep `games_per_pair` and the gauntlet sized to your budget and
//     read only sizeable, repeated deltas as signal.

/// Default neutral filler slot for marginal measurement: a basic land. Swapping
/// the card under test in for a land asks "is this card worth a land slot in the
/// baseline?" — see the FILLER-RELATIVE caveat on this module.
pub const DEFAULT_FILLER: &str = "Forest";

/// A sensible default baseline for marginal measurement: the mono-green creature
/// deck from [`mono_color_creature_deck`] (22 cheap green creatures + 18
/// Forests). Forest — the [`DEFAULT_FILLER`] — is abundant in it, so the
/// one-slot swap is always well-defined. Caveat: green/low-cost candidates are
/// measured fairly here; off-color or expensive candidates are measured against
/// a mana base that can't support them (the BASELINE-RELATIVE caveat).
pub fn default_marginal_baseline(reg: &CardRegistry) -> Deck {
    mono_color_creature_deck(reg, 'G', 11, 22, 18, 4)
}

/// One card's marginal (causal) power score from a swap experiment: the win-rate
/// delta `deck_with − baseline` against the gauntlet. See the module docs for
/// the method and its baseline/filler/gauntlet/referee caveats.
#[derive(Clone, Debug)]
pub struct CardMarginal {
    pub id: CardId,
    pub name: String,
    /// Win-rate delta in `[-1, 1]`: the gauntlet win-rate of the baseline with
    /// one `filler` slot replaced by this card, MINUS the baseline's own
    /// gauntlet win-rate. Positive = the card outperformed the filler slot.
    pub delta: f32,
}

/// Build a copy of `baseline` with ONE copy of `filler` replaced by `card`,
/// keeping the deck size constant. If no copy of `filler` is present, the LAST
/// card is replaced instead (documented fallback); an empty baseline is returned
/// unchanged (nothing to swap). Swapping a card in for itself (`card == filler`)
/// returns an unchanged decklist — the natural zero point.
pub fn swap_one(baseline: &[CardId], filler: CardId, card: CardId) -> Vec<CardId> {
    let mut deck = baseline.to_vec();
    if let Some(pos) = deck.iter().position(|&c| c == filler) {
        deck[pos] = card;
    } else if let Some(last) = deck.last_mut() {
        *last = card;
    }
    deck
}

/// Pooled win-rate (in `[0, 1]`) of `deck` against an entire `gauntlet`: every
/// opponent plays `games_per_pair` games via [`win_rate`] (which alternates
/// seats to cancel first-player bias, both seats using `mk`), and ALL games are
/// pooled into one rate (so each game weighs equally and draws count as
/// non-wins, matching [`DeckRanking::win_pct`]). An empty gauntlet yields `0.0`.
fn deck_win_rate_vs_gauntlet(
    deck: &[CardId],
    gauntlet: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> f32 {
    let mut wins = 0u32;
    let mut games = 0u32;
    for opp in gauntlet {
        let (w, _opp_wins, _draws) =
            win_rate(deck, &opp.cards, registry, games_per_pair, max_steps, mk, mk);
        wins += w;
        games += games_per_pair;
    }
    if games == 0 {
        0.0
    } else {
        wins as f32 / games as f32
    }
}

/// Marginal (causal) power of a single `card` in a given `baseline`, under the
/// FIXED policy ([`fixed_policy`]): see [`marginal_card_power_with`] and the
/// module docs. Measures TWO full gauntlet sweeps (baseline + deck_with); prefer
/// [`rank_cards_marginal`] when scoring several cards against one baseline, as it
/// measures the shared baseline arm only once.
pub fn marginal_card_power(
    card: CardId,
    baseline: &[CardId],
    filler: CardId,
    gauntlet: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
) -> f32 {
    marginal_card_power_with(
        card,
        baseline,
        filler,
        gauntlet,
        registry,
        games_per_pair,
        max_steps,
        &fixed_policy,
    )
}

/// [`marginal_card_power`] with a caller-supplied policy maker (used by the fast
/// unit tests, which swap in [`crate::search::RandomStatePolicy`]). Builds
/// `deck_with` = `baseline` with one `filler` slot replaced by `card`
/// ([`swap_one`]), then returns its gauntlet win-rate MINUS the baseline's
/// gauntlet win-rate. The result is the card's marginal contribution IN THIS
/// CONTEXT — see the module docs for the baseline/filler/gauntlet/referee
/// caveats and the variance/cost note.
#[allow(clippy::too_many_arguments)]
pub fn marginal_card_power_with(
    card: CardId,
    baseline: &[CardId],
    filler: CardId,
    gauntlet: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> f32 {
    let deck_with = swap_one(baseline, filler, card);
    let with =
        deck_win_rate_vs_gauntlet(&deck_with, gauntlet, registry, games_per_pair, max_steps, mk);
    let base =
        deck_win_rate_vs_gauntlet(baseline, gauntlet, registry, games_per_pair, max_steps, mk);
    with - base
}

/// Rank a slate of `candidates` by marginal power against one `baseline`/`filler`
/// /`gauntlet` under the FIXED policy ([`fixed_policy`]). Sorted by delta
/// descending. See [`rank_cards_marginal_with`] and the module docs.
pub fn rank_cards_marginal(
    candidates: &[CardId],
    baseline: &[CardId],
    filler: CardId,
    gauntlet: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
) -> Vec<CardMarginal> {
    rank_cards_marginal_with(
        candidates,
        baseline,
        filler,
        gauntlet,
        registry,
        games_per_pair,
        max_steps,
        &fixed_policy,
    )
}

/// [`rank_cards_marginal`] with a caller-supplied policy maker (used by the fast
/// unit tests). For each candidate, swaps it into the `filler` slot and measures
/// the deck's gauntlet win-rate, then subtracts the baseline's gauntlet win-rate
/// to get the marginal delta. The result is sorted by delta descending (ties
/// broken by name), so the front of the list is the "most suggested" card FOR
/// THIS BASELINE (see the module caveats).
///
/// EFFICIENCY: the baseline arm is the SAME for every candidate, so it is
/// measured exactly ONCE and shared. The per-candidate cost is then a single
/// `deck_with` gauntlet sweep — i.e. one + `candidates.len()` sweeps total, not
/// two per candidate.
#[allow(clippy::too_many_arguments)]
pub fn rank_cards_marginal_with(
    candidates: &[CardId],
    baseline: &[CardId],
    filler: CardId,
    gauntlet: &[Deck],
    registry: &CardRegistry,
    games_per_pair: u32,
    max_steps: u32,
    mk: &dyn Fn(u64) -> Box<dyn StatePolicy>,
) -> Vec<CardMarginal> {
    // The control arm (baseline vs gauntlet) is candidate-independent — measure
    // it once and reuse it for every delta.
    let base =
        deck_win_rate_vs_gauntlet(baseline, gauntlet, registry, games_per_pair, max_steps, mk);
    let mut out: Vec<CardMarginal> = candidates
        .iter()
        .map(|&card| {
            let deck_with = swap_one(baseline, filler, card);
            let with = deck_win_rate_vs_gauntlet(
                &deck_with,
                gauntlet,
                registry,
                games_per_pair,
                max_steps,
                mk,
            );
            let name = catalog::card_info(registry, card)
                .map(|ci| ci.name)
                .unwrap_or_else(|| format!("#{card}"));
            CardMarginal {
                id: card,
                name,
                delta: with - base,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.delta
            .partial_cmp(&a.delta)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a.name.cmp(&b.name))
    });
    out
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::RandomStatePolicy;

    /// Cheap policy maker for the fast tests.
    fn rnd(s: u64) -> Box<dyn StatePolicy> {
        Box::new(RandomStatePolicy::new(s))
    }

    /// Three tiny decks (small + same colors so they share basics, which lets
    /// the contribution test exercise multi-deck co-occurrence). Fast to play.
    fn tiny_decks(reg: &CardRegistry) -> Vec<Deck> {
        vec![
            mono_color_creature_deck(reg, 'R', 4, 8, 10, 4),
            mono_color_creature_deck(reg, 'G', 4, 8, 10, 4),
            mono_color_creature_deck(reg, 'R', 6, 8, 10, 5), // shares Mountain w/ deck 0
        ]
    }

    #[test]
    fn deck_builders_are_well_formed() {
        let reg = arcana_cards::build_catalog();
        let deck = mono_color_creature_deck(&reg, 'R', 11, 22, 18, 4);
        // 22 spells + 18 lands.
        assert_eq!(deck.cards.len(), 40);
        assert_eq!(deck.name, "mono-R");
        // Mountains are present.
        let mtn = reg.card_id_by_name("Mountain").expect("Mountain in catalog");
        assert_eq!(deck.cards.iter().filter(|&&c| c == mtn).count(), 18);

        let set = standard_deck_set(&reg);
        assert_eq!(set.len(), 7);
        for d in &set {
            assert!(d.cards.len() >= 20, "{} too small: {}", d.name, d.cards.len());
        }
    }

    /// The match primitive runs to a terminal result without hanging/panicking.
    /// Uses the cheap random policy maker for speed; structural assertion only
    /// (no winner assertion — high variance).
    #[test]
    fn deck_match_runs_to_a_result() {
        let reg = arcana_cards::build_catalog();
        let decks = tiny_decks(&reg);
        let res = play_deck_match_with(
            &decks[0].cards,
            &decks[1].cards,
            &reg,
            42,
            4000,
            &rnd,
        );
        match res {
            GameResult::Win(p) => assert!(p == 0 || p == 1),
            GameResult::Draw | GameResult::Eliminated(_) => {}
        }
    }

    /// The deck round-robin over a tiny set produces a coherent table:
    /// square matrix, every deck present, totals add up, each pair's games are
    /// fully accounted for. No winner assertion (variance).
    #[test]
    fn rank_decks_tallies_correctly() {
        let reg = arcana_cards::build_catalog();
        let decks = tiny_decks(&reg);
        let g = 2u32;
        let rr = rank_decks_with(&decks, &reg, g, 4000, &rnd);

        let n = decks.len();
        assert_eq!(rr.names.len(), n);
        for (i, d) in decks.iter().enumerate() {
            assert_eq!(rr.names[i], d.name); // every deck appears, in order
            assert_eq!(rr.wins[i].len(), n); // square matrix
        }
        for i in 0..n {
            // Each deck plays (n-1) opponents * g games.
            assert_eq!(rr.total_games[i], (n as u32 - 1) * g);
            // Row of wins sums to the deck's total wins.
            let row: u32 = rr.wins[i].iter().sum();
            assert_eq!(row, rr.total_wins[i]);
            // win_pct in range.
            let p = rr.win_pct(i);
            assert!((0.0..=1.0).contains(&p));
        }
        // Each unordered pair's games are fully accounted for (wins + draws).
        for i in 0..n {
            for j in (i + 1)..n {
                assert_eq!(rr.wins[i][j] + rr.wins[j][i] + rr.draws[i][j], g);
                assert_eq!(rr.played[i][j], g);
            }
        }
        assert_eq!(rr.ranking().len(), n);
        let table = rr.format_table();
        assert!(table.contains("ranking:"));
        assert!(table.contains("win%"));
    }

    /// The contribution signal is coherent: non-empty, scores in `[0,1]` and
    /// sorted descending, every card seen in >=1 deck, copy counts positive,
    /// and a card's score equals the mean win-rate of its decks (the documented
    /// co-occurrence definition), checked on a shared card.
    #[test]
    fn card_contributions_are_coherent() {
        let reg = arcana_cards::build_catalog();
        let decks = tiny_decks(&reg);
        let rr = rank_decks_with(&decks, &reg, 2, 4000, &rnd);
        let contrib = card_contributions(&decks, &rr, &reg);

        assert!(!contrib.is_empty());
        for w in contrib.windows(2) {
            assert!(w[0].score >= w[1].score, "not sorted descending");
        }
        for c in &contrib {
            assert!((0.0..=1.0).contains(&c.score));
            assert!(c.n_decks >= 1);
            assert!(c.n_copies >= 1);
        }

        // Mountain appears in decks 0 and 2 (both red); its score must equal the
        // mean of those two decks' win-rates — the co-occurrence definition.
        let mtn = reg.card_id_by_name("Mountain").unwrap();
        let m = contrib.iter().find(|c| c.id == mtn).expect("Mountain present");
        assert_eq!(m.n_decks, 2);
        let expected = (rr.win_pct(0) + rr.win_pct(2)) / 2.0;
        assert!((m.score - expected).abs() < 1e-6);
    }

    // -------------------------------------------------------------------------
    // Marginal (causal) per-card power
    // -------------------------------------------------------------------------

    /// A couple of green creatures from the catalog to use as marginal-test
    /// candidates (deterministic: `query` returns sorted by mana value, name).
    fn green_creature_candidates(reg: &CardRegistry, n: usize) -> Vec<CardId> {
        catalog::query(
            reg,
            &CardQuery {
                colors: Some(vec!['G']),
                types: Some(vec!["creature".into()]),
                cmc_min: Some(1),
                cmc_max: Some(4),
                limit: Some(n),
                ..Default::default()
            },
        )
        .into_iter()
        .map(|ci| ci.id)
        .collect()
    }

    /// `swap_one` keeps the deck size constant, replaces exactly one filler copy
    /// with the card, falls back to the last slot when the filler is absent, and
    /// is a no-op when swapping a card in for itself.
    #[test]
    fn swap_one_keeps_size_and_swaps_one_slot() {
        let baseline: Vec<CardId> = vec![10, 20, 20, 30];
        // Filler present: exactly one copy (the first) becomes the card.
        let d = swap_one(&baseline, 20, 99);
        assert_eq!(d.len(), baseline.len(), "deck size must stay constant");
        assert_eq!(d, vec![10, 99, 20, 30]);
        // Filler absent: the LAST slot is replaced instead.
        let d2 = swap_one(&baseline, 77, 99);
        assert_eq!(d2.len(), baseline.len());
        assert_eq!(d2, vec![10, 20, 20, 99]);
        // Card == filler: unchanged (the natural zero point).
        let d3 = swap_one(&baseline, 20, 20);
        assert_eq!(d3, baseline);
        // Empty baseline: nothing to swap.
        assert!(swap_one(&[], 1, 2).is_empty());
    }

    /// `marginal_card_power_with` runs under the cheap random policy and returns
    /// a finite delta in `[-1, 1]`; deck size is preserved by the swap. No
    /// win-rate value asserted (high variance).
    #[test]
    fn marginal_power_runs_and_is_finite() {
        let reg = arcana_cards::build_catalog();
        let baseline = mono_color_creature_deck(&reg, 'G', 4, 8, 10, 4); // 18 cards
        let gauntlet = vec![mono_color_creature_deck(&reg, 'R', 4, 8, 10, 4)];
        let filler = reg.card_id_by_name("Forest").expect("Forest in catalog");
        let card = green_creature_candidates(&reg, 1)[0];

        // The swap preserves deck size (the property marginal measurement needs).
        assert_eq!(swap_one(&baseline.cards, filler, card).len(), baseline.cards.len());

        let delta = marginal_card_power_with(
            card,
            &baseline.cards,
            filler,
            &gauntlet,
            &reg,
            2,
            4000,
            &rnd,
        );
        assert!(delta.is_finite(), "delta must be finite");
        assert!((-1.0..=1.0).contains(&delta), "delta {delta} out of range");
    }

    /// Swapping the FILLER in for itself is a deterministic exact zero: the
    /// `deck_with` decklist equals the baseline and `win_rate` is deterministic
    /// in its seeds, so both gauntlet arms produce identical results. (This pins
    /// the "natural zero point" of the marginal scale.)
    #[test]
    fn marginal_of_filler_itself_is_exactly_zero() {
        let reg = arcana_cards::build_catalog();
        let baseline = mono_color_creature_deck(&reg, 'G', 4, 8, 10, 4);
        let gauntlet = vec![mono_color_creature_deck(&reg, 'R', 4, 8, 10, 4)];
        let filler = reg.card_id_by_name("Forest").unwrap();
        let delta = marginal_card_power_with(
            filler,
            &baseline.cards,
            filler,
            &gauntlet,
            &reg,
            2,
            4000,
            &rnd,
        );
        assert_eq!(delta, 0.0, "swapping filler for itself must be exactly 0");
    }

    /// `rank_cards_marginal_with` returns one entry per candidate, sorted by
    /// delta descending, with finite deltas and resolved names. No specific
    /// ordering of real cards asserted (variance).
    #[test]
    fn rank_cards_marginal_is_sorted_and_complete() {
        let reg = arcana_cards::build_catalog();
        let baseline = mono_color_creature_deck(&reg, 'G', 4, 8, 10, 4);
        let gauntlet = vec![mono_color_creature_deck(&reg, 'R', 4, 8, 10, 4)];
        let filler = reg.card_id_by_name("Forest").unwrap();
        // Candidates: two green creatures + the filler itself (its delta is 0).
        let mut candidates = green_creature_candidates(&reg, 2);
        candidates.push(filler);

        let ranked = rank_cards_marginal_with(
            &candidates,
            &baseline.cards,
            filler,
            &gauntlet,
            &reg,
            2,
            4000,
            &rnd,
        );
        assert_eq!(ranked.len(), candidates.len(), "one entry per candidate");
        for w in ranked.windows(2) {
            assert!(w[0].delta >= w[1].delta, "not sorted descending");
        }
        for m in &ranked {
            assert!(m.delta.is_finite());
            assert!(candidates.contains(&m.id));
            assert!(!m.name.is_empty());
        }
        // The filler-vs-itself candidate must score an exact 0 (deterministic).
        let f = ranked.iter().find(|m| m.id == filler).unwrap();
        assert_eq!(f.delta, 0.0);
    }

    /// MEASUREMENT (non-asserting): rank the standard deck set under the FIXED
    /// ValueMc(Material) policy and print the deck ranking + the top/bottom
    /// cards by contribution. Slow (hundreds of full games with value-MC on
    /// both seats); `#[ignore]`, run in release:
    ///   cargo test -p arcana-ai --release --lib \
    ///     deckeval::tests::deck_power_ranking_measurement -- --ignored --nocapture
    #[test]
    #[ignore]
    fn deck_power_ranking_measurement() {
        use std::time::Instant;
        const GAMES_PER_PAIR: u32 = 6;

        let reg = arcana_cards::build_catalog();
        let decks = standard_deck_set(&reg);

        let t0 = Instant::now();
        let ranking = rank_decks(&decks, &reg, GAMES_PER_PAIR, 4000);
        let elapsed = t0.elapsed();

        println!(
            "Deck power ranking — fixed ValueMc(Material) budget \
             [rollouts={FIXED_ROLLOUTS} cap={FIXED_ROLLOUT_CAP} \
             cand={FIXED_MAX_CANDIDATES}], {GAMES_PER_PAIR} games/pair, \
             {} decks, {:.1}s:",
            decks.len(),
            elapsed.as_secs_f32()
        );
        println!("{}", ranking.format_table());

        let contrib = card_contributions(&decks, &ranking, &reg);
        println!(
            "\nPer-card contribution = mean win-rate of the decks containing the \
             card (CO-OCCURRENCE, not isolated power — see docs).\nTop 15:"
        );
        for c in contrib.iter().take(15) {
            println!(
                "  {:>6.1}%  x{:<3} in {} deck(s)  {}",
                c.score * 100.0,
                c.n_copies,
                c.n_decks,
                c.name
            );
        }
        println!("Bottom 10:");
        for c in contrib.iter().rev().take(10) {
            println!(
                "  {:>6.1}%  x{:<3} in {} deck(s)  {}",
                c.score * 100.0,
                c.n_copies,
                c.n_decks,
                c.name
            );
        }
    }

    /// MEASUREMENT (non-asserting): rank a handful of REAL catalog cards by
    /// MARGINAL (causal) power under the FIXED ValueMc(Material) policy — the
    /// honest answer the `card_contributions` confound note asks for. Each card
    /// is swapped into one Forest slot of the mono-green baseline and the deck is
    /// re-measured against the gauntlet; the printed delta is win-rate(with) −
    /// win-rate(baseline). Expect a strong green creature to land above a weak
    /// one, and a Forest (the filler) to land near 0.0 (the control). Slow (each
    /// candidate is a full gauntlet sweep with value-MC on both seats);
    /// `#[ignore]`, run in release:
    ///   cargo test -p arcana-ai --release --lib \
    ///     deckeval::tests::marginal_card_power_measurement -- --ignored --nocapture
    #[test]
    #[ignore]
    fn marginal_card_power_measurement() {
        use std::time::Instant;
        const GAMES_PER_PAIR: u32 = 6;

        let reg = arcana_cards::build_catalog();
        let baseline = default_marginal_baseline(&reg);
        let filler = reg.card_id_by_name(DEFAULT_FILLER).expect("filler in catalog");
        // A small, varied gauntlet (a couple off-color creature decks).
        let gauntlet = vec![
            mono_color_creature_deck(&reg, 'R', 11, 22, 18, 4),
            mono_color_creature_deck(&reg, 'U', 11, 22, 18, 4),
        ];

        // Candidate slate: a strong-ish green beater, a weak green creature, and
        // the filler itself (the 0.0 control). Resolved by name with a graceful
        // fallback so the test stays robust to catalog churn; unknown names are
        // skipped. Tune these names to whatever the catalog actually carries.
        let wanted = [
            "Tarmogoyf",
            "Llanowar Elves",
            "Grizzly Bears",
            "Craw Wurm",
            DEFAULT_FILLER, // control: should score ~0
        ];
        let mut candidates: Vec<CardId> = wanted
            .iter()
            .filter_map(|n| reg.card_id_by_name(n))
            .collect();
        // Top up from the green creature pool if too few names resolved, so the
        // measurement always has something to rank.
        if candidates.len() < 3 {
            let pool: Vec<CardId> = catalog::query(
                &reg,
                &CardQuery {
                    colors: Some(vec!['G']),
                    types: Some(vec!["creature".into()]),
                    cmc_min: Some(1),
                    cmc_max: Some(6),
                    limit: Some(5),
                    ..Default::default()
                },
            )
            .into_iter()
            .map(|ci| ci.id)
            .collect();
            for id in pool {
                if !candidates.contains(&id) {
                    candidates.push(id);
                }
            }
            if !candidates.contains(&filler) {
                candidates.push(filler);
            }
        }

        let t0 = Instant::now();
        let ranked = rank_cards_marginal(
            &candidates,
            &baseline.cards,
            filler,
            &gauntlet,
            &reg,
            GAMES_PER_PAIR,
            4000,
        );
        let elapsed = t0.elapsed();

        println!(
            "Marginal card power — swap ONE '{}' slot in baseline '{}' \
             (size {}), re-measure vs {}-deck gauntlet, fixed ValueMc(Material) \
             [rollouts={FIXED_ROLLOUTS} cap={FIXED_ROLLOUT_CAP} \
             cand={FIXED_MAX_CANDIDATES}], {GAMES_PER_PAIR} games/pair, {:.1}s.",
            DEFAULT_FILLER,
            baseline.name,
            baseline.cards.len(),
            gauntlet.len(),
            elapsed.as_secs_f32()
        );
        println!(
            "delta = win-rate(baseline w/ card) − win-rate(baseline). \
             CAUSAL but baseline/filler/gauntlet/referee-RELATIVE — see docs.\n\
             rank   delta   card"
        );
        for (i, m) in ranked.iter().enumerate() {
            println!("  {:>2}  {:>+6.1}%   {}", i + 1, m.delta * 100.0, m.name);
        }
    }
}

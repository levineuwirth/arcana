//! Observation encoding (feature vectors for neural networks).
//!
//! Implements the E2 strategy from spec §14.2 — fixed-length
//! feature-based encoding, designed to generalize across the full
//! card pool without per-card embeddings or one-hots. Card-specific
//! features (per-name presence, per-set distribution, learned card
//! embeddings) are deliberately deferred until the registry has
//! stabilized; v0 reads only structural signals available on every
//! [`GameState`] today.
//!
//! # Trait shape — non-negotiable design points
//!
//! * **`encode_into(&mut [f32])` is the primary method.** The
//!   convenience [`Encoder::encode`] is a default impl that
//!   allocates a fresh `Vec`. Hot-loop callsites (MCTS rollouts,
//!   policy forward passes, replay-buffer fills) must not allocate
//!   per call — they hit `encode_into` against a pre-sized buffer
//!   (NumPy / `tch::Tensor` storage, etc.).
//! * **`perspective: Option<PlayerId>` is mandatory from day 1.**
//!   AlphaZero-style self-play encodes from the active player's
//!   perspective so a single network plays both sides; retrofitting
//!   this later means re-training every checkpoint. `None` is the
//!   canonical (perspective-agnostic) view, useful for symmetric
//!   value heads and for offline debugging.
//! * **Dimension is exposed as a `pub const`** (see
//!   [`BASIC_E2_DIM_TWO_PLAYERS`]). Downstream code (PyO3 wrapper,
//!   training scripts) sizes buffers off this constant; silent
//!   drift would invalidate loaded checkpoints catastrophically, so
//!   it lives behind an explicit-update test ([`tests::dim_is_stable`]).
//!
//! # v0 feature budget
//!
//! Per-player block (50 floats) + game-level block (20 floats) +
//! perspective indicators (3 floats) = **123 floats for 2 players**.
//! The per-player block now includes identity-free *card-aware*
//! aggregates: an 8-bucket CMC histogram of controlled permanents and
//! per-keyword presence on controlled creatures. Spec §14.2 budgets
//! 1500-2500 for the *finished* E2 — the remaining gap closes when
//! true card-identity features land (embeddings / per-name signals);
//! the scaffolding here is what those features hook into.
//!
//! # Normalization
//!
//! All scalars must come back in roughly `[-1, 1]` for stable
//! neural-net training. Extreme cases (40-mana ritual chains,
//! milled-deck library = 0) must not blow past that range or
//! produce NaN. The two helpers below codify the normalization
//! strategy:
//!
//! * [`sat_tanh`] — saturating, for fields with a typical range and
//!   an extreme tail (life, mana pool size, total power, turn
//!   number).
//! * [`log_size`] — log-scale clamped to `[0, 1]`, for zone sizes
//!   where small-value resolution matters and the tail compresses
//!   gracefully.

use std::collections::HashMap;

use arcana_core::effects::KeywordAbility;
use arcana_core::objects::GameObject;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameState;
use arcana_core::turn::{Phase, Step};
use arcana_core::types::{CardId, Color, PlayerId, PtValue, SmallString};
use arcana_core::zones::Zone;

// =============================================================================
// Trait
// =============================================================================

/// One-shot encoder: `(GameState, perspective) → Vec<f32>` of fixed
/// length [`Self::dim`]. See module docs for design rationale on the
/// `encode_into` signature and the `perspective` parameter.
pub trait Encoder {
    /// Length of the output vector. Must be deterministic over the
    /// lifetime of an [`Encoder`] instance — downstream consumers
    /// pre-size buffers off this value.
    fn dim(&self) -> usize;

    /// Encode `state` into `buf` from the perspective of
    /// `perspective`. `Some(p)` reorders per-player feature blocks
    /// so the perspective player is first; `None` writes blocks in
    /// canonical player-id order and zeros the perspective
    /// indicators.
    ///
    /// # Panics
    /// Panics if `buf.len() < self.dim()`. Implementations write
    /// exactly `self.dim()` floats and leave anything past that
    /// untouched.
    fn encode_into(
        &self,
        state: &GameState,
        perspective: Option<PlayerId>,
        buf: &mut [f32],
    );

    /// Allocating convenience wrapper. Tests, debug tooling, and
    /// one-off use only — hot paths use `encode_into`.
    fn encode(&self, state: &GameState, perspective: Option<PlayerId>) -> Vec<f32> {
        let mut buf = vec![0.0; self.dim()];
        self.encode_into(state, perspective, &mut buf);
        buf
    }
}

// =============================================================================
// BasicE2Encoder
// =============================================================================

/// Per-player feature count. Update [`BASIC_E2_DIM_TWO_PLAYERS`]
/// constant whenever this changes — `tests::dim_is_stable` enforces
/// the explicit-update discipline.
const PER_PLAYER_FEATURES: usize = 50;
/// Game-level feature count (turn, phase, step, combat, stack,
/// storm).
const GAME_LEVEL_FEATURES: usize = 20;
/// Perspective indicators (am-I-active-player, do-I-have-priority,
/// game-over). Zero when perspective is `None`.
const PERSPECTIVE_FEATURES: usize = 3;

/// Total vector length emitted by [`BasicE2Encoder`] for a 2-player
/// game. Exposed as `pub const` so downstream tooling (PyO3 wrapper,
/// rollout buffers) can size at compile time.
///
/// Update with deliberate intent — silent drift breaks loaded
/// checkpoints. The matching test [`tests::dim_is_stable`] is the
/// gate.
pub const BASIC_E2_DIM_TWO_PLAYERS: usize =
    2 * PER_PLAYER_FEATURES + GAME_LEVEL_FEATURES + PERSPECTIVE_FEATURES;

/// E2 (feature-based) encoder, v0. Identity-free — reads only
/// structural aggregates every [`GameState`] exposes (counts, an
/// 8-bucket CMC histogram, per-keyword presence), never a specific
/// card name. Future revisions add true card-identity features
/// (embeddings / per-name signals) once the registry stabilizes.
#[derive(Debug, Clone)]
pub struct BasicE2Encoder {
    num_players: u8,
}

impl BasicE2Encoder {
    pub fn new(num_players: u8) -> Self {
        assert!(num_players >= 1, "at least one player required");
        Self { num_players }
    }

    /// Convenience for the default 2-player setup. Dim equals
    /// [`BASIC_E2_DIM_TWO_PLAYERS`].
    pub fn for_two_players() -> Self {
        Self::new(2)
    }
}

impl Default for BasicE2Encoder {
    fn default() -> Self {
        Self::for_two_players()
    }
}

impl Encoder for BasicE2Encoder {
    fn dim(&self) -> usize {
        PER_PLAYER_FEATURES * self.num_players as usize
            + GAME_LEVEL_FEATURES
            + PERSPECTIVE_FEATURES
    }

    fn encode_into(
        &self,
        state: &GameState,
        perspective: Option<PlayerId>,
        buf: &mut [f32],
    ) {
        let dim = self.dim();
        assert!(
            buf.len() >= dim,
            "buf too small: need {dim}, got {}",
            buf.len()
        );

        // Player ordering: perspective first, then remaining players
        // in canonical id order. None → canonical id order.
        let order = player_order(self.num_players, perspective);

        let mut cursor = 0;
        for &player_id in &order {
            encode_player_block(
                state,
                player_id,
                &mut buf[cursor..cursor + PER_PLAYER_FEATURES],
            );
            cursor += PER_PLAYER_FEATURES;
        }

        encode_game_level_block(state, &mut buf[cursor..cursor + GAME_LEVEL_FEATURES]);
        cursor += GAME_LEVEL_FEATURES;

        encode_perspective_block(
            state,
            perspective,
            &mut buf[cursor..cursor + PERSPECTIVE_FEATURES],
        );
        cursor += PERSPECTIVE_FEATURES;

        debug_assert_eq!(cursor, dim);
    }
}

// =============================================================================
// IdentityEncoder — card-identity features over a fixed vocabulary
// =============================================================================

/// Visible zones the identity block reads, from the perspective player's view.
/// Deliberately excludes opponent hand + all libraries (hidden information a
/// real player cannot see). Order is the per-vocab-card feature-block layout.
const ID_ZONES: usize = 5; // own hand, own bf, opp bf, own gy, opp gy

/// [`BasicE2Encoder`] plus **card-identity** features over a fixed vocabulary
/// (e.g. a capsule's card pool). It APPENDS, per vocabulary card, a count in
/// each of [`ID_ZONES`] visible zones (own hand, own battlefield, opponent
/// battlefield, own graveyard, opponent graveyard) — never opponent hand or any
/// library. This is the identity-vs-identity-free ablation lever: the base 123
/// features are board-shape aggregates that cannot see *which* cards are in play,
/// so a learned value over base-only cannot represent synergy; the appended
/// counts let it.
///
/// Layout: `[ base (base.dim()) | id (vocab_len * ID_ZONES) ]`, id indexed
/// `zone_slot * vocab_len + vocab_index`. Perspective-relative (own/opp resolved
/// from `perspective`, defaulting to player 0 when `None`).
#[derive(Debug, Clone)]
pub struct IdentityEncoder {
    base: BasicE2Encoder,
    /// Card-name (interned [`SmallString`]) → dense vocabulary index.
    index: HashMap<SmallString, usize>,
    vocab_len: usize,
}

impl IdentityEncoder {
    /// Build the vocabulary from a set of card ids (their definition names) —
    /// the capsule's legal pool. Duplicate ids collapse; unknown ids are skipped.
    pub fn from_card_ids(num_players: u8, ids: &[CardId], reg: &CardRegistry) -> Self {
        let mut index: HashMap<SmallString, usize> = HashMap::new();
        let mut vocab_len = 0;
        for &cid in ids {
            if let Some(def) = reg.get(cid) {
                if !index.contains_key(&def.name) {
                    index.insert(def.name, vocab_len);
                    vocab_len += 1;
                }
            }
        }
        Self { base: BasicE2Encoder::new(num_players), index, vocab_len }
    }

    /// Number of distinct cards in the identity vocabulary.
    pub fn vocab_len(&self) -> usize {
        self.vocab_len
    }

    /// Increment the count of every vocabulary card among `objects` at `slot`.
    fn tally<'a>(
        &self,
        objects: impl Iterator<Item = &'a GameObject>,
        slot: usize,
        id: &mut [f32],
    ) {
        for o in objects {
            if let Some(&idx) = self.index.get(&o.characteristics.name) {
                id[slot * self.vocab_len + idx] += 1.0;
            }
        }
    }
}

impl Encoder for IdentityEncoder {
    fn dim(&self) -> usize {
        self.base.dim() + self.vocab_len * ID_ZONES
    }

    fn encode_into(&self, state: &GameState, perspective: Option<PlayerId>, buf: &mut [f32]) {
        let dim = self.dim();
        assert!(buf.len() >= dim, "buf too small: need {dim}, got {}", buf.len());

        let base_dim = self.base.dim();
        self.base.encode_into(state, perspective, &mut buf[..base_dim]);

        let id = &mut buf[base_dim..dim];
        for x in id.iter_mut() {
            *x = 0.0;
        }
        if self.vocab_len == 0 {
            return;
        }

        let me = perspective.unwrap_or(0);
        let opp = state.opponents_of(me).next();

        // own hand (slot 0) — visible to the perspective player.
        self.tally(state.objects.objects_in_zone(Zone::Hand(me)), 0, id);
        // battlefields (slots 1 own / 2 opp) — public zone, controller-split.
        for o in state.objects.objects_in_zone(Zone::Battlefield) {
            let slot = if o.controller == me {
                1
            } else if Some(o.controller) == opp {
                2
            } else {
                continue;
            };
            if let Some(&idx) = self.index.get(&o.characteristics.name) {
                id[slot * self.vocab_len + idx] += 1.0;
            }
        }
        // graveyards (slots 3 own / 4 opp) — public zones.
        self.tally(state.objects.objects_in_zone(Zone::Graveyard(me)), 3, id);
        if let Some(opp) = opp {
            self.tally(state.objects.objects_in_zone(Zone::Graveyard(opp)), 4, id);
        }
    }
}

// =============================================================================
// Player block (PER_PLAYER_FEATURES floats)
// =============================================================================

fn encode_player_block(state: &GameState, player: PlayerId, buf: &mut [f32]) {
    debug_assert_eq!(buf.len(), PER_PLAYER_FEATURES);
    let p = state.player(player);
    let mut i = 0;

    // Scalars (7).
    buf[i] = sat_tanh(p.life as f32, 20.0);
    i += 1;
    buf[i] = sat_tanh(p.poison_counters as f32, 5.0);
    i += 1;
    buf[i] = sat_tanh(p.energy as f32, 5.0);
    i += 1;
    buf[i] = sat_tanh(p.experience as f32, 5.0);
    i += 1;
    buf[i] = sat_tanh(p.land_plays_remaining as f32, 2.0);
    i += 1;
    buf[i] = if p.has_lost { 1.0 } else { 0.0 };
    i += 1;
    buf[i] = if p.has_conceded { 1.0 } else { 0.0 };
    i += 1;

    // Mana pool total (1).
    buf[i] = sat_tanh(p.mana_pool.total() as f32, 5.0);
    i += 1;

    // Zone sizes (4): library, hand, graveyard, exile-owned.
    buf[i] = log_size(state.zone_count(Zone::Library(player)));
    i += 1;
    buf[i] = log_size(state.zone_count(Zone::Hand(player)));
    i += 1;
    buf[i] = log_size(state.zone_count(Zone::Graveyard(player)));
    i += 1;
    let exile_owned = state
        .objects
        .objects_in_zone(Zone::Exile)
        .filter(|o| o.owner == player)
        .count();
    buf[i] = log_size(exile_owned);
    i += 1;

    // Battlefield aggregates — controller-filtered. Materialize once
    // since we walk the same set six times. Cheap for v0; the
    // per-zone index in arcana-core (deferred follow-up #254) makes
    // this proportional to controller's permanent count when it
    // lands.
    let bf: Vec<&GameObject> = state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == player)
        .collect();

    // By type (5).
    buf[i] = log_size(bf.iter().filter(|o| o.is_creature()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_land()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_artifact()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_enchantment()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_planeswalker()).count());
    i += 1;

    // Tapped by type (5).
    buf[i] = log_size(bf.iter().filter(|o| o.is_creature() && o.is_tapped()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_land() && o.is_tapped()).count());
    i += 1;
    buf[i] = log_size(bf.iter().filter(|o| o.is_artifact() && o.is_tapped()).count());
    i += 1;
    buf[i] = log_size(
        bf.iter()
            .filter(|o| o.is_enchantment() && o.is_tapped())
            .count(),
    );
    i += 1;
    buf[i] = log_size(
        bf.iter()
            .filter(|o| o.is_planeswalker() && o.is_tapped())
            .count(),
    );
    i += 1;

    // By color (5) — counts permanents containing each WUBRG color.
    // A multicolor card contributes to multiple bins. Colorless
    // permanents don't appear in any of these bins (intentional —
    // their absence is itself a signal).
    for color in Color::all() {
        let count = bf
            .iter()
            .filter(|o| o.characteristics.colors.contains(color))
            .count();
        buf[i] = log_size(count);
        i += 1;
    }

    // Creature aggregates (3): total power, total toughness, total
    // damage. PtValue::Fixed contributions only — variable values
    // (`*`, `1+*`, …) drop out for v0 since their interpretation
    // depends on context not yet plumbed into observation. Acceptable
    // imprecision; revisit when those cards reach the pool.
    let mut total_power: i32 = 0;
    let mut total_toughness: i32 = 0;
    let mut total_damage: u32 = 0;
    for o in bf.iter().filter(|o| o.is_creature()) {
        if let Some(PtValue::Fixed(v)) = o.characteristics.power {
            total_power += v;
        }
        if let Some(PtValue::Fixed(v)) = o.characteristics.toughness {
            total_toughness += v;
        }
        total_damage += o.damage_marked;
    }
    buf[i] = sat_tanh(total_power as f32, 10.0);
    i += 1;
    buf[i] = sat_tanh(total_toughness as f32, 10.0);
    i += 1;
    buf[i] = sat_tanh(total_damage as f32, 10.0);
    i += 1;

    // Mana-value distribution (8): bins 0,1,2,3,4,5,6,7+.
    for cmc in 0u32..7 {
        let count = bf
            .iter()
            .filter(|o| o.characteristics.mana_value() == cmc)
            .count();
        buf[i] = log_size(count);
        i += 1;
    }
    let cmc_seven_plus = bf
        .iter()
        .filter(|o| o.characteristics.mana_value() >= 7)
        .count();
    buf[i] = log_size(cmc_seven_plus);
    i += 1;

    // Card-aware: keyword presence on controlled creatures (8). The aggregate
    // blocks above are board-shape only (type/color/CMC/P-T); these capture what
    // the creatures DO — evasion, combat tricks, resilience — which materially
    // changes a position's value. Layer-aware (counts granted keywords).
    for kw in [
        KeywordAbility::Flying, KeywordAbility::Trample, KeywordAbility::Deathtouch,
        KeywordAbility::Lifelink, KeywordAbility::FirstStrike, KeywordAbility::Vigilance,
        KeywordAbility::Menace, KeywordAbility::Indestructible,
    ] {
        let count = bf.iter()
            .filter(|o| o.is_creature() && state.has_keyword(o.id, &kw))
            .count();
        buf[i] = log_size(count);
        i += 1;
    }

    // Card-aware: hand composition (4). The plain hand-size count above can't
    // tell 7 lands from 7 spells; lands / creatures / other-spells / total mana
    // value capture mana-screw-vs-flood, gas, and curve. (For a player whose
    // hand is hidden/anonymized these read 0 — no information leak.)
    let hand: Vec<&GameObject> = state.objects.objects_in_zone(Zone::Hand(player)).collect();
    buf[i] = log_size(hand.iter().filter(|o| o.is_land()).count());
    i += 1;
    buf[i] = log_size(hand.iter().filter(|o| o.is_creature()).count());
    i += 1;
    buf[i] = log_size(hand.iter().filter(|o| !o.is_land() && !o.is_creature()).count());
    i += 1;
    let hand_mv: u32 = hand.iter().map(|o| o.characteristics.mana_value()).sum();
    buf[i] = sat_tanh(hand_mv as f32, 10.0);
    i += 1;

    debug_assert_eq!(i, PER_PLAYER_FEATURES);
}

// =============================================================================
// Game-level block (GAME_LEVEL_FEATURES floats)
// =============================================================================

fn encode_game_level_block(state: &GameState, buf: &mut [f32]) {
    debug_assert_eq!(buf.len(), GAME_LEVEL_FEATURES);
    let mut i = 0;

    // Turn number — saturating because turn 1 vs 5 is meaningful but
    // turn 15 vs 16 is not.
    buf[i] = sat_tanh(state.turn.turn_number as f32, 10.0);
    i += 1;

    // Phase one-hot (5).
    let phases = [
        Phase::Beginning,
        Phase::PreCombatMain,
        Phase::Combat,
        Phase::PostCombatMain,
        Phase::Ending,
    ];
    for ph in phases {
        buf[i] = if state.turn.phase == ph { 1.0 } else { 0.0 };
        i += 1;
    }

    // Step one-hot (11).
    let steps = [
        Step::Untap,
        Step::Upkeep,
        Step::Draw,
        Step::Main,
        Step::BeginCombat,
        Step::DeclareAttackers,
        Step::DeclareBlockers,
        Step::CombatDamage,
        Step::CombatDamageRegular,
        Step::EndCombat,
        Step::End,
    ];
    for st in steps {
        buf[i] = if state.turn.step == st { 1.0 } else { 0.0 };
        i += 1;
    }
    // Cleanup is omitted from the one-hot intentionally: it's
    // engine-internal; agents never have priority during cleanup
    // unless an ability triggered, in which case the engine routes
    // back through the phase/step machinery. Encoding it would just
    // waste a dim.

    // Combat present (1).
    buf[i] = if state.combat.is_some() { 1.0 } else { 0.0 };
    i += 1;

    // Stack depth (1) — log so deep stacks don't dominate.
    buf[i] = log_size(state.stack.len());
    i += 1;

    // Storm count (1).
    buf[i] = sat_tanh(state.storm_count as f32, 5.0);
    i += 1;

    debug_assert_eq!(i, GAME_LEVEL_FEATURES);
}

// =============================================================================
// Perspective block (PERSPECTIVE_FEATURES floats)
// =============================================================================

fn encode_perspective_block(
    state: &GameState,
    perspective: Option<PlayerId>,
    buf: &mut [f32],
) {
    debug_assert_eq!(buf.len(), PERSPECTIVE_FEATURES);
    match perspective {
        None => {
            // Canonical (perspective-agnostic) view: all-zero so
            // value networks consuming this can stay symmetric.
            for x in buf.iter_mut() {
                *x = 0.0;
            }
        }
        Some(p) => {
            buf[0] = if state.active_player() == p { 1.0 } else { 0.0 };
            buf[1] = if state.priority_player() == p { 1.0 } else { 0.0 };
            buf[2] = if state.is_game_over() { 1.0 } else { 0.0 };
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Player ordering for the per-player blocks: perspective first
/// (when supplied), then remaining players in canonical id order.
fn player_order(num_players: u8, perspective: Option<PlayerId>) -> Vec<PlayerId> {
    let mut out: Vec<PlayerId> = Vec::with_capacity(num_players as usize);
    if let Some(p) = perspective {
        if p < num_players {
            out.push(p);
        }
    }
    for id in 0..num_players {
        if Some(id) != perspective {
            out.push(id);
        }
    }
    debug_assert_eq!(out.len(), num_players as usize);
    out
}

/// Saturating tanh-style normalization: `value / scale → tanh`. Maps
/// any real input into `(-1, 1)`. Scale picks where the curve "bends"
/// — e.g. `sat_tanh(life, 20)` gives ~0.76 at 20 life, ~0.96 at 40,
/// and saturates beyond.
#[inline]
fn sat_tanh(value: f32, scale: f32) -> f32 {
    (value / scale).tanh()
}

/// Log-scale size normalization: `log(size + 1) / log(MAX + 1)`,
/// clamped to `[0, 1]`. Preserves resolution at small zone sizes
/// (where strategy distinctions live), compresses the tail.
/// `MAX = 60` chosen so a maximally-sized library still maps inside
/// the unit interval.
#[inline]
fn log_size(size: usize) -> f32 {
    const MAX_LOG: f32 = 4.110874; // ln(60 + 1)
    let v = ((size as f32) + 1.0).ln() / MAX_LOG;
    v.clamp(0.0, 1.0)
}

// =============================================================================
// tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn enc() -> BasicE2Encoder {
        BasicE2Encoder::for_two_players()
    }

    // -- dim ------------------------------------------------------------

    #[test]
    fn dim_is_stable() {
        // Constant + Encoder::dim() must agree. Update both
        // deliberately when feature counts change; silent drift
        // would invalidate any loaded checkpoint sized off the
        // public constant.
        assert_eq!(enc().dim(), BASIC_E2_DIM_TWO_PLAYERS);
        assert_eq!(BASIC_E2_DIM_TWO_PLAYERS, 123);
    }

    #[test]
    fn dim_scales_with_player_count() {
        let three = BasicE2Encoder::new(3);
        assert_eq!(
            three.dim(),
            3 * PER_PLAYER_FEATURES + GAME_LEVEL_FEATURES + PERSPECTIVE_FEATURES
        );
    }

    /// IdentityEncoder: dim = base + vocab*ID_ZONES; its first `base` features
    /// exactly equal BasicE2Encoder's output (append-only); the identity block is
    /// all-zero on a fresh game (nothing yet in any tracked zone); and encoding
    /// never panics for either perspective.
    #[test]
    fn identity_encoder_appends_and_matches_base() {
        use arcana_core::engine::new_game;
        let reg = arcana_cards::build_catalog();
        let deck = arcana_cards::sample_deck(&reg, 7);
        // Vocabulary = the deck's cards.
        let id = IdentityEncoder::from_card_ids(2, &deck, &reg);
        let base = BasicE2Encoder::for_two_players();

        assert!(id.vocab_len() > 0, "vocab should be non-empty");
        assert_eq!(id.dim(), base.dim() + id.vocab_len() * ID_ZONES);

        let (state, _y) = new_game(vec![deck.clone(), deck.clone()], &reg, 5);
        let fb = base.encode(&state, Some(0));
        let fi = id.encode(&state, Some(0));
        assert_eq!(fi.len(), id.dim());
        // Append-only: the base prefix is byte-identical.
        assert_eq!(&fi[..base.dim()], &fb[..], "identity block must not disturb base features");
        // Both perspectives encode without panic + right length.
        assert_eq!(id.encode(&state, Some(1)).len(), id.dim());
        assert_eq!(id.encode(&state, None).len(), id.dim());
    }

    // -- output shape ---------------------------------------------------

    #[test]
    fn encode_returns_dim_floats() {
        let state = GameState::new(2, 0);
        let v = enc().encode(&state, Some(0));
        assert_eq!(v.len(), BASIC_E2_DIM_TWO_PLAYERS);
    }

    #[test]
    fn encode_into_panics_on_undersized_buf() {
        let state = GameState::new(2, 0);
        let mut buf = vec![0.0f32; BASIC_E2_DIM_TWO_PLAYERS - 1];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            enc().encode_into(&state, Some(0), &mut buf);
        }));
        assert!(result.is_err());
    }

    // -- determinism ----------------------------------------------------

    #[test]
    fn encoding_is_deterministic() {
        let state = GameState::new(2, 42);
        let a = enc().encode(&state, Some(0));
        let b = enc().encode(&state, Some(0));
        assert_eq!(a, b);
    }

    // -- finiteness -----------------------------------------------------

    #[test]
    fn all_outputs_are_finite() {
        // Encode a handful of representative states (initial + with
        // result variants set). A richer "1000 random reachable
        // states" test belongs in arcana-cli's replay tooling once
        // it can drive games — for v0 this catches the obvious
        // NaN-inducing field regressions (life going negative,
        // poison spiking, etc.).
        use arcana_core::state::GameResult;
        let mut states = vec![GameState::new(2, 0)];

        let mut s = GameState::new(2, 0);
        s.player_mut(0).life = -100;
        s.player_mut(1).life = 1_000_000;
        states.push(s);

        let mut s = GameState::new(2, 0);
        s.player_mut(0).poison_counters = 50;
        s.player_mut(0).energy = 10_000;
        states.push(s);

        let mut s = GameState::new(2, 0);
        s.result = Some(GameResult::Win(0));
        states.push(s);

        let perspectives: [Option<PlayerId>; 3] = [None, Some(0), Some(1)];
        for state in &states {
            for &perspective in &perspectives {
                let v = enc().encode(state, perspective);
                for (i, &x) in v.iter().enumerate() {
                    assert!(
                        x.is_finite(),
                        "non-finite output at index {i}: {x} (perspective={perspective:?})"
                    );
                }
            }
        }
    }

    // -- perspective-flip symmetry --------------------------------------

    #[test]
    fn perspective_flip_swaps_per_player_blocks() {
        // Different life totals so the two player blocks are
        // distinguishable.
        let mut state = GameState::new(2, 0);
        state.player_mut(0).life = 17;
        state.player_mut(1).life = 13;

        let from_p0 = enc().encode(&state, Some(0));
        let from_p1 = enc().encode(&state, Some(1));

        // Per-player blocks should be SWAPPED:
        //   from_p0[block_0] == from_p1[block_1]
        //   from_p0[block_1] == from_p1[block_0]
        let block_0 = 0..PER_PLAYER_FEATURES;
        let block_1 = PER_PLAYER_FEATURES..(2 * PER_PLAYER_FEATURES);
        assert_eq!(from_p0[block_0.clone()], from_p1[block_1.clone()]);
        assert_eq!(from_p0[block_1.clone()], from_p1[block_0.clone()]);

        // Game-level block must be identical (perspective-invariant).
        let game = (2 * PER_PLAYER_FEATURES)..(2 * PER_PLAYER_FEATURES + GAME_LEVEL_FEATURES);
        assert_eq!(from_p0[game.clone()], from_p1[game]);
    }

    // -- canonical (None) view ------------------------------------------

    #[test]
    fn perspective_none_zeros_indicators() {
        let state = GameState::new(2, 0);
        let v = enc().encode(&state, None);
        let start = 2 * PER_PLAYER_FEATURES + GAME_LEVEL_FEATURES;
        for i in 0..PERSPECTIVE_FEATURES {
            assert_eq!(
                v[start + i],
                0.0,
                "perspective indicator {i} should be 0 when perspective=None"
            );
        }
    }

    #[test]
    fn perspective_none_uses_canonical_player_order() {
        let mut state = GameState::new(2, 0);
        state.player_mut(0).life = 17;
        state.player_mut(1).life = 13;

        let from_none = enc().encode(&state, None);
        let from_p0 = enc().encode(&state, Some(0));

        // None should match Some(0) in per-player ordering since
        // canonical id order starts with 0 anyway.
        let blocks = 0..(2 * PER_PLAYER_FEATURES);
        assert_eq!(from_none[blocks.clone()], from_p0[blocks]);
    }

    // -- initial-state sanity ------------------------------------------

    #[test]
    fn initial_state_life_normalizes_to_expected_value() {
        let state = GameState::new(2, 0);
        let v = enc().encode(&state, Some(0));
        // life=20 → tanh(20/20) = tanh(1) ≈ 0.7616
        let expected = (1.0f32).tanh();
        assert!(
            (v[0] - expected).abs() < 1e-5,
            "expected ~{expected}, got {}",
            v[0]
        );
    }

    // -- helpers --------------------------------------------------------

    #[test]
    fn sat_tanh_saturates_in_extremes() {
        assert!(sat_tanh(0.0, 5.0).abs() < 1e-6);
        assert!(sat_tanh(1e9, 5.0) > 0.999);
        assert!(sat_tanh(-1e9, 5.0) < -0.999);
    }

    #[test]
    fn log_size_is_zero_at_zero_and_clamps_at_max() {
        assert_eq!(log_size(0), 0.0);
        let big = log_size(1_000_000);
        assert!(big <= 1.0);
        assert!(big > 0.5);
    }

    #[test]
    fn player_order_perspective_first() {
        assert_eq!(player_order(2, Some(0)), vec![0, 1]);
        assert_eq!(player_order(2, Some(1)), vec![1, 0]);
        assert_eq!(player_order(2, None), vec![0, 1]);
        assert_eq!(player_order(3, Some(2)), vec![2, 0, 1]);
    }

    // -- object safety --------------------------------------------------

    #[test]
    fn encoder_trait_is_object_safe() {
        let _: Box<dyn Encoder> = Box::new(BasicE2Encoder::for_two_players());
    }
}

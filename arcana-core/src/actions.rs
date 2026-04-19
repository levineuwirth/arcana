//! The [`Action`] enum — every decision a player can make, expressed as
//! data.
//!
//! Addendum Listing 6 / Phase 1 Task #8. Depends on tasks 1–5.
//!
//! This is the **single interface** between the engine and any agent
//! (human, bot, RL policy, tree search). The engine yields an
//! [`EngineYield::PendingDecision`](crate::engine::EngineYield) that
//! advertises the legal actions in the current state; the agent chooses
//! one; the engine applies it via [`crate::engine::step`] and yields the
//! next decision point.
//!
//! Every `Action` is **batch-flat**: a full spell cast, including target
//! selection, mode choices, X value, mana payment plan, and alternative
//! costs, is one `Action::CastSpell { .. }`. The engine does not ask for
//! pieces one-by-one. This matches [addendum Section 6.2 "action
//! flattening"] — enumerating the full Cartesian product of sub-choices
//! keeps the agent interface pure function-of-state.
//!
//! `Action` derives `Hash, PartialEq, Eq` so MCTS transposition tables and
//! action deduplication work out of the box. Every sub-type (targets,
//! modes, mana plan, additional costs, choice kinds) does likewise —
//! equality is structural, not reference-based.
//!
//! [addendum Section 6.2 "action flattening"]: arcana-ai/src/action_flattening.rs

use serde::{Deserialize, Serialize};

use crate::combat::{AttackerDeclaration, BlockerDeclaration, DamageAssignment};
use crate::objects::ObjectId;

/// Alternative or cost-modifying cast path for [`Action::CastSpell`].
///
/// Per CR 601.2f, a spell can use at most one alternative cost — that
/// constraint is expressed at the type level by the single-slot
/// `Option`-style shape of this enum on the action. Additional costs
/// (kicker, buyback, emerge's sacrifice) are orthogonal and live in
/// [`Action::CastSpell::additional_costs`].
///
/// Variants are expected to grow as keyword-style alt-cost mechanics
/// land (foretell, adventure, madness, spectacle, …). Keep this a
/// marker enum — look up the *live* cost via
/// [`crate::state::GameState::effective_keywords`] at cast time so
/// cost reductions and Snapcaster-style temporary grants work.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CastModifier {
    /// The default — pay the printed mana cost from the normal cast
    /// zone (hand for spells, etc.). No zone override, no
    /// exile-on-leave.
    #[default]
    None,
    /// CR 702.33 — cast from the graveyard for the flashback cost.
    /// The stack entry is flagged so that leaving the stack (resolve /
    /// counter / fizzle) routes to exile rather than the owner's
    /// graveyard.
    Flashback,
}

/// Bundle of *cost-reduction* choices (CR 601.2f category: "cost
/// reductions") carried on every [`Action::CastSpell`]. Distinct
/// from:
/// * [`CastModifier`] — *alternative* costs (flashback, foretell).
///   At most one applies (CR 601.2f).
/// * `additional_costs` — *additional* costs (kicker extra cost,
///   sacrifice, discard). Orthogonal to both.
///
/// Cost reductions compose. Delve and convoke on the same spell is
/// legal (rules-wise, even though no printed card has both), and
/// improvise joins as another sibling field when it lands. Grouping
/// them here keeps validation in one place and keeps the
/// [`Action::CastSpell`] field list from sprawling.
///
/// Every field is `Option`-wrapped: `None` = the keyword is absent
/// from this cast; `Some(vec![])` = keyword present, chose to use
/// zero. The distinction catches "agent passes data for a keyword
/// the card doesn't have" as invalid at the type level.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostReductions {
    /// Cards to exile from the caster's graveyard as delve payment
    /// (CR 702.66). Each exile satisfies `{1}` generic.
    pub delve_exiles: Option<Vec<ObjectId>>,
    /// Creatures to tap for convoke (CR 702.51). Each assignment
    /// names the creature and which pip it pays for. `None` = card
    /// has no convoke; `Some(vec![])` = has convoke, chose none.
    pub convoke_taps: Option<Vec<ConvokeAssignment>>,
    /// Artifacts to tap for improvise (CR 702.127). Each tapped
    /// artifact satisfies `{1}` generic. No assignment needed (all
    /// pay generic only; improvise can never pay colored pips).
    /// `None` = card has no improvise; `Some(vec![])` = has
    /// improvise, chose none.
    pub improvise_taps: Option<Vec<ObjectId>>,
}

/// One creature's contribution to a convoke payment (CR 702.51b).
/// The agent must make both choices explicitly: *which* creature to
/// tap, and *which pip* that tap pays for. A multicolored creature
/// can offer more than one color; the agent picks one per cast.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvokeAssignment {
    pub creature: ObjectId,
    pub payment: ConvokePayment,
}

/// What pip a convoke-tapped creature covers.
///
/// Per CR 702.51b, each tapped creature pays either `{1}` generic or
/// one mana of any of its colors. Colorless creatures pay generic
/// only; multicolored creatures can pay any of their colors.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConvokePayment {
    /// This creature's tap satisfies one `{1}` generic pip.
    Generic,
    /// This creature's tap satisfies one colored pip of this color.
    /// Only legal if the creature has this color.
    Color(crate::types::ManaColor),
}
use crate::priority::SpecialAction;
use crate::stack::ModeChoice;
use crate::targets::TargetSelection;
use crate::types::*;

// =============================================================================
// Action
// =============================================================================

/// Every player decision the engine accepts. Agents produce `Action`s;
/// the engine consumes them.
///
/// Variants are grouped by decision context (priority window, combat
/// declaration, resolution choice, special-action window). [`Action::kind`]
/// gives the coarse category.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    // === Priority window ===================================================
    /// Pass priority. When both players pass in succession with an empty
    /// stack, the phase/step advances; with a non-empty stack, the top
    /// object resolves.
    PassPriority,

    /// Cast a spell. Carries every decision the spell requires — targets,
    /// mode choices, X value, mana payment, and alternative/additional
    /// costs — so the engine can apply the cast in a single step.
    CastSpell {
        /// Object id of the card being cast. Usually in hand, but can be
        /// any zone with the appropriate alternative cost (flashback,
        /// foretell, adventure).
        object_id: ObjectId,
        targets: TargetSelection,
        /// One entry per modal clause; empty for non-modal spells.
        modes: Vec<ModeChoice>,
        mana_payment: ManaPaymentPlan,
        additional_costs: Vec<AdditionalCostPayment>,
        /// Chosen value for `{X}` when the spell has one; `None` otherwise.
        x_value: Option<u32>,
        /// Alternative or cost-modifying cast path ([`CastModifier`]).
        /// Default [`CastModifier::None`] means paying the card's
        /// printed mana cost from the usual zone (hand). Other variants
        /// pay a different cost and may change zone restrictions or
        /// post-resolution routing (e.g. flashback → exile-on-leave).
        cast_modifier: CastModifier,
        /// Cost-reduction keyword choices (delve exiles today; convoke
        /// taps and improvise artifact-taps on the roadmap). See
        /// [`CostReductions`] for the compositional rules.
        cost_reductions: CostReductions,
    },

    /// Activate an ability on `source`. `ability_index` is the 0-based
    /// position in the card's activated-ability list.
    ActivateAbility {
        source: ObjectId,
        ability_index: usize,
        targets: TargetSelection,
        mana_payment: ManaPaymentPlan,
        additional_costs: Vec<AdditionalCostPayment>,
    },

    /// Play a land. Legal only during the active player's main phase with
    /// the stack empty, and only if `land_plays_remaining > 0`.
    PlayLand { object_id: ObjectId },

    // === Combat declarations ==============================================
    /// Declare attackers — batch action containing every attacker and its
    /// chosen defender. The empty list is legal (no attacks this combat).
    DeclareAttackers { attackers: Vec<AttackerDeclaration> },

    /// Declare blockers — every blocker-to-attacker pairing. Empty list is
    /// legal (no blocks).
    DeclareBlockers { blockers: Vec<BlockerDeclaration> },

    /// Per-attacker damage ordering (CR 510.1c) — only needed when an
    /// attacker is blocked by multiple creatures and the attacker's
    /// controller must choose damage assignment order. One entry per
    /// multi-blocked attacker.
    OrderBlockers { assignments: Vec<DamageAssignment> },

    // === Resolution / in-engine choices ===================================
    /// A choice made during spell/ability resolution (chosen target,
    /// chosen color, distribute-damage amounts, etc.). Contextualized by
    /// the [`DecisionContext`] the engine yielded alongside.
    MakeChoice(ChoiceAction),

    /// Reply to a pending [`PendingChoice`] pushed by a resolving
    /// effect. The `id` must match
    /// [`crate::state::GameState::pending_choice`]'s id — a mismatch
    /// is a hard error (stale reply from the session/UI layer after
    /// the choice it was answering has already resolved).
    SubmitResolutionChoice {
        id: u64,
        response: ChoiceResponse,
    },

    // === Special-action windows ===========================================
    /// Keep the current opening hand (ends the mulligan decision).
    MulliganKeep,

    /// Take another mulligan. Under the London mulligan, this shuffles the
    /// hand back and draws 7; the owed bottoms are paid via
    /// [`Action::BottomCards`] once the player finally keeps.
    MulliganAgain,

    /// London mulligan: put this ordered list of cards on the bottom of
    /// the player's library. Length must equal the number of owed bottoms
    /// (number of mulligans taken).
    BottomCards(Vec<ObjectId>),

    /// Concede. Legal at any time a player has priority.
    Concede,
}

impl Action {
    /// Coarse category — useful for log filtering, policy masking, and
    /// decoupling the AI flattener from per-variant knowledge.
    pub fn kind(&self) -> ActionKind {
        use Action::*;
        match self {
            PassPriority => ActionKind::Priority,
            CastSpell { .. } => ActionKind::Cast,
            ActivateAbility { .. } => ActionKind::Activate,
            PlayLand { .. } => ActionKind::PlayLand,
            DeclareAttackers { .. } | DeclareBlockers { .. } | OrderBlockers { .. } => {
                ActionKind::Combat
            }
            MakeChoice(_) | SubmitResolutionChoice { .. } => ActionKind::ResolutionChoice,
            MulliganKeep | MulliganAgain | BottomCards(_) | Concede => ActionKind::Special,
        }
    }

    pub fn is_pass(&self) -> bool {
        matches!(self, Action::PassPriority)
    }
    pub fn is_concede(&self) -> bool {
        matches!(self, Action::Concede)
    }
    pub fn is_cast(&self) -> bool {
        matches!(self, Action::CastSpell { .. })
    }
    pub fn is_combat(&self) -> bool {
        self.kind() == ActionKind::Combat
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionKind {
    Priority,
    Cast,
    Activate,
    PlayLand,
    Combat,
    ResolutionChoice,
    Special,
}

// =============================================================================
// ManaPaymentPlan
// =============================================================================

/// A concrete plan for paying a mana cost. Output of the payment solver
/// (addendum Section 8, Task #12).
///
/// The plan is **one enumerated possibility** among all ways the cost
/// could be paid. The solver may produce several functionally-equivalent
/// plans (e.g. two Mountains can pay a single `{R}` in two orderings);
/// deduplication is the solver's responsibility before presenting plans
/// as distinct actions.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManaPaymentPlan {
    /// Which mana units in the pool pay which cost components.
    pub assignments: Vec<ManaAssignment>,
    /// Convoke: tap these creatures as proxies for generic mana.
    pub convoke_creatures: Vec<ObjectId>,
    /// Delve: exile these cards from graveyard as proxies for generic mana.
    pub delve_cards: Vec<ObjectId>,
    /// Phyrexian: the cost-component indices paid by paying 2 life instead.
    pub phyrexian_life_payments: Vec<usize>,
}

impl ManaPaymentPlan {
    /// The empty plan — paying zero mana.
    pub fn empty() -> Self {
        Self::default()
    }
}

/// A single mana unit assigned to a single cost component.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManaAssignment {
    pub pool_index: usize, // index into ManaPool.pool
    pub cost_index: usize, // index into ManaCost.components
}

// =============================================================================
// AdditionalCostPayment
// =============================================================================

/// Costs beyond mana that casting a spell or activating an ability may
/// require (sacrifice, pay-life, reveal, etc.).
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdditionalCostPayment {
    Sacrifice(ObjectId),
    Discard(ObjectId),
    /// Delve's exile is in the mana plan; this is for cost-text exile like
    /// "exile N cards from your graveyard" in non-delve contexts.
    ExileFromGraveyard(Vec<ObjectId>),
    PayLife(u32),
    /// Convoke's tap targets. Redundant with [`ManaPaymentPlan::convoke_creatures`]
    /// but kept separate for costs that require tapping without mana.
    TapCreatures(Vec<ObjectId>),
    RemoveCounters {
        source: ObjectId,
        kind: CounterKind,
        count: u32,
    },
    /// Planeswalker plus-loyalty costs (CR 606.2). Mirror of
    /// [`Self::RemoveCounters`]; the activation places `count`
    /// counters of `kind` on `source`. Routed through
    /// [`crate::state::GameState::place_counters`] so Doubling
    /// Season et al. compose for `+1` activations.
    AddCounters {
        source: ObjectId,
        kind: CounterKind,
        count: u32,
    },
    RevealCard(ObjectId),
}

// =============================================================================
// ChoiceAction
// =============================================================================

// =============================================================================
// PendingChoice — a mid-resolution choice the engine is waiting on
// =============================================================================

/// A mid-resolution choice the engine has pushed onto
/// [`crate::state::GameState::pending_choice`]. The engine yields an
/// [`crate::engine::EngineYield::PendingDecision`] with
/// [`DecisionContext::ResolutionChoice`] carrying the `kind`; the agent
/// answers with [`Action::SubmitResolutionChoice`].
///
/// The `id` is a monotonic token; the submitted action must carry the
/// same id or the engine rejects the response (CR-independent guard
/// against stale replies after undo / session replay).
///
/// Single-slot (Phase 2-A): only one `pending_choice` exists at a time.
/// Multi-target effects that want per-target choices decompose into
/// sequential single-choice submissions.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PendingChoice {
    pub id: u64,
    pub choosing_player: PlayerId,
    pub context: ChoiceContext,
    pub kind: ChoiceKind,
}

/// Where the choice is happening. Discriminates agent-facing rendering
/// and dispatcher logic (stack-resolution vs SBA-time vs other).
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceContext {
    /// Mid-resolution of a specific stack entry (Scry, Tutor, Ward
    /// during a spell, etc.).
    ResolvingStack(ObjectId),
    /// State-based-action time choice (Legend rule tiebreak).
    Sba,
    /// Some other engine-driven prompt that doesn't fit the above.
    Other,
}

/// What kind of choice is being asked. Each variant pairs with a
/// specific [`ChoiceAction`] shape the agent must submit.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceKind {
    /// Put each of `cards` into one of `allowed` zones (Scry, Surveil,
    /// Fateseal). Answer: [`ChoiceAction::OrderCardsWithDestinations`].
    OrderCards {
        cards: Vec<ObjectId>,
        allowed: Vec<CardDestination>,
    },
    /// Pick between `min` and `max` (inclusive) cards from `candidates`
    /// (Tutor, Sacrifice, Discard, Legend rule). Answer:
    /// [`ChoiceAction::ChooseOrdered`] with indices into `candidates`.
    PickCards {
        candidates: Vec<ObjectId>,
        min: u32,
        max: u32,
    },
    /// Distribute `total` counters of `kind` among `among`. Answer:
    /// [`ChoiceAction::Distribute`].
    DistributeCounters {
        among: Vec<ObjectId>,
        total: u32,
        kind: CounterKind,
    },
    /// Distribute `total` damage among `among`, each assigned at least
    /// `min_per_target` (CR 510.1c trample/Lightning Helix-style). Answer:
    /// [`ChoiceAction::Distribute`].
    DistributeDamage {
        among: Vec<ObjectId>,
        total: u32,
        min_per_target: u32,
    },
    /// Pay the cost or decline. Ward's pay-or-counter fits here.
    /// Answer: [`ChoiceAction::ChooseYesNo`] (`true` = pay).
    PayOrDecline {
        cost: crate::mana::ManaCost,
        on_decline: DeclineConsequence,
    },
    /// Binary yes/no prompt (e.g. "may cast for free — yes/no").
    /// Answer: [`ChoiceAction::ChooseYesNo`].
    YesNo {
        prompt: SmallString,
    },
    /// Pick a player from `candidates`. Answer:
    /// [`ChoiceAction::ChoosePlayer`].
    PickPlayer {
        candidates: Vec<PlayerId>,
    },
    /// Pick targets at resolution time (storm copies, copy-spell
    /// effects). The actual list of targeting clauses lives on
    /// [`crate::state::GameState::pending_target_requirements`] —
    /// stored there because [`crate::targets::TargetRequirement`]
    /// carries fn-pointer-bearing filters and isn't Hash/Eq/Serialize.
    /// Answer: [`ChoiceResponse::ChooseTargets`] with selection length
    /// equal to that requirement vector's length.
    /// `source` is the stack object on whose behalf targets are chosen.
    ChooseTargets {
        source: ObjectId,
    },
}

/// Destinations a card can be placed during an `OrderCards` choice.
///
/// Names follow spec §41.3 verbatim — `TopOfLibrary` / `BottomOfLibrary`
/// are explicit rather than the ambiguous `Top` / `Bottom` to avoid
/// confusion with "top of the graveyard" or "top of exile pile" which
/// have no game meaning.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardDestination {
    TopOfLibrary,
    BottomOfLibrary,
    Graveyard,
    Hand,
    Exile,
    /// Used by tutor-to-battlefield effects (Natural Order, reanimation).
    /// The picked card enters the battlefield under the choosing player's
    /// control per the effect's semantics.
    Battlefield,
}

/// What happens when a player declines to pay a [`ChoiceKind::PayOrDecline`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeclineConsequence {
    /// Counter the named stack entry (Ward's declined payment).
    CounterStackEntry(ObjectId),
    /// Skip the effect that offered the choice (optional "may" effects).
    SkipEffect,
}

/// Agent reply to a [`PendingChoice`]. Paired 1:1 with [`ChoiceKind`];
/// submitting a response whose variant doesn't match the pending
/// [`ChoiceKind`] panics at the dispatcher (programmer / agent bug,
/// per spec §41.6 R4).
///
/// This is distinct from the legacy [`ChoiceAction`] used by
/// [`Action::MakeChoice`] for SpecialAction-driven flows (mulligan
/// bottoms, discard-to-hand-size). New resolution-choice code should
/// use `ChoiceResponse` exclusively.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceResponse {
    /// Reply to [`ChoiceKind::OrderCards`]: each card paired with its
    /// chosen destination.
    OrderCards { placements: Vec<(ObjectId, CardDestination)> },
    /// Reply to [`ChoiceKind::PickCards`]: the chosen subset.
    PickCards { picked: Vec<ObjectId> },
    /// Reply to [`ChoiceKind::DistributeCounters`].
    DistributeCounters { distribution: Vec<(ObjectId, u32)> },
    /// Reply to [`ChoiceKind::DistributeDamage`].
    DistributeDamage { distribution: Vec<(ObjectId, u32)> },
    /// Reply to [`ChoiceKind::PayOrDecline`]: `true` = pay.
    PayOrDecline { pay: bool },
    /// Reply to [`ChoiceKind::YesNo`].
    YesNo { answer: bool },
    /// Reply to [`ChoiceKind::PickPlayer`].
    PickPlayer { picked: PlayerId },
    /// Reply to [`ChoiceKind::ChooseTargets`]: the chosen selection
    /// over the requirements. Selection length must match the number
    /// of requirements; each `TargetChoice` is validated at the
    /// dispatcher (legality is rechecked per CR 608.2b).
    ChooseTargets { selection: crate::targets::TargetSelection },
}

/// A spell/ability resolution that yielded mid-way to push a
/// [`PendingChoice`]. Held on [`crate::state::GameState::pending_resolution`]
/// until the agent submits the answer; the engine then resumes the
/// remaining effects and finalizes the entry.
///
/// Single-slot (Phase 2-A): at most one parked resolution at any time —
/// matches the single-slot `pending_choice`.
///
/// `Effect` does not implement serde (it carries fn pointers in a few
/// variants), so `PendingResolution` is not serialized — agent
/// checkpoints taken while a resolution is parked mid-effect cannot be
/// restored across process boundaries. See `TODO(serialize)` thread in
/// the engine doc.
#[derive(Clone, Debug)]
pub struct PendingResolution {
    pub entry: crate::stack::StackEntry,
    pub remaining_effects: Vec<crate::effects::Effect>,
    /// Tracks whether to finalize as spell or ability when the
    /// remaining effects drain. Cached off `entry.is_spell()` so the
    /// stashed entry can be consumed by finalization without a
    /// second borrow.
    pub is_spell: bool,
    /// Pre-effect Ward prompts still to resolve (Phase 2-A stopgap —
    /// CR 702.21a is *actually* a triggered ability that stacks
    /// separately; we approximate with a sequential PayOrDecline at
    /// resolution time). Each `(target, cost)` pushes a
    /// [`ChoiceKind::PayOrDecline`]; a decline counters the entry and
    /// drops the queue. When drained, `remaining_effects` proceed.
    /// TODO(phase-2b): real trigger routing with Stifle interaction.
    pub ward_queue: Vec<(crate::objects::ObjectId, crate::mana::ManaCost)>,
}

/// Effect-supplied follow-up the dispatcher runs after an agent answers
/// a [`ChoiceKind::PickCards`] or [`ChoiceKind::ChooseTargets`] prompt.
/// The pushing effect stashes its semantics in
/// [`crate::state::GameState::pending_choice_follow_up`]; the
/// dispatcher consumes it alongside the choice answer and applies the
/// named operation to the chosen ids / targets.
#[derive(Clone, Debug)]
pub enum ChoiceFollowUp {
    /// Move each picked card to `destination`. `reveal` marks the card
    /// as known to all players (public-info tutor). When
    /// `shuffle_library_owner` is `Some`, the given player's library is
    /// shuffled after the move (tutor-style).
    MoveToZone {
        destination: crate::zones::Zone,
        reveal: bool,
        shuffle_library_owner: Option<PlayerId>,
    },
    /// Put the picked card onto the battlefield under `controller`'s
    /// control (reanimator / Natural-Order-style). Optionally enters
    /// tapped. When `shuffle_library_owner` is `Some`, that player's
    /// library is shuffled after the move.
    MoveToBattlefield {
        controller: PlayerId,
        tapped: bool,
        shuffle_library_owner: Option<PlayerId>,
    },
    /// Sacrifice each picked permanent (move to owner's graveyard,
    /// emit [`crate::events::GameEvent::Sacrifice`]).
    Sacrifice { player: PlayerId },
    /// Discard each picked card from hand (move to graveyard, emit
    /// [`crate::events::GameEvent::Discarded`]).
    Discard { player: PlayerId },
    /// Pair with a [`ChoiceKind::ChooseTargets`] response: overwrite
    /// the named stack entry's `targets` with the answered selection.
    /// Used by storm copies and by `Effect::CopySpell` to attach
    /// per-copy chosen targets (CR 706.10).
    ApplyTargetsToStackEntry { entry_id: ObjectId },
    /// Pair with a [`ChoiceKind::PickCards`] response: for each
    /// picked id, install a layer-6 grant of
    /// `KeywordAbility::Flashback(C)` where `C` is the picked
    /// card's printed mana cost (CR 702.33). Snapcaster Mage's
    /// ETB is the canonical consumer. `source` is the grantor
    /// permanent; `duration` is typically `Duration::EndOfTurn`.
    ///
    /// The grant is keyed on the *picked* ObjectId, not the card
    /// id — when the card leaves the graveyard (cast via flashback,
    /// exiled, shuffled), the zone change re-ids it per CR 400.7
    /// and the re-entered object doesn't inherit the grant. This is
    /// the behavior the rules require.
    GrantFlashbackEqualToOwnManaCost {
        source: ObjectId,
        duration: crate::layers::Duration,
    },
}

// =============================================================================
// ChoiceAction
// =============================================================================

/// A choice made during spell/ability resolution. The engine yields a
/// [`DecisionContext`] describing *what* is being chosen; the agent picks
/// one variant here to answer.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChoiceAction {
    ChooseObject(ObjectId),
    ChoosePlayer(PlayerId),
    /// "As ~ enters, choose a color" — always one of the 5 colors. For
    /// effects that need "color or colorless", use
    /// [`ChoiceAction::ChooseManaColor`].
    ChooseColor(Color),
    /// "Choose a color of mana" — rare; explicitly allows colorless.
    ChooseManaColor(ManaColor),
    /// "Choose a creature type" / "choose a land type" — interned string.
    ChooseType(SmallString),
    ChooseNumber(u32),
    ChooseYesNo(bool),
    /// "Choose one or more" / "choose an order". Indices refer to the
    /// ordered list the engine presented.
    ChooseOrdered(Vec<usize>),
    /// Distribute N among targets (e.g. "distribute 3 damage among any
    /// number of creatures").
    Distribute(Vec<(ObjectId, u32)>),
    /// Legacy Scry-shape answer from the pre-framework era: the list is
    /// the agent's chosen library ordering. Kept for [`Action::MakeChoice`]
    /// paths that predate the resolution-choice framework. New code
    /// should use [`Action::SubmitResolutionChoice`] +
    /// [`ChoiceResponse::OrderCards`].
    OrderCards(Vec<ObjectId>),
}

// =============================================================================
// DecisionContext — what the engine is asking about
// =============================================================================

/// Context packaged with every `PendingDecision` yield. Tells the agent
/// *which kind of decision* is being requested so it can pick the right
/// action family.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionContext {
    /// Ordinary priority window — agent may pass, cast, activate, play a
    /// land, or concede.
    Priority,
    CastingSpell {
        spell: ObjectId,
        sub_step: CastSubStep,
    },
    ActivatingAbility {
        source: ObjectId,
        sub_step: CastSubStep,
    },
    DeclareAttackers,
    DeclareBlockers,
    /// Attacker must distribute damage among its blockers in order
    /// (CR 510.1c).
    DistributeDamage {
        attacker: ObjectId,
    },
    /// A choice made during resolution of a stack entry (e.g. Collected
    /// Company's "choose up to 2 creatures").
    ResolutionChoice {
        stack_entry: ObjectId,
        prompt: String,
    },
    Mulligan,
    BottomCards {
        count: u32,
    },
    DiscardToHandSize {
        count: u32,
    },
    SpecialAction(SpecialAction),
}

/// The six sub-steps of casting a spell (CR 601.2) that the engine may
/// pause on. Only meaningful within
/// [`DecisionContext::CastingSpell`] / [`DecisionContext::ActivatingAbility`].
///
/// # Deferred: Shape B-full substep pipeline
///
/// Phase 1 ships *atomic* casts — every cast decision (targets, modes,
/// X, mana, additional costs, cost-modifier choices like delve) is
/// bundled into a single [`Action::CastSpell`] packet. This enum names
/// the intended substeps but the engine does not currently yield a
/// [`crate::engine::EngineYield::PendingDecision`] at each one; it
/// consumes the full action in one step.
///
/// When we eventually refactor to a true substep pipeline (yield per
/// substep, agent responds each time), it will be one change that
/// touches every cast path simultaneously: hand cast, flashback,
/// storm copies, cascade hits, future alt-costs. Doing it piecemeal
/// per new mechanic would leak inconsistent shapes into the codebase.
///
/// Gating: do the refactor when AI action-space enumeration becomes a
/// bottleneck in training. With characteristic-equivalence dedup,
/// delve-heavy casts stay well under the spec's P99 target of 500
/// actions — but if profiling shows otherwise the substep yield is
/// the answer.
///
/// Until then: the compositional-fields pattern (cost-reduction
/// choices bundled into [`CostReductions`] on [`Action::CastSpell`])
/// is the correct shape. It matches CR 601.2f-h's "simultaneous cost
/// payment" framing directly; Shape B would model those sub-events
/// as sequential round-trips, which is a *less* rules-faithful
/// execution model unless carefully re-collapsed at cost-payment
/// emit time.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CastSubStep {
    ChooseModes,
    ChooseTargets { clause_index: usize },
    ChooseXValue,
    ChooseAdditionalCosts,
    PayMana,
    Confirm,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DefendingEntity;
    use crate::targets::{TargetChoice, TargetSelection};

    fn pay_nothing() -> ManaPaymentPlan {
        ManaPaymentPlan::empty()
    }

    fn cast(object_id: ObjectId) -> Action {
        Action::CastSpell {
            object_id,
            targets: TargetSelection { targets: vec![] },
            modes: vec![],
            mana_payment: pay_nothing(),
            additional_costs: vec![],
            x_value: None,
            cast_modifier: CastModifier::None,
            cost_reductions: CostReductions::default(),
        }
    }

    // --- kind() / category helpers ------------------------------------------

    #[test]
    fn kind_categorizes_every_variant() {
        assert_eq!(Action::PassPriority.kind(), ActionKind::Priority);
        assert_eq!(cast(1).kind(), ActionKind::Cast);
        assert_eq!(
            Action::PlayLand { object_id: 1 }.kind(),
            ActionKind::PlayLand
        );
        assert_eq!(
            Action::DeclareAttackers { attackers: vec![] }.kind(),
            ActionKind::Combat
        );
        assert_eq!(
            Action::MakeChoice(ChoiceAction::ChooseYesNo(true)).kind(),
            ActionKind::ResolutionChoice
        );
        assert_eq!(Action::MulliganKeep.kind(), ActionKind::Special);
        assert_eq!(Action::Concede.kind(), ActionKind::Special);
        assert_eq!(Action::BottomCards(vec![1, 2]).kind(), ActionKind::Special);
    }

    #[test]
    fn is_pass_is_concede_is_cast() {
        assert!(Action::PassPriority.is_pass());
        assert!(!cast(1).is_pass());

        assert!(Action::Concede.is_concede());
        assert!(!Action::PassPriority.is_concede());

        assert!(cast(1).is_cast());
        assert!(!Action::PassPriority.is_cast());
    }

    #[test]
    fn is_combat_matches_declaration_and_ordering() {
        let attackers = Action::DeclareAttackers { attackers: vec![] };
        let blockers = Action::DeclareBlockers { blockers: vec![] };
        let order = Action::OrderBlockers {
            assignments: vec![],
        };

        assert!(attackers.is_combat());
        assert!(blockers.is_combat());
        assert!(order.is_combat());
        assert!(!cast(1).is_combat());
    }

    // --- structural equality and hashing -----------------------------------

    #[test]
    fn same_cast_is_structurally_equal() {
        // Two Actions built from the same pieces compare equal and hash
        // equal — essential for MCTS transposition tables.
        let a = cast(42);
        let b = cast(42);
        assert_eq!(a, b);

        use crate::collections::HashSet;
        let mut set = HashSet::default();
        set.insert(a);
        assert!(set.contains(&b));
    }

    #[test]
    fn different_object_ids_are_not_equal() {
        assert_ne!(cast(1), cast(2));
    }

    #[test]
    fn attackers_equal_by_field_value() {
        let a = Action::DeclareAttackers {
            attackers: vec![AttackerDeclaration {
                attacker: 1,
                defending: DefendingEntity::Player(1),
            }],
        };
        let b = Action::DeclareAttackers {
            attackers: vec![AttackerDeclaration {
                attacker: 1,
                defending: DefendingEntity::Player(1),
            }],
        };
        assert_eq!(a, b);
    }

    // --- ChoiceAction: Color vs ManaColor variants --------------------------

    #[test]
    fn choose_color_takes_a_color_not_mana_color() {
        // By design, "as ~ enters, choose a color" is always one of the 5.
        // A `ChoiceAction::ChooseColor(ManaColor::Colorless)` is a type
        // error — enforced by the compiler.
        let c = ChoiceAction::ChooseColor(Color::Red);
        assert_ne!(c, ChoiceAction::ChooseColor(Color::Blue));
    }

    #[test]
    fn choose_mana_color_can_be_colorless() {
        let c = ChoiceAction::ChooseManaColor(ManaColor::Colorless);
        assert_ne!(c, ChoiceAction::ChooseManaColor(ManaColor::Red));
    }

    #[test]
    fn distribute_and_order_carry_vec_fields() {
        let d = ChoiceAction::Distribute(vec![(1, 3), (2, 1)]);
        let d2 = ChoiceAction::Distribute(vec![(1, 3), (2, 1)]);
        assert_eq!(d, d2);

        // Order matters — different orderings are different choices.
        let different_order = ChoiceAction::Distribute(vec![(2, 1), (1, 3)]);
        assert_ne!(d, different_order);
    }

    // --- ManaPaymentPlan ----------------------------------------------------

    #[test]
    fn empty_payment_plan_is_empty() {
        let p = ManaPaymentPlan::empty();
        assert!(p.assignments.is_empty());
        assert!(p.convoke_creatures.is_empty());
        assert!(p.delve_cards.is_empty());
        assert!(p.phyrexian_life_payments.is_empty());
    }

    // --- DecisionContext / CastSubStep -------------------------------------

    #[test]
    fn decision_context_structural_equality() {
        let a = DecisionContext::CastingSpell {
            spell: 1,
            sub_step: CastSubStep::PayMana,
        };
        let b = DecisionContext::CastingSpell {
            spell: 1,
            sub_step: CastSubStep::PayMana,
        };
        assert_eq!(a, b);

        let c = DecisionContext::CastingSpell {
            spell: 1,
            sub_step: CastSubStep::ChooseTargets { clause_index: 0 },
        };
        assert_ne!(a, c);
    }

    // --- Serde roundtrip ----------------------------------------------------

    #[test]
    fn action_serializes_and_deserializes() {
        let a = Action::CastSpell {
            object_id: 7,
            targets: TargetSelection {
                targets: vec![TargetChoice::Player(1)],
            },
            modes: vec![],
            mana_payment: pay_nothing(),
            additional_costs: vec![AdditionalCostPayment::PayLife(2)],
            x_value: Some(3),
            cast_modifier: CastModifier::None,
            cost_reductions: CostReductions::default(),
        };
        let json = serde_json::to_string(&a).expect("serialize");
        let back: Action = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(a, back);
    }

    #[test]
    fn decision_context_roundtrip() {
        let ctx = DecisionContext::CastingSpell {
            spell: 3,
            sub_step: CastSubStep::ChooseTargets { clause_index: 1 },
        };
        let json = serde_json::to_string(&ctx).expect("serialize");
        let back: DecisionContext = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ctx, back);
    }

    #[test]
    fn choice_action_roundtrip() {
        for c in [
            ChoiceAction::ChooseObject(1),
            ChoiceAction::ChoosePlayer(0),
            ChoiceAction::ChooseColor(Color::Green),
            ChoiceAction::ChooseManaColor(ManaColor::Colorless),
            ChoiceAction::ChooseNumber(7),
            ChoiceAction::ChooseYesNo(true),
            ChoiceAction::ChooseOrdered(vec![0, 1, 2]),
            ChoiceAction::Distribute(vec![(1, 2), (3, 4)]),
            ChoiceAction::OrderCards(vec![10, 11, 12]),
        ] {
            let j = serde_json::to_string(&c).expect("ser");
            let back: ChoiceAction = serde_json::from_str(&j).expect("de");
            assert_eq!(c, back);
        }
    }
}

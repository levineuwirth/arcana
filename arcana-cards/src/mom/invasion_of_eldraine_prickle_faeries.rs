//! Invasion of Eldraine // Prickle Faeries
//!
//! Front: Battle — Siege {3}{B}, enters with 6 defense counters.
//!   When this Siege enters, target opponent discards two cards.
//!
//! Back: Creature — Faerie (Prickle Faeries)
//!   Flying
//!   At the beginning of each opponent's upkeep, if that player has two or fewer cards in hand,
//!   this creature deals 2 damage to them.
//!
//! Defeat-transform to the Faerie back face is auto-wired by the engine SBA
//! (CR 310.11). The back face's "at the beginning of each opponent's upkeep, if
//! that player has two or fewer cards in hand, deal 2 damage to them" is wired as
//! a StepBegins{Upkeep, Opponent} trigger face-gated to face 1, with an
//! intervening-if on the active player's hand size; Flying is intrinsic on the
//! back-face characteristics.
//!
//! Note: defense counter count = 6 per the card's printed defense.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Eldraine");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Prickle Faeries");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(faerie_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying],
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(back)
            // Front ETB: target opponent discards two cards.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                ],
            })
            // Back face: at the beginning of each opponent's upkeep, if that
            // player has two or fewer cards in hand, deal 2 damage to them.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: Some(active_opponent_low_hand),
                effect: back_ping_active_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1)
    )
}

fn etb_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![
        Effect::Discard { player: *p, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}

/// Intervening-if: the opponent whose upkeep this is (= the active player) has
/// two or fewer cards in hand.
fn active_opponent_low_hand(
    state: &GameState,
    _source: ObjectId,
    _controller: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    arcana_core::script::hand_size(state, state.active_player()) <= 2
}

/// "deals 2 damage to them" — the opponent whose upkeep it is (= the active
/// player; the StepBegins{whose: Opponent} gate guarantees it isn't us).
fn back_ping_active_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let them = state.active_player();
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(them),
        amount: 2,
    }]
}

//! Mondronen Shaman // Tovolar's Magehunter — `{3}{R}` Human Shaman Werewolf 3/2 (front) /
//! Werewolf (back). Transform werewolf.
//!
//! Front face:
//!   At the beginning of each upkeep, if no spells were cast last turn, transform.
//!
//! Back face (Tovolar's Magehunter):
//!   Whenever an opponent casts a spell, this creature deals 2 damage to that player.
//!   At the beginning of each upkeep, if a player cast two or more spells last turn, transform.
//!
//! GAP: "if no spells were cast last turn" and "if a player cast two or more spells last
//!      turn" werewolf transform conditions not expressible as TriggerCondition predicates;
//!      both upkeep triggers fire unconditionally.
//! GAP: back-face abilities (spell-damage trigger, upkeep back-transform) are authored here
//!      but are not auto-scoped to the back face; they fire on both faces. Ideally
//!      the spell-damage trigger should be gated to face 1 (back), but TriggeredAbilityDef
//!      has no face_gate field.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mondronen Shaman");
    let human_sub = reg.interner_mut().intern("Human");
    let shaman_sub = reg.interner_mut().intern("Shaman");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(shaman_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Tovolar's Magehunter — Werewolf
    let back_name = reg.interner_mut().intern("Tovolar's Magehunter");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: at beginning of each upkeep, if no spells were cast last turn, transform.
            // Intervening-if modeled via conditions::no_spells_cast_last_turn (CR 603.4).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_no_spells),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: whenever an opponent casts a spell, deal 2 damage to that player.
            // GAP: no face_gate on TriggeredAbilityDef; fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: on_opponent_spell,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: at beginning of each upkeep, if a player cast two or more spells last turn,
            // transform back. Intervening-if modeled via conditions::a_player_cast_two_or_more_last_turn.
            // GAP: no face_gate; fires on both faces.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(iif_two_or_more),
                effect: back_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn iif_no_spells(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn iif_two_or_more(state: &GameState, _source: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(state)
}

fn front_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn on_opponent_spell(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Deal 2 damage to the opponent who cast the spell.
    let Some(caster) = trig.triggering_caster() else { return Vec::new(); };
    vec![Effect::DealDamage {
        target: DamageTarget::Player(caster),
        amount: 2,
        source: trig.source,
    }]
}

fn back_upkeep_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire if a player cast two or more spells last turn.
    // GAP: this fires as a back-face ability; should only trigger on face 1.
    vec![Effect::Transform { target: trig.source }]
}

//! Huntmaster of the Fells // Ravager of the Fells — `{2}{R}{G}` Creature —
//! Human Werewolf 2/2 (red-green), transforming DFC.
//!
//! Front (Huntmaster of the Fells):
//!  * "Whenever this creature enters or transforms into Huntmaster of the
//!    Fells, create a 2/2 green Wolf creature token and you gain 2 life."
//!  * "At the beginning of each upkeep, if no spells were cast last turn,
//!    transform this creature."
//!
//! Back (Ravager of the Fells, 4/4 Werewolf, Trample):
//!  * "Whenever this creature transforms into Ravager of the Fells, it
//!    deals 2 damage to target opponent or planeswalker and 2 damage to
//!    up to one target creature that player or that planeswalker's
//!    controller controls."
//!  * "At the beginning of each upkeep, if a player cast two or more
//!    spells last turn, transform this creature."
//!
//! GAP: the back-face damage trigger's "opponent OR planeswalker" target
//! is modeled as a player target, and the "up to one creature that player
//! controls" rider is modeled as an unconstrained up-to-one creature
//! target (controller restriction not enforced).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::effects::{KeywordAbility, TokenDefinition};
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huntmaster of the Fells");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let _wolf_sub = reg.interner_mut().intern("Wolf");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(human_sub);
    front_subs.0.insert(werewolf_sub);

    let front_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ravager of the Fells");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(werewolf_back_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, front_chars)
            .with_transform_back(back)
            // Trigger 1 (front): enters or transforms into Huntmaster ->
            // create a Wolf token and gain 2 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::Custom(enters_or_becomes_huntmaster),
                intervening_if: None,
                effect: huntmaster_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 2 (front): each upkeep, if no spells cast last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_no_spells_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 3 (back): transforms into Ravager -> 2 damage to target
            // opponent/planeswalker, 2 damage to up to one target creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::Custom(transforms_into_ravager),
                intervening_if: None,
                effect: ravager_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Player,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
            })
            // Trigger 4 (back): each upkeep, if a player cast 2+ spells last turn, transform.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_player_cast_two_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Face gates: 1 & 2 only on the front (face 0); 3 & 4 only on the back (face 1).
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1)
            .with_trigger_face_gate(4, 1),
    )
}

/// Matches this creature entering the battlefield OR transforming (into the
/// front face — gated to face 0 so it only fires while showing Huntmaster).
fn enters_or_becomes_huntmaster(event: &GameEvent, _state: &GameState, source: ObjectId) -> bool {
    match event {
        GameEvent::EntersBattlefield { object_id, .. } => *object_id == source,
        GameEvent::Transformed { object_id } => *object_id == source,
        _ => false,
    }
}

/// Matches this creature transforming (gated to face 1 so it only fires
/// while showing Ravager of the Fells).
fn transforms_into_ravager(event: &GameEvent, _state: &GameState, source: ObjectId) -> bool {
    matches!(event, GameEvent::Transformed { object_id } if *object_id == source)
}

fn huntmaster_etb(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wolf_name = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subs = SubtypeSet::default();
    subs.0.insert(wolf_name);
    let wolf = TokenDefinition {
        name: wolf_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: subs,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: wolf,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn ravager_damage(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    let mut iter = trig.targets.targets.iter();
    // First target: opponent (or planeswalker — modeled as the player target).
    if let Some(t) = iter.next() {
        match t {
            TargetChoice::Player(p) => effects.push(Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Player(*p),
                amount: 2,
            }),
            TargetChoice::Object(id) => effects.push(Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Object(*id),
                amount: 2,
            }),
            _ => {}
        }
    }
    // Second target: up to one creature.
    if let Some(TargetChoice::Object(id)) = iter.next() {
        effects.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Object(*id),
            amount: 2,
        });
    }
    effects
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}

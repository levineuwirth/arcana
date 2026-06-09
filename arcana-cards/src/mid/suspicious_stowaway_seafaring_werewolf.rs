//! Suspicious Stowaway // Seafaring Werewolf (transforming DFC, layout "transform")
//!
//! Front face: Suspicious Stowaway — {1}{U} Creature — Human Rogue Werewolf, 1/1.
//!   This creature can't be blocked. (static, own id, while on battlefield)
//!   Whenever this creature deals combat damage to a player, draw a card, then discard a card.
//!   Daybound (if a player casts no spells during their own turn, it becomes night next turn).
//! Back face: Seafaring Werewolf — Creature — Werewolf, 1/2.
//!   This creature can't be blocked.
//!   Whenever this creature deals combat damage to a player, draw a card.
//!   Nightbound (if a player casts at least two spells during their own turn, it becomes day next turn).
//!
//! GAP: Daybound/Nightbound day-night state cycling is modeled here as the werewolf
//!   transform pair driven by intervening-if (no-spells / two-or-more-spells last turn),
//!   approximating the day/night flip. The strict per-turn day/night ratchet of
//!   Daybound/Nightbound is not separately tracked.

use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suspicious Stowaway");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let werewolf = reg.interner_mut().intern("Werewolf");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(human);
    front_subtypes.0.insert(rogue);
    front_subtypes.0.insert(werewolf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Seafaring Werewolf");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front "this creature can't be blocked" — static on its own id (ETB seeds it).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: cant_be_blocked_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front combat-damage trigger: draw a card, then discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: front_draw_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front Daybound: at the beginning of each upkeep, if no spells were cast
            // last turn, transform (becomes night / back face).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
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
            // Back combat-damage trigger: draw a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: back_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back Nightbound: at the beginning of each upkeep, if a player cast two or
            // more spells last turn, transform (becomes day / front face).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
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
            // Face gating: 1/2/3 front-only, 4/5 back-only.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 0)
            .with_trigger_face_gate(4, 1)
            .with_trigger_face_gate(5, 1),
    )
}

fn cant_be_blocked_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn front_draw_discard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn back_draw(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}

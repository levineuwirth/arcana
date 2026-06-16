//! Kaito, Cunning Infiltrator — `{1}{U}{U}` Legendary Planeswalker — Kaito,
//! starting loyalty 3.
//! Whenever a creature you control deals combat damage to a player, put a
//!   loyalty counter on Kaito.
//! +1: Up to one target creature you control can't be blocked this turn. Draw
//!   a card, then discard a card.
//! −2: Create a 2/1 blue Ninja creature token.
//! −9: You get an emblem with "Whenever a player casts a spell, you create a
//!   2/1 blue Ninja creature token."
//!
//! GAP: the +1 "can't be blocked this turn" rider (and its up-to-one target)
//!   is not expressible with the demonstrated Effect surface; only the
//!   draw-then-discard is emitted.
//! GAP: the −9 emblem is not in the demonstrated Effect catalog.

use arcana_core::effects::{DiscardChoice, Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaito, Cunning Infiltrator");
    let kaito = reg.interner_mut().intern("Kaito");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaito);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever a creature you control deals combat damage to a player,
            // put a loyalty counter on Kaito.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_loyalty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // +1: ... can't be blocked this turn. Draw a card, then discard a card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature you control can't be blocked this turn. Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_loot,
            })
            // −2: Create a 2/1 blue Ninja creature token.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 2/1 blue Ninja creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_ninja,
            })
            // −9: emblem — GAP
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: You get an emblem with \"Whenever a player casts a spell, you create a 2/1 blue Ninja creature token.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_emblem,
            }),
    )
}

fn combat_damage_loyalty(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Loyalty,
        count: 1,
    }]
}

fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "up to one target creature you control can't be blocked this turn"
    // is not expressible; only the draw-then-discard is emitted.
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn minus_two_ninja(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ninja = reg.interner().lookup("Ninja").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ninja);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: ninja,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn minus_nine_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not in the demonstrated Effect catalog.
    Vec::new()
}

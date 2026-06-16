//! Huatli, Radiant Champion — `{2}{G}{W}` Legendary Planeswalker — Huatli,
//! starting loyalty 3.
//!
//! +1: Put a loyalty counter on Huatli for each creature you control.
//! −1: Target creature gets +X/+X until end of turn, where X is the number of
//!     creatures you control.
//! −8: You get an emblem with "Whenever a creature you control enters, you may
//!     draw a card."
//!
//! Note: the printed +1 loyalty symbol pays the activation cost; the ability
//!   then places an ADDITIONAL loyalty counter for each creature you control.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Huatli, Radiant Champion");
    let huatli = reg.interner_mut().intern("Huatli");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(huatli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a loyalty counter on Huatli for each creature \
                       you control."
                    .into(),
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
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Target creature gets +X/+X until end of turn, where X \
                       is the number of creatures you control."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever a creature you \
                       control enters, you may draw a card.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn creatures_you_control(state: &GameState, you: arcana_core::types::PlayerId) -> u32 {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    script::count_matching(state, &filter, you)
}

fn plus_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = creatures_you_control(state, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Loyalty,
        count: n,
    }]
}

fn minus_one(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let x = creatures_you_control(state, ctx.controller) as i32;
    vec![Effect::Pump {
        target: *id,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Huatli, Radiant Champion")
        .unwrap_or_default();
    let creature_you =
        ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let emblem = EmblemDefinition {
        name: emblem_name,
        abilities: vec![TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: creature_you,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: emblem_draw,
            trigger_zones: vec![Zone::Command],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![],
        }],
    };
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem,
    }]
}

fn emblem_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

//! Kumena, Tyrant of Orazca — `{1}{G}{U}` 2/4 Legendary Merfolk Shaman.
//! "Tap another untapped Merfolk you control: Kumena can't be blocked this turn."
//! "Tap three untapped Merfolk you control: Draw a card."
//! "Tap five untapped Merfolk you control: Put a +1/+1 counter on each Merfolk
//! you control."
//!
//! Three activated abilities, each paying by tapping N untapped Merfolk you
//! control (tap_other / tap_other_count; the engine excludes Kumena itself).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kumena, Tyrant of Orazca");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(shaman);

    let merfolk_filter = script::subtype_filter(reg, "Merfolk");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap another untapped Merfolk you control: Kumena can't be blocked this turn.".into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter.clone()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: self_unblockable,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap three untapped Merfolk you control: Draw a card.".into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter.clone()),
                    tap_other_count: 3,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Tap five untapped Merfolk you control: Put a +1/+1 counter on each Merfolk you control.".into(),
                cost: ActivationCost {
                    tap_other: Some(merfolk_filter),
                    tap_other_count: 5,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: counter_each_merfolk,
            }),
    )
}

fn self_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn counter_each_merfolk(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Merfolk").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }),
    }]
}

//! Possessed Goat — `{W}` 1/1 white Goat.
//! `{3}, Discard a card: Put three +1/+1 counters on this creature and it becomes a black Demon in addition to its other colors and types. Activate only once.`
//! The Demon subtype-add is a targeted continuous effect (ContinuousEffect::add_subtypes,
//! Duration::Permanent).
//! GAP: ActivationCost has no "discard a card" cost field. "Becomes black in addition
//! to its other colors" not modeled (only the attached color-add exists).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Possessed Goat");
    let goat = reg.interner_mut().intern("Goat");
    let _demon = reg.interner_mut().intern("Demon"); // looked up in become_demon
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, Discard a card: Put three +1/+1 counters on this creature and it becomes a black Demon in addition to its other colors and types. Activate only once.".into(),
                // GAP: no "discard a card" cost field; using mana only
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_demon,
            }),
    )
}

fn become_demon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut subs = SubtypeSet::default();
    if let Some(demon) = reg.interner().lookup("Demon") {
        subs.0.insert(demon);
    }
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 3,
        },
        // "…and it becomes a Demon in addition to its other types" —
        // Duration::Permanent (no stated duration).
        // GAP: "black in addition to its other colors" not modeled (no
        // targeted color-add continuous effect).
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::add_subtypes(
                ctx.source,
                ctx.source,
                subs,
                Duration::Permanent,
            ),
        },
    ]
}

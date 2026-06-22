//! Genasi Rabble-Rouser — `{1}{R}` 1/3 Creature — Elemental Shaman.
//! Double team (Alchemy — conjure a duplicate on attack).
//! {1}{R}: Creatures you control named Genasi Rabble-Rouser get +1/+0 until
//! end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Genasi Rabble-Rouser");
    let elemental = reg.interner_mut().intern("Elemental");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Double team" and "Conjure" are Alchemy-only keywords not in the
        // supported KeywordAbility surface; emitted as no keywords.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{R}: Creatures you control named Genasi Rabble-Rouser get +1/+0 until end of turn."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_named,
        }),
    )
}

fn pump_named(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    // Creatures you control named "Genasi Rabble-Rouser".
    let nm = reg.interner().lookup("Genasi Rabble-Rouser");
    let filter = ObjectFilter {
        name: nm,
        ..ObjectFilter::creature().controlled_by(ControllerConstraint::You)
    };
    let ids = script::ids_matching(state, &filter, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}

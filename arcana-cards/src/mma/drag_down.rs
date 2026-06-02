//! Drag Down — `{2}{B}` instant. "Domain — Target creature gets -1/-1
//! until end of turn for each basic land type among lands you control."
//!
//! Domain counts the number of DISTINCT basic land types (Plains,
//! Island, Swamp, Mountain, Forest) present among the lands you
//! control. We compute it by checking, for each of the five basic land
//! subtypes, whether you control at least one land with that subtype
//! (via `script::count_matching` over a subtype-filtered land filter)
//! and summing the present types. The result drives a single
//! `Effect::Pump` of -N/-N until end of turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drag Down");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Domain — Target creature gets -1/-1 until end of turn for each basic land type among lands you control.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };

    let mut domain: i32 = 0;
    for ty in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let filter = script::subtype_filter(reg, ty)
            .with_types(TypeLine::LAND.into())
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, entry.controller) > 0 {
            domain += 1;
        }
    }

    vec![Effect::Pump {
        target: *id,
        power: -domain,
        toughness: -domain,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

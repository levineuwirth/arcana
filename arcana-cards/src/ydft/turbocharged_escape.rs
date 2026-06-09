//! Turbocharged Escape — `{3}{W}{W}` sorcery. "Destroy all non-Vehicle
//! creatures. Then choose a Vehicle you control. It perpetually becomes an
//! artifact creature."
//! The non-Vehicle sweep is expressed with `ObjectFilter::without_subtype_sym`.
//! GAP: the "perpetually becomes an artifact creature" effect is not in the
//! Effect catalog; only the sweep is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Turbocharged Escape");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all non-Vehicle creatures. Then choose a Vehicle you control. It perpetually becomes an artifact creature.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Non-Vehicle creatures ("Vehicle" interned in register; on a failed
    // lookup skip the exclusion).
    // GAP: perpetually-becomes-artifact-creature effect not in catalog
    let mut filter = ObjectFilter::creature();
    if let Some(vehicle) = reg.interner().lookup("Vehicle") {
        filter = filter.without_subtype_sym(vehicle);
    }
    let ids = script::ids_matching(state, &filter, entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}

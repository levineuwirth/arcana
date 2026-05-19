//! Turbocharged Escape — `{3}{W}{W}` sorcery. "Destroy all non-Vehicle
//! creatures. Then choose a Vehicle you control. It perpetually becomes an
//! artifact creature."
//! GAP: Filtering non-Vehicle creatures (subtype exclusion for Vehicle) and
//! the "perpetually becomes an artifact creature" effect are not in the
//! Effect catalog. Best effort: destroy all creatures (full wipe), Vehicle
//! exclusion and perpetual effect are GAPs.

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
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: non-Vehicle subtype exclusion filter not supported; destroys all creatures
    // GAP: perpetually-becomes-artifact-creature effect not in catalog
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    }]
}

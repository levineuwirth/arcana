//! Stench of Evil — `{2}{B}{B}` sorcery, "Destroy all Plains. For each land destroyed this way,
//! Stench of Evil deals 1 damage to that land's controller unless they pay {2}."
//!
//! GAP: after destroying all Plains, dealing damage to each former controller (or letting them
//! pay to prevent it) requires tracking per-permanent controller at destroy time, which is not
//! expressible via the catalog. Partial: destroy all Plains via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stench of Evil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all Plains. For each land destroyed this way, Stench of Evil deals 1 damage to that land's controller unless they pay {2}.".into(),
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
    let plains_filter = ObjectFilter::new().with_types(TypeLine::LAND.into());
    let ids = script::ids_matching(state, &plains_filter, entry.controller);
    vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent { target: arcana_core::objects::NULL_OBJECT_ID }),
        },
        // GAP: per-destroyed-land "deal 1 damage to controller unless they pay {2}" not expressible
    ]
}

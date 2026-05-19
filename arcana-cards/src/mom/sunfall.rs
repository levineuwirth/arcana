//! Sunfall — `{3}{W}{W}` sorcery, "Exile all creatures. Incubate X, where
//! X is the number of creatures exiled this way."
//! GAP: Incubate mechanic (create Incubator token with X +1/+1 counters
//! and transform ability) is not in the Effect catalog.

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
    let name = reg.interner_mut().intern("Sunfall");
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
                text: "Exile all creatures. Incubate X, where X is the number of creatures exiled this way.".into(),
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
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    // GAP: Incubate X (create Incubator token with X counters)
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ExilePermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}

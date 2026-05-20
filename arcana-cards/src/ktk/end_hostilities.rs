//! End Hostilities — `{3}{W}{W}` sorcery. "Destroy all creatures and
//! all permanents attached to creatures."

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
    let name = reg.interner_mut().intern("End Hostilities");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures and all permanents attached to \
                   creatures."
                .into(),
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
    // "permanents attached to creatures" (auras/equipment) is not an
    // ObjectFilter refinement — best-effort destroys all creatures.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        entry.controller,
    );
    ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}

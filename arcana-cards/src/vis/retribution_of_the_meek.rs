//! Retribution of the Meek — `{2}{W}` sorcery. Destroy all creatures
//! with power 4 or greater. They can't be regenerated. (Regen-block
//! rider not modeled.)

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
    let name = reg.interner_mut().intern("Retribution of the Meek");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Destroy all creatures with power 4 or greater. They can't be regenerated.".into(),
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
    // GAP: "they can't be regenerated" rider not modeled.
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().with_min_power(4),
        entry.controller,
    );
    ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect()
}

//! Paraselene — `{2}{W}` sorcery, "Destroy all enchantments. You gain 1 life
//! for each enchantment destroyed this way."

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
    let name = reg.interner_mut().intern("Paraselene");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Destroy all enchantments. You gain 1 life for each enchantment destroyed this way.".into(),
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
    let filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());
    let ids = script::ids_matching(state, &filter, entry.controller);
    let count = ids.len() as u32;
    let mut effects: Vec<Effect> = ids.into_iter()
        .map(|id| Effect::DestroyPermanent { target: id })
        .collect();
    if count > 0 {
        effects.push(Effect::GainLife { player: entry.controller, amount: count });
    }
    effects
}

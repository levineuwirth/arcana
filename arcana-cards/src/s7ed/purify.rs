//! Purify — `{3}{W}{W}` sorcery. "Destroy all artifacts and all enchantments."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Purify");
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
                text: "Destroy all artifacts and all enchantments.".into(),
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
    let artifact_filter = ObjectFilter::new().with_types(TypeLine::ARTIFACT.into());
    let artifact_ids = script::ids_matching(state, &artifact_filter, entry.controller);

    let enchantment_filter = ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into());
    let enchantment_ids = script::ids_matching(state, &enchantment_filter, entry.controller);

    vec![
        Effect::ForEach {
            targets: artifact_ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
        Effect::ForEach {
            targets: enchantment_ids,
            effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
        },
    ]
}

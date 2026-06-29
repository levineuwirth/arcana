//! Triumphant Reckoning — `{6}{W}{W}{W}` sorcery, "Return all artifact, enchantment, and
//! planeswalker cards from your graveyard to the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triumphant Reckoning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Return all artifact, enchantment, and planeswalker cards from your graveyard to the battlefield.".into(),
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
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT | TypeLine::PLANESWALKER));
    let ids = script::graveyard_ids_matching(state, &filter, entry.controller, entry.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::ReturnFromGraveyardToBattlefield {
            target: NULL_OBJECT_ID,
        }),
    }]
}

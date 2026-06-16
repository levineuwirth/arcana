//! Eerie Procession — `{2}{U}` sorcery — Arcane. "Search your library
//! for an Arcane card, reveal that card, put it into your hand, then
//! shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eerie Procession");
    let _arcane = reg.interner_mut().intern("Arcane");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an Arcane card, reveal that card, put it into your hand, then shuffle.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Arcane" is a subtype; subtype_filter selects cards with it.
    let filter = arcana_core::script::subtype_filter(reg, "Arcane");
    let _ = &entry;
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter,
        reveal: true,
    }]
}

//! Mystical Tutor — `{U}` instant. "Search your library for an instant or
//! sorcery card, reveal it, then shuffle and put that card on top."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystical Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an instant or sorcery card, reveal it, then shuffle and put that card on top.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // TutorToHand with reveal: true is the closest match; the card goes to hand not top of library.
    // GAP: "put that card on top of library" (topdeck tutor) — TutorToHand goes to hand instead.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types_any(TypeLine::INSTANT.into())
            .with_types_any(TypeLine::SORCERY.into()),
        reveal: true,
    }]
}

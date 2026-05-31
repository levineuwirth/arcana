//! Gift of Estates — `{1}{W}` sorcery, "If an opponent controls more
//! lands than you, search your library for up to three Plains cards,
//! reveal them, put them into your hand, then shuffle."
//!
//! The "up to three Plains cards" search is modelled as three
//! tutor-to-hand searches for a card with the Plains subtype.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gift of Estates");
    let _plains = reg.interner_mut().intern("Plains");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "If an opponent controls more lands than you, search your library for up to three Plains cards, reveal them, put them into your hand, then shuffle."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "if an opponent controls more lands than you" intervening
    // gate has no expressible condition helper (no opponent-vs-you land
    // comparison in `script` / `conditions`), so the search runs
    // unconditionally rather than being gated.
    let Some(plains) = reg.interner().lookup("Plains") else { return Vec::new(); };
    let filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtypes_any(vec![plains]);
    vec![
        Effect::TutorToHand { player: entry.controller, filter: filter.clone(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: filter.clone(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter, reveal: true },
    ]
}

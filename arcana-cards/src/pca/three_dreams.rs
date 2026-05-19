//! Three Dreams — `{4}{W}` sorcery. "Search your library for up to
//! three Aura cards with different names, reveal them, put them into
//! your hand, then shuffle."
//!
//! GAP: "up to three Aura cards with different names" filter (Aura
//! subtype, distinct names) is not expressible via TutorToHand which
//! supports a single ObjectFilter and one card. Best-effort: single
//! tutor with enchantment type filter.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Three Dreams");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to three Aura cards with different names, reveal them, put them into your hand, then shuffle.".into(),
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
    // GAP: up-to-three distinct Aura search (only single-filter TutorToHand available)
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ENCHANTMENT.into()),
        reveal: true,
    }]
}

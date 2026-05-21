//! Three Dreams — `{4}{W}` sorcery. "Search your library for up to
//! three Aura cards with different names, reveal them, put them into
//! your hand, then shuffle."
//!
//! TutorToHand searches for a single card; there is no primitive for
//! searching out up to three cards with distinct names. Modeled as a
//! single Aura tutor, with the multi-fetch a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for up to three Aura cards with different names, reveal them, put them into your hand, then shuffle.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: searching for up to three Aura cards with distinct names is
    // not expressible; only a single Aura tutor is emitted.
    let aura = arcana_core::script::subtype_filter(reg, "Aura");
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: aura,
        reveal: true,
    }]
}

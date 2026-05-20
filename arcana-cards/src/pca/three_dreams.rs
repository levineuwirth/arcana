//! Three Dreams — `{4}{W}` sorcery. "Search your library for up to
//! three Aura cards with different names, reveal them, put them into
//! your hand, then shuffle."
//!
//! Modeled as a single Aura tutor. GAP: 'up to three with different
//! names' isn't a TutorToHand option (single card per call).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Three Dreams");
    let _aura = reg.interner_mut().intern("Aura");
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
    let aura_filter = script::subtype_filter(reg, "Aura");
    // GAP: 'up to three different-named' — TutorToHand resolves one
    // card per call; we tutor a single Aura.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: aura_filter,
        reveal: true,
    }]
}

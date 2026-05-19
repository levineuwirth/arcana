//! Open the Armory — `{1}{W}` sorcery. "Search your library for an Aura
//! or Equipment card, reveal it, put it into your hand, then shuffle."
//!
//! # GAP
//! Cannot express `ObjectFilter` for "Aura OR Equipment" (subtype union);
//! best-effort tutors for Equipment only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Open the Armory");
    let _equipment = reg.interner_mut().intern("Equipment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an Aura or Equipment card, reveal it, put it into your hand, then shuffle.".into(),
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
    // GAP: cannot express Aura OR Equipment in one ObjectFilter; tutoring Equipment only
    let filter = script::subtype_filter(reg, "Equipment");
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter,
        reveal: true,
    }]
}

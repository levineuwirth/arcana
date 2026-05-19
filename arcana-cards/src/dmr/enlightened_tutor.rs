//! Enlightened Tutor — `{W}` instant.
//! "Search your library for an artifact or enchantment card, reveal it, then
//! shuffle and put that card on top."
//! GAP: TutorToHand puts to hand; TutorToBattlefield puts to battlefield.
//! Neither puts the card on top of library. Using TutorToHand as best effort;
//! the 'put on top' destination is a GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enlightened Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an artifact or enchantment card, reveal it, then shuffle and put that card on top.".into(),
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
    // GAP: no 'put on top of library' tutor destination; using TutorToHand as best effort
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT)),
        reveal: true,
    }]
}

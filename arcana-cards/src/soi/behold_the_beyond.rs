//! Behold the Beyond — `{5}{B}{B}` sorcery. "Discard your hand. Search your
//! library for three cards, put them into your hand, then shuffle."
//!
//! # Note: TutorToHand is used three times (repeated Effect) to represent
//! searching for three cards. The discard-all-hand is modeled via hand_size.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Behold the Beyond");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard your hand. Search your library for three cards, put them into your hand, then shuffle.".into(),
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
    use arcana_core::script;
    let hand = script::hand_size(state, entry.controller);
    let mut effects = Vec::new();
    effects.push(Effect::Discard {
        player: entry.controller,
        count: hand,
        choice: DiscardChoice::ControllerChooses,
    });
    // Search for three cards — repeat TutorToHand three times
    let filter = ObjectFilter::permanent();
    effects.push(Effect::TutorToHand { player: entry.controller, filter: filter.clone(), reveal: false });
    effects.push(Effect::TutorToHand { player: entry.controller, filter: filter.clone(), reveal: false });
    effects.push(Effect::TutorToHand { player: entry.controller, filter, reveal: false });
    effects
}

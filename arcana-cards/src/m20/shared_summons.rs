//! Shared Summons — `{3}{G}{G}` instant. "Search your library for up to
//! two creature cards with different names, reveal them, put them into
//! your hand, then shuffle."
//!
//! GAP: searching for two creature cards with different names in a single
//! effect is not expressible (TutorToHand fetches one card). Approximated
//! as two separate TutorToHand effects; the 'different names' constraint
//! is not enforced.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::targets::ObjectFilter;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shared Summons");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to two creature cards with different names, reveal them, put them into your hand, then shuffle.".into(),
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
    // GAP: 'different names' constraint not expressible; two separate tutors used
    vec![
        Effect::TutorToHand { player: entry.controller, filter: ObjectFilter::creature(), reveal: true },
        Effect::TutorToHand { player: entry.controller, filter: ObjectFilter::creature(), reveal: true },
    ]
}

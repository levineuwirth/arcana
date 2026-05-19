//! Adventure Awaits — `{1}{G}` sorcery. "Look at the top five cards of your
//! library. You may reveal a creature card from among them and put it into
//! your hand. Put the rest on the bottom of your library in a random order.
//! If you didn't put a card into your hand this way, draw a card."
//!
//! Best effort: TutorToHand searches and draws the creature (reveal: true),
//! which covers the primary branch. The 'put the rest on the bottom in random
//! order' and the 'if you didn't find one, draw a card' branches are GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adventure Awaits");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top five cards of your library. You may reveal a creature card from among them and put it into your hand. Put the rest on the bottom of your library in a random order. If you didn't put a card into your hand this way, draw a card.".into(),
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
    // GAP: look at top 5 (not full library tutor), conditional draw if no
    //      creature found, and random-order bottom placement not expressible.
    // Best effort: tutor a creature card to hand (reveal true).
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::creature(),
        reveal: true,
    }]
}

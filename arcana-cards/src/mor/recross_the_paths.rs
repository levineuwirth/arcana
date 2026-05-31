//! Recross the Paths — `{2}{G}` sorcery. "Reveal cards from the top
//! of your library until you reveal a land card. Put that card onto
//! the battlefield and the rest on the bottom of your library in any
//! order. Clash with an opponent. If you win, return Recross the
//! Paths to its owner's hand."
//!
//! The reveal-until-land + put-onto-battlefield half is expressed with
//! `Effect::RevealUntil`. The Clash half (and its win-conditional
//! return-to-hand) is not an expressible primitive and is GAP-ed.

use arcana_core::effects::{DigRest, Effect, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Recross the Paths");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        // GAP: Clash keyword is not an expressible keyword for this card class.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Reveal cards from the top of your library until you reveal a land card. Put that card onto the battlefield and the rest on the bottom of your library in any order. Clash with an opponent. If you win, return Recross the Paths to its owner's hand.".into(),
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
    // GAP: "Clash with an opponent. If you win, return this to its owner's
    // hand." — Clash is not an expressible primitive.
    vec![Effect::RevealUntil {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: None,
    }]
}

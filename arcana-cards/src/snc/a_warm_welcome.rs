//! A-Warm Welcome — `{3}{G}` sorcery.
//! "Look at the top five cards of your library. You may reveal a creature card
//! from among them and put it into your hand. Put the rest on the bottom of
//! your library in a random order. Create two 1/1 green and white Citizen
//! creature tokens."
//!
//! # GAP: look-at-top-N / optional reveal creature to hand / rest to bottom
//! random — no Effect variant for this selection pattern. The token creation
//! is expressible; the library manipulation is not.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Warm Welcome");
    let _citizen = reg.interner_mut().intern("Citizen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top five cards of your library. You may reveal a creature card from among them and put it into your hand. Put the rest on the bottom of your library in a random order. Create two 1/1 green and white Citizen creature tokens.".into(),
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
    // GAP: look at top 5, optional reveal creature to hand, rest to bottom random
    let citizen = reg.interner().lookup("Citizen").expect("Citizen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(citizen);
    let token = TokenDefinition {
        name: citizen,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}

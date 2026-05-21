//! Warm Welcome — `{2}{G}` instant. "Look at the top five cards of
//! your library. You may reveal a creature card from among them and
//! put it into your hand. Put the rest on the bottom of your library
//! in a random order. Create a 1/1 green and white Citizen creature
//! token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warm Welcome");
    let _citizen = reg.interner_mut().intern("Citizen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top five cards of your library. You may reveal a creature card from among them and put it into your hand. Put the rest on the bottom of your library in a random order. Create a 1/1 green and white Citizen creature token.".into(),
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
    let citizen = reg.interner().lookup("Citizen")
        .expect("Citizen interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(citizen);
    let token = TokenDefinition {
        name: citizen,
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: look-at-top-five then optionally reveal-and-take a creature
    // card (rest to bottom) is not expressible — no dig-from-top
    // effect. The token creation is emitted.
    vec![Effect::CreateToken { controller: entry.controller, token }]
}

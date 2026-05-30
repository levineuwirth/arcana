//! Decorated Knight // Present Arms
//! Creature face: `{3}{U}` Legendary Creature — Human Knight 3/3
//! Whenever Decorated Knight attacks, draw a card from your original deck if it's outside the game.
//! Adventure face: "Present Arms" `{2}{U}` Sorcery — Exchange your library with another deck
//! you own from outside the game. Shuffle your library.
//!
//! GAP: Creature trigger "draw a card from your original deck if it's outside the game" —
//!      "outside the game" zone is not modeled in the engine (no Zone::SideBoard or similar).
//! GAP: Adventure effect "exchange your library with another deck you own from outside the game" —
//!      "outside the game" zone is not modeled; no Effect variant for library exchange.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Decorated Knight");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "Whenever Decorated Knight attacks, draw a card from your original deck if it's
        //      outside the game." — Zone::OutsideGame / sideboard not modeled.
        ..Default::default()
    };

    // Adventure face: Present Arms
    let adv_name = reg.interner_mut().intern("Present Arms");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let adv_ability = SpellAbilityDef {
        text: "Exchange your library with another deck you own from outside the game. Shuffle your library.".into(),
        target_requirements: vec![],
        modal: None,
        effect: present_arms_resolve,
    };

    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn present_arms_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Exchange your library with another deck you own from outside the game" —
    //      Zone::OutsideGame / sideboard not modeled; no Effect::ExchangeLibrary variant.
    Vec::new()
}

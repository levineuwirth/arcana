//! Jadzi, Oracle of Arcavios // Journey to the Oracle
//!
//! Front: Legendary Creature — Human Wizard {6}{U}{U} 5/5 (blue)
//!   Discard a card: Return Jadzi to its owner's hand.
//!   Magecraft — Whenever you cast or copy an instant or sorcery spell, reveal the top card of your library. If nonland, you may cast it by paying {1}. If land, put it onto the battlefield.
//! Back: Sorcery
//!   You may put any number of land cards from your hand onto the battlefield. Then if you control eight or more lands, you may discard a card. If you do, return Journey to the Oracle to its owner's hand.
//! GAP: Magecraft trigger not modeled
//! GAP: Back face land-batch deployment not modeled
//! GAP: Self-return to hand in back face not modeled

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jadzi, Oracle of Arcavios");
    let back_name = reg.interner_mut().intern("Journey to the Oracle");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(human);
    subtypes.insert(wizard);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let back_ability = SpellAbilityDef {
        text: "You may put any number of land cards from your hand onto the battlefield. Then if you control eight or more lands, you may discard a card. If you do, return Journey to the Oracle to its owner's hand.".into(),
        target_requirements: vec![],
        modal: None,
        effect: back_resolve,
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: Some(back_ability),
        }),
    )
}

fn back_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Put any number of land cards from hand to battlefield" not in Effect catalog
    Vec::new()
}

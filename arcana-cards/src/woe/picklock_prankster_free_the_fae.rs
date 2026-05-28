//! Picklock Prankster // Free the Fae — `{1}{U}` / `{1}{U}` Adventure
//!
//! Creature: `{1}{U}` Creature — Faerie Rogue (1/3)
//!   Flying, vigilance.
//!
//! Adventure: `{1}{U}` Instant — Free the Fae
//!   Mill four cards. Then put an instant, sorcery, or Faerie card from
//!   among them into your hand. (GAP: "from among milled cards" retrieval
//!   not expressible; emitting only Mill.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Picklock Prankster");
    let faerie_sub = reg.interner_mut().intern("Faerie");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie_sub);
    subtypes.0.insert(rogue_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Free the Fae");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Mill four cards. Then put an instant, sorcery, or Faerie card from among them into your hand.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "put a card from among milled cards into hand" not expressible
    vec![Effect::Mill { player: entry.controller, count: 4 }]
}

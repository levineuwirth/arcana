//! Beluna Grandsquall // Seek Thrills — `{G}{U}{R}` / `{2}{G}{U}{R}` Adventure
//!
//! Creature: `{G}{U}{R}` Legendary Creature — Giant Noble (4/4)
//!   Trample.
//!   Permanent spells you cast that have an Adventure cost {1} less. (GAP.)
//!
//! Adventure: `{2}{G}{U}{R}` Instant — Seek Thrills
//!   Mill seven cards. Then put all Adventure cards from among them into your
//!   hand. (GAP: "Adventure cards from milled" retrieval not expressible;
//!   emitting only Mill.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beluna Grandsquall");
    let giant_sub = reg.interner_mut().intern("Giant");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    subtypes.0.insert(noble_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Trample],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Seek Thrills");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}{R}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Mill seven cards. Then put all Adventure cards from among them into your hand.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace { name: adv_name, characteristics: adv_chars, spell_ability: Some(adv_ability) };
    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Adventure cards from milled" retrieval not expressible
    vec![Effect::Mill { player: entry.controller, count: 7 }]
}

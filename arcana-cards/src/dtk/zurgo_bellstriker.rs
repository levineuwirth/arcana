//! Zurgo Bellstriker — `{R}` 2/2 Legendary Orc Warrior.
//! "Zurgo can't block creatures with power 2 or greater." (static block
//!  restriction — not expressible.)
//! "Dash {1}{R}" (Dash is not in the usable KeywordAbility surface.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zurgo Bellstriker");
    let orc = reg.interner_mut().intern("Orc");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Zurgo can't block creatures with power 2 or greater." — static
    //       power-gated block restriction, no expressible surface.
    // GAP: "Dash {1}{R}" — Dash is not among the usable KeywordAbility variants.
    reg.register(CardDefinition::new(name, chars))
}

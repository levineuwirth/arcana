//! Solemn Doomguide — `{3}{B}{B}` 4/5 black Tiefling Cleric with Flying.
//!
//! Oracle:
//! * "Flying" — keyword, on the characteristics.
//! * "Each creature card in your graveyard that's a Cleric, Rogue, Warrior,
//!   and/or Wizard has unearth {1}{B}." — a STATIC continuous ability that
//!   grants the unearth keyword-ability to OTHER cards in a graveyard. It has
//!   no trigger word and no activation cost on this card, and there is no
//!   demonstrated primitive to confer "unearth {cost}" onto a set of
//!   graveyard cards. GAP'd; Flying is the only expressible piece.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static ability "Each creature card in your graveyard that's a Cleric,
// Rogue, Warrior, and/or Wizard has unearth {1}{B}." — confers a parametrized
// keyword onto a filtered set of graveyard cards. Not a triggered/activated
// ability of this card, and no demonstrated Effect / mechanic grants unearth.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solemn Doomguide");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

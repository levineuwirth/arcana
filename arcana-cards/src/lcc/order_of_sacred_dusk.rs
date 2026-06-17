//! Order of Sacred Dusk — `{6}{W}{B}` 5/5 Vampire Knight.
//! Convoke; Flying, lifelink, haste; Exalted; other Vampires you control
//! have exalted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Order of Sacred Dusk");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Convoke is not in the usable KeywordAbility surface.
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Lifelink,
            KeywordAbility::Haste,
            KeywordAbility::Exalted,
        ],
        ..Default::default()
    };

    // GAP: static "Other Vampires you control have exalted" — keyword-granting
    // anthem is a continuous ability, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}

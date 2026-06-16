//! Rograkh, Son of Rohgahh — `{0}` 0/1 Legendary Kobold Warrior.
//! First strike, menace, trample.
//! Partner — a commander-deck-construction keyword not in the usable surface (GAP'd).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rograkh, Son of Rohgahh");
    let kobold = reg.interner_mut().intern("Kobold");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kobold);
    subtypes.0.insert(warrior);

    // GAP: Partner — commander deck-construction keyword, not in usable surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Menace,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

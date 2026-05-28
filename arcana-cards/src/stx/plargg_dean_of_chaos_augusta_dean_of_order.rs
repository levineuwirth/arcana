//! Plargg, Dean of Chaos // Augusta, Dean of Order
//!
//! Front: Legendary Creature — Orc Shaman {1}{R} 2/2 (red)
//!   {T}, Discard a card: Draw a card.
//!   {4}{R}, {T}: Reveal cards from the top until nonlegendary nonland card with mana value 3 or less; may cast for free.
//! Back: Legendary Creature — Human Cleric
//!   Other tapped creatures you control get +1/+0.
//!   Other untapped creatures you control get +0/+1.
//!   Whenever you attack, untap each creature you control, then tap any number of creatures you control.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: Activated abilities not modeled
//! GAP: Static P/T boost based on tapped/untapped state not modeled
//! GAP: Attack trigger with untap/tap effects not modeled

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plargg, Dean of Chaos");
    let back_name = reg.interner_mut().intern("Augusta, Dean of Order");
    let orc = reg.interner_mut().intern("Orc");
    let shaman = reg.interner_mut().intern("Shaman");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(orc);
    subtypes.insert(shaman);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::new();
    back_subtypes.insert(human);
    back_subtypes.insert(cleric);

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: back_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: None,
        }),
    )
}

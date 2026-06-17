//! Brawn — `{3}{G}` 3/3 Incarnation with Trample.
//! "As long as this card is in your graveyard and you control a Forest, creatures
//!  you control have trample." (graveyard-static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Brawn");
    let incarnation = reg.interner_mut().intern("Incarnation");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(incarnation);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "As long as this card is in your graveyard and you control a Forest,
    //   creatures you control have trample." — a graveyard-resident static
    //   continuous ability granting a keyword to your board; not a
    //   triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}

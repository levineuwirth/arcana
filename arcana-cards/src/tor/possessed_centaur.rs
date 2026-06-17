//! Possessed Centaur — `{2}{G}{G}` 3/3 green Centaur Horror.
//! Trample. Threshold: while seven or more cards are in your graveyard, it gets
//! +1/+1, is black, and gains "{2}{B}, {T}: Destroy target green creature."
//!
//! Only the keyword line is expressible. The Threshold clause is a single
//! static continuous ability that conditionally pumps, recolors, and grants an
//! activated ability — none of which is expressible as a triggered/activated
//! ability here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Possessed Centaur");
    let centaur = reg.interner_mut().intern("Centaur");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Threshold static — conditional +1/+1, color change, and granted
    // activated ability is one static continuous ability, not expressible here.
    reg.register(CardDefinition::new(name, chars))
}

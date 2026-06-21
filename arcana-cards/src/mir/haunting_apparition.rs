//! Haunting Apparition — `{1}{U}{B}` 1+*/2 black/blue Spirit with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * As this creature enters, choose an opponent. — a replacement/choice on
//!   entry; not expressible in this shape.
//! * Power is 1 plus the number of green creature cards in the chosen player's
//!   graveyard — a characteristic-defining ability. Bones carry power as
//!   `StarPlus(1)` (1+*); the CDA computation is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haunting Apparition");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::StarPlus(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "As this creature enters, choose an opponent" + the CDA
    // ("power = 1 + green creature cards in that player's graveyard") are not
    // expressible as triggered/activated abilities; bones carry power as 1+*.
    reg.register(CardDefinition::new(name, chars))
}

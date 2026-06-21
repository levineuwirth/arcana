//! Wall of Glare — `{1}{W}` 0/5 white Wall.
//!
//! Oracle:
//! * Defender.
//! * "This creature can block any number of creatures." — a static
//!   combat-rules modification; no expressible block-any-number primitive.
//!   GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Glare");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    // GAP: static — "This creature can block any number of creatures" (no
    // block-any-number combat-modification primitive).
    reg.register(CardDefinition::new(name, chars))
}

//! Morkrut Behemoth — `{4}{B}` 7/6 Zombie Giant with Menace.
//!
//! Oracle:
//! * As an additional cost to cast this spell, sacrifice a creature or pay
//!   {1}{B}. (additional alternative casting cost — GAP: not expressible.)
//! * Menace.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Morkrut Behemoth");
    let zombie = reg.interner_mut().intern("Zombie");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(giant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "As an additional cost to cast this spell, sacrifice a creature or
    // pay {1}{B}." — additional casting costs (and the modal "or") are not
    // expressible with the demonstrated card API.

    reg.register(CardDefinition::new(name, chars))
}

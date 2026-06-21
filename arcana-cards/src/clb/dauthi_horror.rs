//! Dauthi Horror — `{1}{B}` 2/1 black Dauthi Horror.
//!
//! Oracle:
//! * Shadow
//! * This creature can't be blocked by white creatures.
//!
//! Shadow is a base keyword. The "can't be blocked by white creatures" line
//! is a static conditional block restriction with no primitive in this
//! surface (`CantBeBlocked` is fully unblockable, which would over-state it),
//! so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauthi Horror");
    let dauthi = reg.interner_mut().intern("Dauthi");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dauthi);
    subtypes.0.insert(horror);

    // GAP: static "This creature can't be blocked by white creatures." — no
    // conditional block-restriction (cant-be-blocked-by-filter) primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

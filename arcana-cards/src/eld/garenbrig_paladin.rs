//! Garenbrig Paladin — `{4}{G}` 4/4 green Giant Knight.
//!
//! Oracle:
//! * Adamant — If at least three green mana was spent to cast this spell,
//!   this creature enters with a +1/+1 counter on it.
//! * This creature can't be blocked by creatures with power 2 or less.
//!
//! Both abilities are GAP'd: Adamant (mana-spent-to-cast tracking) is not an
//! expressible keyword or trigger condition, and "can't be blocked by
//! creatures with power N or less" is a static blocking restriction with a
//! power-filtered clause that the demonstrated `CantBeBlocked` (blanket /
//! single-target only) cannot model.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garenbrig Paladin");
    let giant = reg.interner_mut().intern("Giant");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Adamant (three-green-mana-spent → enters with +1/+1 counter) is
        // not an expressible keyword or trigger condition.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "can't be blocked by creatures with power 2 or less" —
    // power-filtered blocking restriction not expressible with CantBeBlocked.
    reg.register(CardDefinition::new(name, chars))
}

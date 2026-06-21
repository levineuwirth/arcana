//! Mosquito Guard — `{W}` 1/1 white Kithkin Soldier.
//!
//! Oracle:
//! * First strike
//! * Reinforce 1—{1}{W} (GAP)
//!
//! First strike is wired. Reinforce is not in the usable keyword
//! surface, and its discard-from-hand activated ability is not
//! expressible with the demonstrated API, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mosquito Guard");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(soldier);

    // GAP: "Reinforce 1—{1}{W}" — not in the usable keyword surface; the
    // discard-from-hand +1/+1-counter activated ability is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

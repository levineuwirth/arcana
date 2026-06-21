//! Volunteer Reserves — `{1}{W}` 2/4 white Human Soldier.
//!
//! Oracle:
//! * Banding
//! * Cumulative upkeep {1} (GAP)
//!
//! Banding is a usable keyword. Cumulative upkeep is not in the usable
//! keyword surface and its age-counter-then-sacrifice-unless-pay
//! upkeep mechanic is not expressible with the demonstrated API, so it
//! is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volunteer Reserves");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    // GAP: "Cumulative upkeep {1}" — not in the usable keyword surface;
    // the age-counter upkeep-pay-or-sacrifice mechanic is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

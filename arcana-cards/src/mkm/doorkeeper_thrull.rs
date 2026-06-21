//! Doorkeeper Thrull — `{1}{W}` 1/2 Thrull.
//!
//! Oracle:
//! * Flash, flying.
//! * Artifacts and creatures entering don't cause abilities to trigger.
//!
//! The keyword line is fully modeled. The "entering don't cause abilities to
//! trigger" static is a global trigger-suppression replacement with no
//! primitive, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doorkeeper Thrull");
    let thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);

    // GAP: static — "Artifacts and creatures entering don't cause abilities to
    // trigger" (global trigger-suppression replacement; no primitive).

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

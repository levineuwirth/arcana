//! Sadistic Skymarcher — `{2}{B}` 2/2 Vampire Soldier with Flying and Lifelink.
//!
//! Oracle:
//! * As an additional cost to cast this spell, reveal a Vampire card from your
//!   hand or pay {1}.
//! * Flying, lifelink.
//!
//! GAP: the additional casting cost ("reveal a Vampire card from your hand or
//! pay {1}") is a spell-casting additional cost with no representation on a
//! creature card definition — omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sadistic Skymarcher");
    let vampire = reg.interner_mut().intern("Vampire");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

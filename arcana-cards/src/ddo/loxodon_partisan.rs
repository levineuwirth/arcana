//! Loxodon Partisan — `{4}{W}` 3/4 Elephant Soldier with Battle Cry.
//! Mirrodin Besieged common; a five-mana creature that boosts the
//! rest of the attacking team via Battle Cry.
//!
//! # Rules references
//!
//! * CR 702.91 — Battle Cry. Whenever this creature attacks, each
//!   other attacking creature gets +1/+0 until end of turn. Engine
//!   wiring lives in the combat attack trigger pipeline.
//!
//! Battle Cry is a fully-implemented keyword; listing it in `keywords`
//! is sufficient — the runtime pipeline handles the trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loxodon Partisan");
    let elephant = reg.interner_mut().intern("Elephant");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::BattleCry],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

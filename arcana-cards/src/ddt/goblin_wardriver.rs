//! Goblin Wardriver — `{R}{R}` 2/2 Goblin Warrior with Battle Cry.
//! Mirrodin Besieged uncommon; an aggressive two-mana red creature that
//! pumps the rest of the attacking team via Battle Cry.
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
    let name = reg.interner_mut().intern("Goblin Wardriver");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::BattleCry],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

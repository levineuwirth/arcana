//! Keen-Eared Sentry — `{1}{W}` 2/1 white Human Soldier.
//!
//! * "You have hexproof." — a static granting hexproof to the player; not
//!   expressible (player-targeted static). GAP.
//! * "Each opponent can't venture into the dungeon more than once each
//!   turn." — a static activation-frequency restriction on opponents; not
//!   expressible. GAP.
//!
//! Both non-keyword lines are statics with no triggered/activated form, so
//! only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Keen-Eared Sentry");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

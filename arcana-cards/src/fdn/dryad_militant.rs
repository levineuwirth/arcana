//! Dryad Militant — `{G/W}` 2/1 Dryad Soldier.
//!
//! Oracle:
//! * "If an instant or sorcery card would be put into a graveyard from anywhere,
//!   exile it instead." — a static replacement effect (CR 614). There is no
//!   triggered/activated primitive for a continuous graveyard-replacement, so
//!   the static is GAP'd; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dryad Militant");
    let dryad = reg.interner_mut().intern("Dryad");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dryad);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static replacement "If an instant or sorcery card would be put into a
    // graveyard from anywhere, exile it instead." — no triggered/activated
    // primitive for a continuous graveyard-replacement on a creature.
    reg.register(CardDefinition::new(name, chars))
}

//! Paladin of Predation — `{5}{G}{G}` 6/7 Creature — Phyrexian Knight with Toxic 6.
//!
//! * Toxic 6 (keyword).
//! * "This creature can't be blocked by creatures with power 2 or less." — a
//!   power-conditional block restriction; the only block primitive is the
//!   unconditional Effect::CantBeBlocked, so the filtered evasion is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paladin of Predation");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Toxic(6)],
        ..Default::default()
    };

    // GAP: "can't be blocked by creatures with power 2 or less" — power-filtered
    // block restriction not expressible (only unconditional CantBeBlocked).
    reg.register(CardDefinition::new(name, chars))
}

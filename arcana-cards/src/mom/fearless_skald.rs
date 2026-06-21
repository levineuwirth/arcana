//! Fearless Skald — `{4}{R}` 3/2 Dwarf Berserker.
//! "Backup 1 (When this creature enters, put a +1/+1 counter on target
//!  creature. If that's another creature, it gains the following ability until
//!  end of turn.)"
//! "Double strike"
//!
//! Backup is not an available KeywordAbility variant, and the Backup ETB
//! (counter + conditional ability-grant of THIS card's other abilities) is not
//! expressible (GAP). Double strike is a usable keyword.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fearless Skald");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(berserker);

    // GAP: Backup 1 — not an available KeywordAbility variant; its ETB
    // (put a +1/+1 counter on target creature, then grant that creature this
    // card's other abilities until end of turn) is not expressible.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

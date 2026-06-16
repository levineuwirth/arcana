//! Solemn Recruit — `{1}{W}{W}` 2/2 Dwarf Warrior with Double strike.
//! "Revolt — At the beginning of your end step, if a permanent left the
//! battlefield under your control this turn, put a +1/+1 counter on
//! this creature."
//!
//! The Revolt intervening-if ("a permanent left the battlefield under
//! your control this turn") has no available condition predicate;
//! firing the counter unconditionally would be materially wrong, so the
//! whole trigger is GAP'd. Only Double strike and the bones are emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Solemn Recruit");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    // GAP: Revolt end-step trigger — "if a permanent left the battlefield
    // under your control this turn" has no available intervening-if
    // predicate; omitted rather than fired unconditionally.
    reg.register(CardDefinition::new(name, chars))
}

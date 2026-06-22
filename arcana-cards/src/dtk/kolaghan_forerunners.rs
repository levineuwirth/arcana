//! Kolaghan Forerunners — `{2}{R}` */3 Human Berserker with Trample.
//!
//! Oracle:
//! * Trample — base keyword.
//! * "Kolaghan Forerunners's power is equal to the number of creatures
//!   you control." — GAP: a characteristic-defining static (power
//!   marked `*` via PtValue::Star); no triggered/activated form.
//! * Dash {2}{R} — GAP: Dash is not a `KeywordAbility` variant (the
//!   dash alternative-cost cast mechanic is not modeled).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kolaghan Forerunners");
    let human = reg.interner_mut().intern("Human");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "power equal to the number of creatures you control" — CDA
    // static (power marked `*`). GAP: Dash {2}{R} — not a keyword
    // variant.
    reg.register(CardDefinition::new(name, chars))
}

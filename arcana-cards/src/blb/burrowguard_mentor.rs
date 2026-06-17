//! Burrowguard Mentor — `{G}{W}` */* Rabbit Soldier with Trample.
//! "Burrowguard Mentor's power and toughness are each equal to the number of
//! creatures you control." This characteristic-defining ability (CDA) is a
//! static; the `*/*` is rendered with `PtValue::Star` but the count-creatures
//! definition itself is GAP'd (no documented effect/CDA hook expresses it).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burrowguard Mentor");
    let rabbit = reg.interner_mut().intern("Rabbit");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "power and toughness are each equal to the number of creatures you
    // control." — CDA static; no documented hook to define the `*` value.

    reg.register(CardDefinition::new(name, chars))
}

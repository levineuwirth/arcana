//! Wilderness Elemental — `{1}{R}{G}` */3 Creature — Elemental.
//! Trample.
//! "Wilderness Elemental's power is equal to the number of nonbasic lands your
//! opponents control." — a characteristic-defining ability setting the `*`
//! power; no CDA registration hook is demonstrated, so power is left as `*`
//! (PtValue::Star) and the computing static is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wilderness Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: CDA "power is equal to the number of nonbasic lands your opponents
    // control" — no demonstrated hook to register a `*`-computing static.
    reg.register(CardDefinition::new(name, chars))
}

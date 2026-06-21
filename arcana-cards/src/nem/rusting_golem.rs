//! Rusting Golem — `{4}` */* Artifact Creature — Golem.
//! Fading 5 (enters with five fade counters; at the beginning of your
//! upkeep, remove a fade counter, else sacrifice). Synthesized from the
//! Fading keyword by the engine.
//! Rusting Golem's power and toughness are each equal to the number of
//! fade counters on it. (Static CDA — GAP'd: recorded as */* via
//! PtValue::Star.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rusting Golem");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    // GAP: static CDA "power and toughness each equal to the number of
    // fade counters on it" — recorded as */* via PtValue::Star.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Fading(5)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

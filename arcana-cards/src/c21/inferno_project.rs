//! Inferno Project — `{6}{R}` 0/0 Elemental with Trample.
//! "Enters with X +1/+1 counters, where X is the total mana value of instant
//! and sorcery cards in your graveyard." — no script helper for summed mana
//! value of a graveyard subset, so the enter-counters clause is a GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inferno Project");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "This creature enters with X +1/+1 counters on it, where X is the
    // total mana value of instant and sorcery cards in your graveyard." — no
    // summed-mana-value-of-graveyard-subset script helper.
    reg.register(CardDefinition::new(name, chars))
}

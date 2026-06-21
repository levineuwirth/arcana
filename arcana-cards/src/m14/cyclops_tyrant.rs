//! Cyclops Tyrant — `{5}{R}` 3/4 Creature — Cyclops.
//! Intimidate.
//! "This creature can't block creatures with power 2 or less." — a pure
//! static combat restriction with no trigger/cost; not expressible with the
//! demonstrated triggered/activated API, so it is GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cyclops Tyrant");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Intimidate],
        ..Default::default()
    };

    // GAP: static "This creature can't block creatures with power 2 or less."
    // — no blocking-restriction static available in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}

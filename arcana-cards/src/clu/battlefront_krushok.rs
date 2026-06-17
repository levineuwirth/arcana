//! Battlefront Krushok — `{4}{G}` 3/4 Beast.
//! "This creature can't be blocked by more than one creature."
//! "Each creature you control with a +1/+1 counter on it can't be
//! blocked by more than one creature."
//! Both are pure static abilities (no trigger word, no cost) — neither
//! is a triggered/activated ability, so both are GAP'd here.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battlefront Krushok");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "can't be blocked by more than one creature" (self) —
    // no menace-inverse blocking restriction primitive available.
    // GAP: static "each creature you control with a +1/+1 counter can't
    // be blocked by more than one creature."
    reg.register(CardDefinition::new(name, chars))
}

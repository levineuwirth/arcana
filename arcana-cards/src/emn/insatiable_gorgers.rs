//! Insatiable Gorgers — `{2}{R}{R}` 5/3 Vampire Berserker.
//! "This creature attacks each combat if able." (static — GAP)
//! "Madness {3}{R}." (GAP — Madness not in the usable keyword surface)
//!
//! GAP: static — "attacks each combat if able" (a combat-requirement static, no
//! trigger/activated form).
//! GAP: keyword — Madness {3}{R} (not among the usable KeywordAbility variants
//! for this card class).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Insatiable Gorgers");
    let vampire = reg.interner_mut().intern("Vampire");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

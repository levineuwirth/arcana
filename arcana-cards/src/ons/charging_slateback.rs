//! Charging Slateback — `{4}{R}` 4/3 Beast.
//! "This creature can't block. Morph {4}{R}."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging Slateback");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: "This creature can't block." — a permanent self-static
        // can't-block has no continuous primitive (ForbidBlocking is a
        // targeted, duration-bound effect, not a printed static).
        // GAP: Morph {4}{R} — Morph is not in the usable KeywordAbility
        // surface; the face-down cast mechanic is omitted.
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

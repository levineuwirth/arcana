//! Flow of Maggots — `{2}{B}` 2/2 Insect.
//! "Cumulative upkeep {1}" — not a supported KeywordAbility variant, GAP.
//! "This creature can't be blocked by non-Wall creatures." — a conditional
//! evasion static (blockable only by Walls) is not expressible (CantBeBlocked
//! is unconditional), GAP.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Flow of Maggots");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Cumulative upkeep {1}" (unsupported keyword) and "can't be blocked
    // by non-Wall creatures" (conditional evasion static) are not expressible.

    reg.register(CardDefinition::new(name, chars))
}

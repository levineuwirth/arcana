//! Incisor Steed — `{1}{W}` 1/4 Artifact Creature — Phyrexian Horse.
//! Vigilance.
//! Corrupted Metalcraft — As long as you control three or more artifacts
//! and an opponent has three or more poison counters, Incisor Steed gets
//! +3/+0. (conditional static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incisor Steed");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horse);

    // GAP: "Corrupted Metalcraft" conditional static "+3/+0 while you control
    // 3+ artifacts and an opponent has 3+ poison counters" — a continuous,
    // condition-gated self buff, not a triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

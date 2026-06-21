//! Bronze Horse — `{7}` 4/4 Artifact Creature — Horse.
//!
//! Trample
//! As long as you control another creature, prevent all damage that would be
//! dealt to this creature by spells that target it.
//!
//! Trample is wired as a keyword. The conditional damage-prevention static is
//! a pure continuous replacement (no trigger / no cost) with a control
//! condition and a "by spells that target it" source restriction; it has no
//! expressible declarative form on this card class and is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bronze Horse");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);

    // GAP: static — conditional "prevent all damage by spells that target it
    //      while you control another creature" continuous replacement.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

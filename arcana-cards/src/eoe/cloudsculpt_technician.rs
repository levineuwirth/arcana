//! Cloudsculpt Technician — `{2}{U}` 1/4 Jellyfish Artificer.
//! Flying.
//! "As long as you control an artifact, this creature gets +1/+0." —
//! a conditional static buff with no trigger or activation cost; not
//! expressible in this card class → GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloudsculpt Technician");
    let jellyfish = reg.interner_mut().intern("Jellyfish");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jellyfish);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "As long as you control an artifact, this creature gets +1/+0." —
    // a conditional continuous static; no triggered/activated surface.
    reg.register(CardDefinition::new(name, chars))
}

//! Barricade Breaker — `{7}` 7/5 Artifact Creature — Juggernaut.
//! * "Improvise" (artifacts help cast this spell).
//! * "This creature attacks each combat if able."
//!
//! GAP: Improvise is not in the usable keyword surface — emitted as no
//! keyword. GAP: "attacks each combat if able" is a static combat
//! requirement with no expressible primitive. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barricade Breaker");
    let juggernaut = reg.interner_mut().intern("Juggernaut");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(juggernaut);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Improvise is not in the usable keyword surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "attacks each combat if able" — no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}

//! Malleable Impostor — `{3}{U}` 0/0 blue Faerie Shapeshifter.
//! Flash, Flying.
//! You may have this creature enter as a copy of a creature an opponent
//! controls, except it's a Faerie Shapeshifter in addition to its other
//! types and it has flying.
//!
//! Flash and Flying are emitted. The "enter as a copy" replacement
//! effect (a clone-on-ETB choice) has no expressible primitive — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malleable Impostor");
    let faerie = reg.interner_mut().intern("Faerie");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "You may have this creature enter as a copy of a creature an
    //      opponent controls..." Enter-as-copy ETB replacement — no
    //      expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}

//! Esix, Fractal Bloom — `{4}{G}{U}` 4/4 Legendary Fractal with Flying.
//! "Flying"
//! "The first time you would create one or more tokens during each of your
//!  turns, you may instead choose a creature other than Esix and create that
//!  many tokens that are copies of that creature." (token-creation replacement
//!  — GAP)
//!
//! Flying is the keyword line. The once-per-turn token-creation replacement
//! effect is not expressible with the available primitives (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esix, Fractal Bloom");
    let fractal = reg.interner_mut().intern("Fractal");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fractal);

    // GAP: "The first time you would create one or more tokens during each of
    // your turns, you may instead ... create that many tokens that are copies
    // of [a chosen] creature." — a token-creation replacement effect with no
    // expressible primitive.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

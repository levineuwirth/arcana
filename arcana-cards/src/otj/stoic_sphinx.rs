//! Stoic Sphinx — `{2}{U}{U}` 5/3 blue Sphinx.
//! Flash, Flying.
//! This creature has hexproof as long as you haven't cast a spell this turn.
//!
//! Flash and Flying are emitted. The conditional static (gain hexproof
//! while you haven't cast a spell this turn) is a continuous conditional
//! ability with no expressible primitive — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stoic Sphinx");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "has hexproof as long as you haven't cast a spell this turn."
    //      Conditional continuous self-static — no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}

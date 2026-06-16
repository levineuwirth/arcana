//! Earl of Squirrel — `{4}{G}{G}` 4/4 Squirrel Noble Advisor.
//! Squirrellink (damage it deals also creates that many 1/1 green Squirrels).
//! Creature tokens you control are Squirrels in addition to their other types.
//! Other Squirrels you control get +1/+1.
//!
//! All three lines are statics with no expressible primitive:
//! - Squirrellink is not a usable KeywordAbility and has no damage→token-mint hook.
//! - The token-typeshift static has no continuous type-grant-by-controlled-set primitive.
//! - The "+1/+1 to other Squirrels" anthem static has no expressible primitive in this shape.
//! Emitting faithful bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earl of Squirrel");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let noble = reg.interner_mut().intern("Noble");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(noble);
    subtypes.0.insert(advisor);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Squirrellink — not a usable KeywordAbility, no damage→token primitive.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Creature tokens you control are Squirrels …" — no continuous type-grant primitive.
    // GAP: static "Other Squirrels you control get +1/+1" — no expressible anthem static in this shape.
    reg.register(CardDefinition::new(name, chars))
}

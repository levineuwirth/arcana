//! Angel of Vitality — `{2}{W}` 2/2 Creature — Angel.
//! "Flying"
//! "If you would gain life, you gain that much life plus 1 instead."
//! "This creature gets +2/+2 as long as you have 25 or more life."
//!
//! Decomposition:
//! - Keyword line: Flying.
//! - GAP: "If you would gain life, you gain that much life plus 1 instead." is
//!   a static replacement effect — no expressible Effect.
//! - GAP: "gets +2/+2 as long as you have 25 or more life" is a static
//!   conditional P/T continuous ability — no expressible Effect.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Angel of Vitality");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

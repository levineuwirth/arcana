//! Thought Nibbler — `{U}` 1/1 Beast with Flying.
//!
//! * Flying.
//! * Your maximum hand size is reduced by two. (GAP — static)
//!
//! The maximum-hand-size static has no expressible Effect/primitive (no
//! "modify maximum hand size" effect in the API), so it is GAP'd. Only the
//! Flying base keyword is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thought Nibbler");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Your maximum hand size is reduced by two." — no max-hand-size
    //       modification primitive exists.

    reg.register(CardDefinition::new(name, chars))
}

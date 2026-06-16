//! Deceptive Frostkite — `{U}{U}` 1/1 Dragon with Flying.
//!
//! Flying.
//! You may have this creature enter as a copy of a creature you control with
//! power 4 or greater, except it's a Dragon in addition to its other types
//! and it has flying (not modeled — see GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deceptive Frostkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "enter as a copy of a creature you control with power 4+" is a
    // copy-on-enter replacement effect — no enter-as-copy primitive exists
    // (CopyPermanent mints a token, it does not transform the entering card).
    reg.register(CardDefinition::new(name, chars))
}

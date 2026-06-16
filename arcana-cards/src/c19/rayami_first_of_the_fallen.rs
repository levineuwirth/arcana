//! Rayami, First of the Fallen — `{1}{B}{G}{U}` 5/4 Legendary Vampire.
//! "If a nontoken creature would die, exile that card with a blood counter
//! on it instead.
//! As long as an exiled creature card with a blood counter on it has
//! [keyword], Rayami has [keyword]. ..."
//!
//! Both lines are static abilities (a death-replacement effect and a
//! keyword-inheritance static). Neither is a triggered or activated
//! ability, and the demonstrated API exposes no replacement-effect /
//! conditional-keyword-grant static for the creature-with-abilities shape,
//! so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rayami, First of the Fallen");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    // GAP: "If a nontoken creature would die, exile that card with a blood
    // counter on it instead." — a death-replacement effect; no replacement
    // primitive is available for this creature shape.
    // GAP: "As long as an exiled creature card with a blood counter has
    // <keyword>, Rayami has <keyword>." — a conditional keyword-inheritance
    // static; not expressible as a trigger/activated/keyword ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

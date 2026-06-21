//! Sharkey, Tyrant of the Shire — `{2}{U}{B}` 2/4 Legendary Avatar Rogue.
//! Activated abilities of lands your opponents control can't be activated
//!   unless they're mana abilities.
//! Sharkey has all activated abilities of lands your opponents control
//!   except mana abilities.
//! Mana of any type can be spent to activate Sharkey's abilities.
//!
//! All three clauses are static continuous abilities (no trigger word,
//! no cost). None are expressible in this card class — there is no
//! primitive for restricting opponents' land activations, copying their
//! activated abilities onto this creature, or relaxing mana-cost color
//! requirements. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sharkey, Tyrant of the Shire");
    let avatar = reg.interner_mut().intern("Avatar");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(avatar);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP (static): "Activated abilities of lands your opponents control
    //   can't be activated unless they're mana abilities."
    // GAP (static): "Sharkey has all activated abilities of lands your
    //   opponents control except mana abilities."
    // GAP (static): "Mana of any type can be spent to activate Sharkey's
    //   abilities."
    reg.register(CardDefinition::new(name, chars))
}

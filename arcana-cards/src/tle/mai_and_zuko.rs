//! Mai and Zuko — `{1}{U}{B}{R}` 3/5 Legendary Human Noble Ally.
//! "Firebending 3
//!  You may cast Ally spells and artifact spells as though they had
//!  flash."
//!
//! Bones only. Firebending is a keyword not in the usable set. The
//! flash-permission static (cast Ally/artifact spells at instant speed)
//! has no Effect/permission primitive in this card class — GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mai and Zuko");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        // GAP: Firebending 3 — keyword not in the usable set.
        ..Default::default()
    };

    // GAP: "You may cast Ally spells and artifact spells as though they
    // had flash" — a static casting-permission with no Effect/permission
    // primitive in this card class.
    reg.register(CardDefinition::new(name, chars))
}

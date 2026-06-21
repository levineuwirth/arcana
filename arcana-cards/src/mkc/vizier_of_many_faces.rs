//! Vizier of Many Faces — `{2}{U}{U}` 0/0 Shapeshifter Cleric.
//!
//! * You may have this creature enter as a copy of any creature on the
//!   battlefield, except if this creature was embalmed, the token has no mana
//!   cost, it's white, and it's a Zombie in addition to its other types.
//!   GAP: "enter as a copy of any creature" (clone-on-ETB) is not expressible —
//!   there is no ETB-copy effect (Effect::CopyPermanent mints a NEW token rather
//!   than having this object enter as a copy).
//! * Embalm {3}{U}{U}. GAP: Embalm is not an available KeywordAbility variant.
//!
//! Both rules clauses are gaps; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vizier of Many Faces");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Stunt Double — `{3}{U}` 0/0 Creature — Shapeshifter. Flash.
//! "You may have this creature enter as a copy of any creature on the
//! battlefield."
//!
//! Flash is a base keyword. The clone-as-it-enters replacement ("enter as a
//! copy of any creature") has no expressible primitive in this card class —
//! `Effect::CopyPermanent` mints a NEW token copy of a target, it does not make
//! THIS object enter as a copy — so the copy effect is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stunt Double");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    // GAP: "You may have this creature enter as a copy of any creature on the
    // battlefield" — clone-as-it-enters replacement; no primitive expresses a
    // self enter-as-copy here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

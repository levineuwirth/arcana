//! Dauthi Slayer — `{B}{B}` 2/2 Dauthi Soldier (B). Shadow.
//! "This creature attacks each combat if able."
//!
//! Shadow is a base keyword. "Attacks each combat if able" is a static combat
//! requirement with no Effect / keyword surface (Goad applies to a target, not
//! a self-static), so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauthi Slayer");
    let dauthi = reg.interner_mut().intern("Dauthi");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dauthi);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Shadow],
        ..Default::default()
    };

    // GAP: "This creature attacks each combat if able" — static attack
    // requirement has no Effect / keyword surface.
    reg.register(CardDefinition::new(name, chars))
}

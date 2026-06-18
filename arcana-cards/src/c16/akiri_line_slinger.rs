//! Akiri, Line-Slinger — `{R}{W}` 0/3 Legendary Kor Soldier Ally.
//! First strike, vigilance. "Akiri gets +1/+0 for each artifact you
//! control." Partner.
//!
//! Keywords First strike + Vigilance are base characteristics.
//! Partner is not in the usable keyword surface (commander-only) and
//! is GAP'd. The dynamic +1/+0 static is a pure continuous ability
//! (no trigger, no cost) and cannot be expressed here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akiri, Line-Slinger");
    let kor = reg.interner_mut().intern("Kor");
    let soldier = reg.interner_mut().intern("Soldier");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kor);
    subtypes.0.insert(soldier);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Vigilance],
        // GAP: Partner is not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: static "Akiri gets +1/+0 for each artifact you control" is a
    // pure continuous ability with no trigger/cost — not expressible here.
    reg.register(CardDefinition::new(name, chars))
}

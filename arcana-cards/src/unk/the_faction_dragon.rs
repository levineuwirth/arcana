//! The Faction Dragon — Legendary Creature — Dragon, 4/3, with Flying.
//! A commander whose mana cost is chosen during deck building (a faction's mana
//! symbols); spells with the chosen watermark cost {1} less and other creatures
//! with that watermark get +1/+1.

use arcana_core::effects::KeywordAbility;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Faction Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    // GAP: mana cost is chosen during deck building from a faction's mana symbols
    // (no fixed printed cost); modeled as None.
    // GAP: "Spells you cast with the chosen faction's watermark cost {1} less" —
    // watermark-based cost reduction is not modeled; static omitted.
    // GAP: "Other creatures you control with the chosen faction's watermark get
    // +1/+1" — watermark-based continuous anthem is not modeled; static omitted.
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

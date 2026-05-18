//! Nyxborn Courser — vanilla 2/4 white enchantment creature for `{1}{W}{W}`.
//! Enchantment Creature — Centaur Scout. No abilities, pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nyxborn Courser");
    let subtype_centaur = reg.interner_mut().intern("Centaur");
    let subtype_scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_centaur);
    subtypes.0.insert(subtype_scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: (TypeLine::ENCHANTMENT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

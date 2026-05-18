//! Gilded Sentinel — vanilla 3/3 colorless artifact creature for `{4}`. Artifact Creature — Golem.
//! No abilities; pure stats.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gilded Sentinel");
    let subtype_golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(subtype_golem);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::default(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

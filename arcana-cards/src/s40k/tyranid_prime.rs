//! Tyranid Prime — `{1}{G}{U}` 0/4 Creature — Tyranid.
//! Evolve.
//! Synapse Creature — Other creatures you control have evolve. (Static keyword-granting
//! continuous ability — GAP.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tyranid Prime");
    let tyranid = reg.interner_mut().intern("Tyranid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tyranid);

    // GAP: static — "Synapse Creature — Other creatures you control have evolve."

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Simian Grunts — `{2}{G}` 3/4 Ape with Flash.
//!
//! Oracle:
//! * Flash
//! * Echo {2}{G} (At the beginning of your upkeep, if this came under your
//!   control since the beginning of your last upkeep, sacrifice it unless
//!   you pay its echo cost.)
//!
//! GAP: "Echo {2}{G}" is not in the usable KeywordAbility surface and has
//! no triggered/activated form expressible here (the came-under-control-
//! since-last-upkeep gate is not available). Only Flash is expressed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Simian Grunts");
    let ape = reg.interner_mut().intern("Ape");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Spirit of the Spires — `{3}{W}` 2/4 Spirit.
//! "Flying. Other creatures you control with flying get +0/+1."
//!
//! Flying is a base keyword. The anthem ("other creatures you control with
//! flying get +0/+1") is a pure static continuous ability with no trigger or
//! activation cost — not expressible as a TriggeredAbilityDef /
//! ActivatedAbilityDef in this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spirit of the Spires");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: static anthem "Other creatures you control with flying get +0/+1".
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

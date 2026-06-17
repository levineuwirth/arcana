//! Sylvan Advocate — `{1}{G}` 2/3 Elf Druid Ally with Vigilance.
//! "As long as you control six or more lands, this creature and land
//! creatures you control get +2/+2."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sylvan Advocate");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    subtypes.0.insert(ally);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: "As long as you control six or more lands, this creature and land
    //      creatures you control get +2/+2." — a conditional static
    //      continuous P/T-buff anthem, not a triggered/activated ability;
    //      not expressible with the demonstrated trigger/activated API.
    reg.register(CardDefinition::new(name, chars))
}

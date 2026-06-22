//! Hancock, Ghoulish Mayor — `{2}{B}` 2/1 Legendary black Zombie Mutant Advisor.
//! Each other creature you control that's a Zombie or Mutant gets +X/+X,
//! where X is the number of counters on Hancock.
//! Undying.
//!
//! Undying is a base keyword. The anthem ("each other Zombie/Mutant you
//! control gets +X/+X for X = counters on Hancock") is a dynamic continuous
//! static, not a triggered/activated ability — not expressible here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hancock, Ghoulish Mayor");
    let zombie = reg.interner_mut().intern("Zombie");
    let mutant = reg.interner_mut().intern("Mutant");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(mutant);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Undying],
        ..Default::default()
    };
    // GAP: anthem "each other Zombie/Mutant you control gets +X/+X (X =
    // counters on Hancock)" is a dynamic continuous static, not an ability.
    reg.register(CardDefinition::new(name, chars))
}

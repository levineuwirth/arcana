//! Gaddock Teeg — `{G}{W}` 2/2 Legendary Kithkin Advisor.
//! Noncreature spells with mana value 4 or greater can't be cast. (static — GAP)
//! Noncreature spells with {X} in their mana costs can't be cast. (static — GAP)
//!
//! Both lines are pure static cast-restriction abilities with no
//! triggered/activated form expressible in this card class — emitted as a
//! vanilla legendary creature with the statics GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaddock Teeg");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let advisor = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(advisor);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

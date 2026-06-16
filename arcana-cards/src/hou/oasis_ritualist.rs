//! Oasis Ritualist — `{3}{G}` 2/4 Snake Druid.
//! "{T}: Add one mana of any color."
//! "{T}, Exert this creature: Add two mana of any one color."
//!
//! GAP: "any color" mana production is not expressible — `Effect::AddMana`
//! takes a fixed `ManaColor` per pip, with no any-color choice primitive.
//! GAP: the Exert additional cost is not an `ActivationCost` field.
//! Both activated abilities are therefore unexpressible; bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oasis Ritualist");
    let snake = reg.interner_mut().intern("Snake");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Jiffy, Vehicle Repairer — `{R}{W}` Legendary 3/2 Dwarf Artificer.
//!
//! GAP (static): "Vehicles in your graveyard have jump-start" grants an
//! alternative graveyard-cast permission to other cards — not expressible
//! with the available Effect / static surface.
//! GAP (static): "Spells you cast with jump-start aren't exiled" is a
//! replacement effect on jump-start casts — not expressible.
//! Both lines are pure statics with no trigger/cost, so only the bones are
//! emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jiffy, Vehicle Repairer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}

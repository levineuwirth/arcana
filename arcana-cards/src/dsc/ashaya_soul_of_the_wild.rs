//! Ashaya, Soul of the Wild — `{3}{G}{G}` Legendary Elemental, */*.
//! "Ashaya's power and toughness are each equal to the number of lands
//! you control." (a characteristic-defining ability — not expressible.)
//! "Nontoken creatures you control are Forest lands in addition to their
//! other types." (a static type-changing ability — not expressible.)
//! Both abilities are static continuous effects with no trigger/cost, so
//! neither is a triggered/activated ability — GAP'd; emit bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashaya, Soul of the Wild");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: */* — power/toughness equal to lands you control (CDA);
        // placeholder fixed 0/0 bones.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "Nontoken creatures you control are Forest lands in addition to
    // their other types" — static type-granting continuous ability.
    reg.register(CardDefinition::new(name, chars))
}

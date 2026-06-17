//! Phila, Unsealed — `{4}` 4/4 Legendary Artifact Creature — Golem.
//! Both lines are conditional static type/ability grants keyed on the outcome of a
//! real-world event (Mirrans vs. Phyrexians, Feb 17 2023). Neither the event-result
//! gate nor the granted statics (added subtype + granted keyword/activated ability,
//! Toxic-granting anthem) is expressible on this surface — GAP both.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phila, Unsealed");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "If the Mirrans won … Phila is also a Rebel with … and has … activated
    // ability" — event-result-gated static type/ability grant, not expressible.
    // GAP: "If the Phyrexians won … Phila is also a Phyrexian with … anthem" — same.
    reg.register(CardDefinition::new(name, chars))
}

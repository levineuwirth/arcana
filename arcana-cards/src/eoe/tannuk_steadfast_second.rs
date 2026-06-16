//! Tannuk, Steadfast Second — `{2}{R}{R}` 3/5 Legendary Kavu Pilot.
//!
//! Other creatures you control have haste.
//! Artifact cards and red creature cards in your hand have warp {2}{R}.
//!
//! Both lines are static abilities — an anthem-style keyword grant to other
//! creatures, and a hand-wide warp-cost grant — neither expressible with the
//! demonstrated triggered/activated API. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tannuk, Steadfast Second");
    let kavu = reg.interner_mut().intern("Kavu");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static "Other creatures you control have haste" — no keyword-anthem
    // continuous-grant primitive in the demonstrated API.
    // GAP: static "Artifact cards and red creature cards in your hand have warp
    // {2}{R}" — hand-wide cost-modifier grant not expressible.
    reg.register(CardDefinition::new(name, chars))
}

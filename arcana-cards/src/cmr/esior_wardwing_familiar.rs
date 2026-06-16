//! Esior, Wardwing Familiar — `{1}{U}` 1/3 Legendary Bird with Flying.
//! "Spells your opponents cast that target one or more commanders you
//!  control cost {3} more to cast."
//! "Partner."
//!
//! Flying is expressible. The commander cost-increase static has no
//! primitive (GAP), and Partner is not in the available keyword surface
//! (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esior, Wardwing Familiar");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    // GAP: "Spells your opponents cast that target ... cost {3} more" — static
    //   cost-increase, no primitive.
    // GAP: Partner — not in the available keyword surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

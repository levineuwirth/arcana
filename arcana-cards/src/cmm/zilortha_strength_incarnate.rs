//! Zilortha, Strength Incarnate — `{3}{R}{G}` 7/3 Legendary Dinosaur with
//! Trample.
//! "Lethal damage dealt to creatures you control is determined by their power
//! rather than their toughness." (static)
//!
//! Decomposition:
//! * Trample — keyword.
//! * Lethal-damage redefinition — a pure static continuous (combat-damage rules
//!   modification) with no trigger/cost; GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zilortha, Strength Incarnate");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "Lethal damage to creatures you control is determined by power rather
    //      than toughness" — static combat-rules modification, not expressible.
    reg.register(CardDefinition::new(name, chars))
}

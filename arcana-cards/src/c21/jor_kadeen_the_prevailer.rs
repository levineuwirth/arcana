//! Jor Kadeen, the Prevailer — `{3}{R}{W}` 5/4 Legendary Human Warrior.
//!
//! Oracle:
//! * First strike
//! * Metalcraft — Creatures you control get +3/+0 as long as you control
//!   three or more artifacts.
//!
//! First strike is a base keyword. The Metalcraft anthem is a conditional
//! static continuous ability with no keyword-anthem primitive in this
//! surface, so it is GAP'd. (Metalcraft is a labeling keyword, not a usable
//! `KeywordAbility` variant.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jor Kadeen, the Prevailer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    // GAP: static "Metalcraft — Creatures you control get +3/+0 as long as
    // you control three or more artifacts." — no conditional continuous
    // anthem primitive in the MultiAbilityCreature surface.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Neriv, Heart of the Storm — `{1}{R}{W}{B}` 4/5 Legendary Spirit Dragon.
//!
//! * Flying.
//! * "If a creature you control that entered this turn would deal damage, it
//!   deals twice that much damage instead." — GAP: no double-damage replacement
//!   effect exists in the engine, and there is no entered-this-turn-scoped
//!   damage replacement primitive. Only the Flying keyword is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Neriv, Heart of the Storm");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: double-damage replacement for creatures that entered this turn is
    // not expressible (no double-damage / damage-multiplier replacement effect).
    reg.register(CardDefinition::new(name, chars))
}

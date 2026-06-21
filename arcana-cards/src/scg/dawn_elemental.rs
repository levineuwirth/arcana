//! Dawn Elemental — `{W}{W}{W}{W}` 3/3 Creature — Elemental.
//!
//! Oracle:
//! * Flying
//! * Prevent all damage that would be dealt to this creature.
//!
//! The damage-prevention shield is a static replacement effect ("prevent all
//! damage that would be dealt to ~"). The demonstrated catalog exposes
//! `Effect::PreventDamage` only as a one-shot from a triggered/activated
//! ability, not as a permanent static replacement, so the shield is GAP'd.
//! Flying is a base characteristic.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dawn Elemental");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Prevent all damage that would be dealt to this creature" —
    // a permanent self-targeting damage-prevention replacement is not in the
    // demonstrated MultiAbilityCreature surface.
    reg.register(CardDefinition::new(name, chars))
}

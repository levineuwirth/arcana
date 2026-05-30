//! Drogskol Infantry // Drogskol Armaments — `{1}{W}` Creature — Spirit
//! Soldier 2/2.
//!
//! Front face: 2/2 Spirit Soldier (white).
//! Disturb {3}{W} (cast from graveyard transformed).
//!
//! Back face: Drogskol Armaments — Enchantment — Aura (white).
//! - Enchant creature.
//! - Enchanted creature gets +2/+2.
//! - If Drogskol Armaments would be put into a graveyard from anywhere, exile
//!   it instead.
//!
//! # GAPs
//! - Disturb keyword: not in the keyword catalog. Emitting `keywords: vec![]`.
//!   The disturb cast-from-graveyard-transformed mechanic is not modeled.
//! - "Enchanted creature gets +2/+2": static aura pump on back face is not
//!   representable via triggered abilities (it's a continuous effect). GAP:
//!   aura pump effect not modeled.
//! - "If Drogskol Armaments would be put into a graveyard from anywhere,
//!   exile it instead": replacement effect; not expressible. GAP: exile-
//!   instead-of-graveyard replacement not modeled.
//! - GAP: back-face-only triggered ability not modeled.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drogskol Infantry");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    subtypes.0.insert(soldier_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Drogskol Armaments");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            // Disturb cost {3}{W} — back face has its own mana cost for disturb
            mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid disturb cost")),
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}

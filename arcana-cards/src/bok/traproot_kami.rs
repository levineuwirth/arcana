//! Traproot Kami — `{G}` 0/* Creature — Spirit.
//! Defender, Reach.
//! Traproot Kami's toughness is equal to the number of Forests on the battlefield.
//!
//! GAP: this CDA sets ONLY toughness (power stays a fixed 0), so the symmetric
//! self_pt_from_match (which sets both P/T equal to a battlefield count) is
//! wrong, and the asymmetric self_pt_cda compute fn has no registry, so it
//! cannot resolve the "Forest" subtype by name. Neither CDA constructor can
//! express an asymmetric-by-subtype count; toughness is left as Star.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Traproot Kami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

//! Omnipresent Impostor — `{2}` 2/1 Basic Creature — Shapeshifter,
//! changeling. The two non-keyword abilities are static and have no
//! expressible primitive, so this is bones + Changeling only.
//!
//! GAP: "Omnipresent Impostor has all card names." — static, no
//! all-card-names primitive.
//! GAP: "As you search your library for one or more cards, you may
//! choose Omnipresent Impostor as one of those cards." — static search
//! replacement, no primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Omnipresent Impostor");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::BASIC),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

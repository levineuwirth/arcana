//! Beloved Beggar // Generous Soul — `{1}{W}` Creature — Human Peasant 0/4.
//!
//! Disturb {4}{W}{W} (You may cast this card from your graveyard transformed
//! for its disturb cost.)
//!
//! Back face (Generous Soul): Creature — Spirit. Flying, Vigilance.
//!   If Generous Soul would be put into a graveyard from anywhere, exile it
//!   instead.
//!
//! # GAP: Disturb keyword is not in the available engine keyword surface;
//!   omitted. The disturb cast mechanic (cast from graveyard transformed) is
//!   engine debt.
//! # GAP: "If Generous Soul would be put into a graveyard from anywhere, exile
//!   it instead" — replacement effect on the back face is not expressible with
//!   available Effect variants.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Beloved Beggar");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Disturb keyword not in available engine keyword surface
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Generous Soul — Creature — Spirit, Flying, Vigilance
    let back_name = reg.interner_mut().intern("Generous Soul");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
            // GAP: "If ~ would be put into a graveyard from anywhere, exile it
            // instead" — graveyard replacement effect not expressible.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}

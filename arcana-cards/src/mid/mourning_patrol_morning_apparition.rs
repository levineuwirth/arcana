//! Mourning Patrol // Morning Apparition
//!
//! Front face: `{2}{W}` Creature — Human Soldier 2/3.
//! Vigilance.
//! "Disturb {3}{W} (You may cast this card from your graveyard transformed for its
//!  disturb cost.)"
//!
//! Back face: Creature — Spirit Soldier.
//! Flying, vigilance.
//! "If Morning Apparition would be put into a graveyard from anywhere, exile it instead."
//!
//! # GAPs
//! - Disturb keyword (cast from graveyard transformed) is not in the supported keyword set.
//!   The keyword is not emitted; the mechanic is not wired.
//! - Back-face "if would be put into graveyard ... exile it instead" — replacement effect
//!   not expressible in the current catalog (no ReplacementEffect::ExileInsteadOfGraveyard).
//!   GAP: back-face-only replacement effect not modeled.
//! - Back face P/T not stated in oracle; using 2/3 to match front face as reasonable guess.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mourning Patrol");
    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        // GAP: Disturb {3}{W} — keyword not in supported set; not emitted.
        ..Default::default()
    };

    // Back face: Morning Apparition — Creature — Spirit Soldier, Flying, Vigilance
    let back_name = reg.interner_mut().intern("Morning Apparition");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let back_soldier_sub = reg.interner_mut().intern("Soldier");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit_sub);
    back_subtypes.0.insert(back_soldier_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
            // GAP: "If Morning Apparition would be put into a graveyard from anywhere, exile
            // it instead." — replacement effect not expressible in catalog.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}

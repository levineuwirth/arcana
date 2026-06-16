//! Krang, Utrom Warlord — `{9}` 9/9 Legendary Artifact Creature — Utrom Robot.
//! Flying, trample, indestructible, haste.
//! "Other artifact creatures you control have flying, trample, indestructible,
//!  and haste."
//!
//! The keyword line maps directly to KeywordAbility variants. The second line
//! is a PURE STATIC continuous ability (no trigger word, no cost — a keyword-
//! granting anthem over other artifact creatures); it is not a triggered or
//! activated ability, so it is GAP'd per card-class rules (static keyword
//! grants are not expressible via TriggeredAbilityDef / ActivatedAbilityDef).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krang, Utrom Warlord");
    let utrom = reg.interner_mut().intern("Utrom");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(utrom);
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{9}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(9)),
        toughness: Some(PtValue::Fixed(9)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Indestructible,
            KeywordAbility::Haste,
        ],
        // GAP: static continuous ability "Other artifact creatures you control
        // have flying, trample, indestructible, and haste" — a keyword-granting
        // anthem with no trigger/cost is not expressible as a triggered/
        // activated ability for this card class.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}

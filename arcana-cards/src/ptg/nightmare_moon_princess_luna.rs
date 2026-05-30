//! Nightmare Moon // Princess Luna
//!
//! Front face: `{4}{B}{B}` Legendary Creature — Alicorn 6/6.
//! Flying.
//! "As long as it's nighttime, Nightmare Moon gets +2/+2 and has menace."
//! "{6}: Transform Nightmare Moon. Anypony may activate this ability or help pay the cost.
//!  When they do, they become your friend."
//!
//! Back face: Legendary Creature — Alicorn (no P/T specified in oracle; keeping 6/6 as same).
//! Flying.
//! "When this creature transforms into Princess Luna, choose up to six cards you own from
//!  outside the game with a moon in their art, then exile those cards. As long as those cards
//!  remain exiled, you may cast them, and your friends may cast them with your permission."
//!
//! # GAPs
//! - "As long as it's nighttime" — day/night cycle not modeled; the static +2/+2 and menace
//!   rider are omitted (no conditional static layer API).
//! - "{6}: Transform Nightmare Moon" is an activated ability. The engine doesn't expose
//!   ActivatedAbilityDef in the generation API here; the transform is not wired as an
//!   activated ability (GAP: activated-ability-driven transform not modeled).
//! - "Anypony may activate this ability or help pay the cost" — multi-player cost-sharing
//!   not supported.
//! - "When they do, they become your friend" — friendship tracking not modeled.
//! - Back-face transform-into trigger: "choose up to six cards from outside the game" —
//!   outside-game zone not modeled. GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightmare Moon");
    let alicorn_sub = reg.interner_mut().intern("Alicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alicorn_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // Back face: Princess Luna
    let back_name = reg.interner_mut().intern("Princess Luna");
    let back_alicorn = reg.interner_mut().intern("Alicorn");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_alicorn);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: "{6}: Transform Nightmare Moon" is an activated ability; ActivatedAbilityDef
    // is not authored here (no cost path available in this generation shape). The transform
    // trigger is left unwired.
    // GAP: back-face-only triggered ability (transform-into Princess Luna) not modeled.
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back),
    )
}

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
//! - "As long as it's nighttime" — day/night cycle's conditional static +2/+2 and menace
//!   rider are omitted (no conditional static layer API for the night-only buff).
//! - "Anypony may activate this ability or help pay the cost" — multi-player cost-sharing
//!   not supported; the {6} transform is wired as a normal controller-only activated ability.
//! - "When they do, they become your friend" — friendship tracking not modeled.
//! - Back-face transform-into trigger: "choose up to six cards from outside the game" —
//!   the outside-the-game zone is not modeled, so this back-face triggered ability is GAP'd
//!   (no Effect can exile/cast cards from outside the game).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
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

    // Front face: "{6}: Transform Nightmare Moon." The controller-only core is wired;
    // the "anypony may help pay / become your friend" multiplayer riders are GAP'd.
    // GAP: back-face-only triggered ability (transform-into Princess Luna → exile up to
    // six cards from outside the game) is not modeled (no outside-the-game zone).
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}: Transform Nightmare Moon.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: transform_self,
            }),
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

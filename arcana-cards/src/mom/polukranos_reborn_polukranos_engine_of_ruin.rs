//! Polukranos Reborn // Polukranos, Engine of Ruin — `{G}{G}{G}` green Legendary Hydra 4/5 (front) /
//! Legendary Phyrexian Hydra (back). Transform card.
//!
//! Front face:
//!   Reach
//!   {6}{W/P}: Transform Polukranos Reborn. Activate only as a sorcery.
//!
//! Back face (Polukranos, Engine of Ruin):
//!   Reach, lifelink
//!   Whenever Polukranos or another nontoken Hydra you control dies, create a 3/3 green
//!   and white Phyrexian Hydra creature token with reach and a 3/3 green and white
//!   Phyrexian Hydra creature token with lifelink.
//!
//! GAP: back-face-only triggered ability (nontoken Hydra dies → create tokens) not
//!      auto-installed on transform.

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
    let name = reg.interner_mut().intern("Polukranos Reborn");
    let hydra_sub = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Reach],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // Back face: Polukranos, Engine of Ruin — Legendary Phyrexian Hydra
    let back_name = reg.interner_mut().intern("Polukranos, Engine of Ruin");
    let back_hydra_sub = reg.interner_mut().intern("Hydra");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_hydra_sub);
    back_subtypes.0.insert(phyrexian_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Reach, KeywordAbility::Lifelink],
            power: Some(PtValue::Fixed(8)),
            toughness: Some(PtValue::Fixed(8)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Activated ability: {6}{W/P}: Transform. Sorcery speed (face 0 only).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{W/P}: Transform Polukranos Reborn. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{W/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false, // sorcery speed
                face_gate: Some(0),      // front face only
                effect: transform_self,
            })
        // GAP: back-face-only triggered ability not auto-installed on transform.
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

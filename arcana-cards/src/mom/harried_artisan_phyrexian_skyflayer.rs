//! Harried Artisan // Phyrexian Skyflayer — `{2}{R}` Human Artificer creature
//! 2/3 with Haste. Front face has an activated ability to transform.
//! Back face: Phyrexian Artificer creature with Flying and Haste.
//!
//! # Rules text (front face — Harried Artisan)
//! Haste
//! {3}{W/P}: Transform this creature. Activate only as a sorcery.
//! ({W/P} can be paid with either {W} or 2 life.)
//!
//! # Rules text (back face — Phyrexian Skyflayer)
//! Flying, haste
//!
//! # GAPs
//! - {W/P} Phyrexian mana is not supported by ManaCost; the transform cost
//!   is approximated as {3}{W} (dropping the Phyrexian mana rider).
//!   // GAP: {W/P} Phyrexian mana not in ManaCost; using {3}{W} as closest
//!   approximation (omits the "or 2 life" alternate payment).
//! - "Activate only as a sorcery" restriction is honored via is_instant_speed: false.

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
    let name = reg.interner_mut().intern("Harried Artisan");
    let human_sub = reg.interner_mut().intern("Human");
    let artificer_sub = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(artificer_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid creature cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // Back face — Phyrexian Skyflayer
    let back_name = reg.interner_mut().intern("Phyrexian Skyflayer");
    let back_phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_artificer_sub = reg.interner_mut().intern("Artificer");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_phyrexian_sub);
    back_subtypes.0.insert(back_artificer_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: {W/P} Phyrexian mana not in ManaCost; using {3}{W} as
                // closest approximation (omits the "or 2 life" alternate payment).
                text: "{3}{W/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
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

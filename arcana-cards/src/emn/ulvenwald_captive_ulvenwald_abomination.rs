//! Ulvenwald Captive // Ulvenwald Abomination — `{1}{G}` transform creature.
//! Front: Werewolf Horror 1/2, Defender.
//!   {T}: Add {G}.
//!   {5}{G}{G}: Transform this creature.
//! Back: Eldrazi Werewolf (no mana cost), colorless.
//!   {T}: Add {C}{C}.
//!
//! GAP: Back-face activated ability "{T}: Add {C}{C}" is modeled only on
//!      the front face via face_gate; the engine doesn't auto-install back-face
//!      abilities on transform, so the back face has its {T}: Add {C}{C} as a
//!      face-gated ability (face_gate: Some(1)).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ulvenwald Captive");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(werewolf_sub);
    front_subs.0.insert(horror_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ulvenwald Abomination");
    let eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let werewolf_back_sub = reg.interner_mut().intern("Werewolf");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(eldrazi_sub);
    back_subs.0.insert(werewolf_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: Add {G}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: add_green_mana,
            })
            // Front face: {5}{G}{G}: Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}{G}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_front_to_back,
            })
            // Back face: {T}: Add {C}{C}
            // GAP: back-face-only ability — live because the engine retains all
            //      abilities on the CardDefinition regardless of face; face_gate
            //      restricts it to face 1 (back).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}{C}.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: add_two_colorless_mana,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn transform_front_to_back(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn add_two_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
            ManaUnit::plain(ManaColor::Colorless, ctx.source),
        ],
    }]
}

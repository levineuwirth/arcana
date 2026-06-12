//! Blightreaper Thallid // Blightsower Thallid —
//! `{1}{B}` black Fungus creature 2/2 (front).
//!
//! Front: `{3}{G/P}: Transform this creature. Activate only as a sorcery.`
//! ({G/P} can be paid with either {G} or 2 life.)
//!
//! Back: Blightsower Thallid — Phyrexian Fungus creature.
//! "When this creature transforms into Blightsower Thallid or dies, create a
//! 1/1 green Phyrexian Saproling creature token."
//!
//! # GAP
//! - The "dies" half of the trigger is authored on CardDefinition and fires on
//!   both faces (oracle: the ability is back-face-only, but the dies half only
//!   matters once transformed in practice — front-face dies also fires here).
//! - Back face P/T inferred from Scryfall: 3/3.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blightreaper Thallid");
    let fungus_sub = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Blightsower Thallid");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let fungus_sub2 = reg.interner_mut().intern("Fungus");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(fungus_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Pre-intern token subtypes
    let _ = reg.interner_mut().intern("Saproling");

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front activated ability: {3}{G/P}: Transform. Sorcery speed only.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G/P}").expect("valid phyrexian cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // "When this creature ... or dies, create a 1/1 green Phyrexian Saproling token."
            // Dies half. GAP: fires on both faces (oracle: back-face-only ability).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: create_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Transforms into Blightsower Thallid" half.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: create_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn create_saproling(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // Creates a 1/1 green Phyrexian Saproling creature token.
    let saproling = reg.interner().lookup("Saproling")
        .expect("Saproling interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(saproling);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: saproling,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

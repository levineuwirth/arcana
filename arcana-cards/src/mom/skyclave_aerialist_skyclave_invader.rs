//! Skyclave Aerialist // Skyclave Invader — `{1}{U}` blue transform creature.
//! Front: Merfolk Scout 2/1, Flying.
//!   "{4}{G/P}: Transform this creature. Activate only as a sorcery."
//!   ({G/P} can be paid with either {G} or 2 life.)
//! Back: Phyrexian Merfolk Scout (no mana cost), Flying.
//!   "When this creature transforms into Skyclave Invader, look at the top card
//!    of your library. If it's a land card, you may put it onto the battlefield.
//!    If you don't put the card onto the battlefield, put it into your hand."
//! GAP: {G/P} hybrid-Phyrexian mana cost for the activation — modeled as {4}{G}
//!      (the green alternative; life-payment option is engine debt).
//! GAP: the non-land branch of the transforms-into trigger ("if you don't put the
//!      card onto the battlefield, put it into your hand") is not expressible —
//!      `DigRest` has no Hand destination; a non-land top card goes to the bottom
//!      instead. The land branch ("you may put it onto the battlefield") is wired
//!      via RevealUntil (the put is deterministic, not optional).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::effects::{DigRest, RevealDest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyclave Aerialist");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");

    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(merfolk);
    front_subs.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Skyclave Invader");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let merfolk_b = reg.interner_mut().intern("Merfolk");
    let scout_b = reg.interner_mut().intern("Scout");

    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(phyrexian);
    back_subs.0.insert(merfolk_b);
    back_subs.0.insert(scout_b);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet::default(),
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // "{4}{G/P}: Transform this creature. Activate only as a sorcery."
            // GAP: {G/P} modeled as {4}{G} (Phyrexian life-payment alternative deferred).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{G/P}: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_to_invader,
            })
            // "When this creature transforms into Skyclave Invader, look at the
            // top card of your library. If it's a land card, you may put it onto
            // the battlefield. If you don't put the card onto the battlefield,
            // put it into your hand."
            // GAP: the non-land → hand branch is not expressible (DigRest has no
            // Hand destination); a non-land top card goes to the bottom instead.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: on_transform_dig_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Activated ability: transform to Skyclave Invader.
fn transform_to_invader(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

/// Transforms-into trigger: look at the top card; a land goes onto the
/// battlefield. GAP: a non-land card should go to hand; it bottoms instead.
fn on_transform_dig_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(1),
    }]
}

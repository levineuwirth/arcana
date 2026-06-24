//! Sygg, Wanderwine Wisdom // Sygg, Wanderbrine Shield —
//! `{1}{U}` Legendary Merfolk Wizard 2/2 (front) / Merfolk Rogue 2/2 (back).
//! Transform card.
//!
//! Front face (Sygg, Wanderwine Wisdom):
//!   Sygg can't be blocked.
//!   Whenever this creature enters or transforms into Sygg, Wanderwine Wisdom,
//!   target creature gains "Whenever this creature deals combat damage to a
//!   player or planeswalker, draw a card" until end of turn.
//!   At the beginning of your first main phase, you may pay {W}. If you do,
//!   transform Sygg.
//!
//! Back face (Sygg, Wanderbrine Shield):
//!   Sygg can't be blocked.
//!   Whenever this creature transforms into Sygg, Wanderbrine Shield, target
//!   creature you control gains protection from each color until your next turn.
//!   At the beginning of your first main phase, you may pay {U}. If you do,
//!   transform Sygg.
//!
//! GAP: Static "can't be blocked" characteristic is not modeled as a keyword;
//!   CantBeBlocked is an Effect, not a static characteristic.
//! The front "enters or transforms into" trigger grants the targeted creature
//!   "whenever this deals combat damage to a player, draw a card" until end of
//!   turn via `Effect::GrantTriggeredAbility` (Warrior's Lesson idiom). NOTE:
//!   the player half is faithful; the "or planeswalker" half is dropped because
//!   no player-or-planeswalker-but-not-creature TargetFilter exists.
//! The back transform trigger grants the targeted creature you control
//!   protection from each color (`ProtectionQuality::AnyColor`) until your next
//!   turn via `Effect::GrantKeyword`.
//! GAP: "beginning of your first main phase" is the PreCombatMain step; wired
//!   as a sorcery-speed activated ability (timing not separately enforced).

use arcana_core::effects::{Effect, KeywordAbility, ProtectionQuality};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
    GRANTED_TRIGGER_ID_BASE,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sygg, Wanderwine Wisdom");
    let merfolk_sub = reg.interner_mut().intern("Merfolk");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: static "can't be blocked" not expressible as a keyword; no keywords field set
        ..Default::default()
    };

    // Back face: Sygg, Wanderbrine Shield — Merfolk Rogue 2/2
    let back_name = reg.interner_mut().intern("Sygg, Wanderbrine Shield");
    let back_merfolk = reg.interner_mut().intern("Merfolk");
    let back_rogue = reg.interner_mut().intern("Rogue");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_merfolk);
    back_subtypes.0.insert(back_rogue);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            // GAP: static "can't be blocked" not expressible as a keyword
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: "whenever this enters OR transforms into Sygg,
            // Wanderwine Wisdom, target creature gains 'whenever this creature
            // deals combat damage to a player, draw a card' until end of turn."
            // ETB half:
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sygg_grant_draw_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // "Transforms into front face" half (front-face directional).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(0) },
                intervening_if: None,
                effect: sygg_grant_draw_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // Back-face: "whenever this transforms into Sygg, Wanderbrine Shield,
            // target creature you control gains protection from each color until
            // your next turn."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: sygg_grant_protection,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Front-face activated ability: at beginning of first main phase,
            // pay {W} to transform. Modeled as an activated ability with mana cost {W}.
            // GAP: "at the beginning of your first main phase" timing not enforced;
            // modeled as a sorcery-speed activated ability.
            .with_activated_ability(ActivatedAbilityDef {
                text: "At the beginning of your first main phase, you may pay {W}. If you do, transform Sygg.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_to_back,
            })
            // Back-face activated ability: at beginning of first main phase,
            // pay {U} to transform back.
            .with_activated_ability(ActivatedAbilityDef {
                text: "At the beginning of your first main phase, you may pay {U}. If you do, transform Sygg.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: transform_to_front,
            }),
    )
}

/// Front trigger: grant the targeted creature "whenever this creature deals
/// combat damage to a player, draw a card" until end of turn.
fn sygg_grant_draw_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let granted = TriggeredAbilityDef {
        id: GRANTED_TRIGGER_ID_BASE + 1,
        trigger_condition: TriggerCondition::DamageDealt {
            source_filter: ObjectFilter::creature(),
            target_filter: TargetFilter::Player,
            combat_only: true,
        },
        intervening_if: None,
        effect: granted_draw_a_card,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    };
    vec![Effect::GrantTriggeredAbility {
        target: *id,
        ability: Box::new(granted),
        duration: Duration::EndOfTurn,
    }]
}

/// Granted "whenever this creature deals combat damage to a player, draw a card."
fn granted_draw_a_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

/// Back trigger: grant the targeted creature you control protection from each
/// color until your next turn.
fn sygg_grant_protection(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword: KeywordAbility::Protection(ProtectionQuality::AnyColor),
        duration: Duration::UntilYourNextTurn(trig.controller),
    }]
}

fn transform_to_back(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn transform_to_front(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

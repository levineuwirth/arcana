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
//! GAP: "grant a triggered ability until end of turn" (draw on combat damage)
//!   is not an expressible Effect variant — front ETB/transform trigger omitted.
//! GAP: "protection from each color until your next turn" is not a modeled
//!   keyword in the supported set — back-face transform trigger omitted.
//! GAP: "beginning of your first main phase" is the PreCombatMain step; wired
//!   as StepBegins { step: Step::PreCombatMain, whose: You } (closest approximation).
//! GAP: back-face-only triggered ability not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
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
            // Front-face ETB trigger: "when this enters, target creature gets
            // draw-on-damage triggered ability until EOT".
            // GAP: "grant a triggered ability until end of turn" not expressible;
            // trigger registered but effect is empty.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sygg_etb_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
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
            })
            // GAP: back-face-only triggered ability (transform trigger granting
            // protection from each color) not modeled.
    )
}

fn sygg_etb_trigger(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "grant triggered ability until end of turn" not expressible.
    Vec::new()
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

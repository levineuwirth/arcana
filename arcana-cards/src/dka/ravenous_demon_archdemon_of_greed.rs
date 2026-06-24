//! Ravenous Demon // Archdemon of Greed — `{3}{B}{B}` Demon creature 4/4.
//! Front: Sacrifice a Human: Transform this creature. Activate only as a sorcery.
//! Back: Flying, trample. At the beginning of your upkeep, sacrifice a Human. If you can't,
//!       tap this creature and it deals 9 damage to you.
//!
//! GAP: "Sacrifice a Human" as an activation COST — ActivationCost.sacrifice_other supports
//! a filter but the front-face activated ability requires sorcery speed AND a sacrifice cost.
//! The sacrifice_other field is modeled on ActivationCost but "Sacrifice a Human" specifically
//! requires a Human filter. Authored below with sacrifice_other.
//! Back-face upkeep trigger "sacrifice a Human. If you can't, tap this creature and it deals 9
//! damage to you" is wired via Effect::Conditional (ControlPermanentMatching a Human → sacrifice,
//! otherwise tap + 9 damage to controller).

use arcana_core::effects::{Condition, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Demon");
    let demon_sub = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon_sub);

    // Pre-intern "Human" for sacrifice_other filter
    let human_sub = reg.interner_mut().intern("Human");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Archdemon of Greed");
    let mut back_subtypes = SubtypeSet::default();
    let back_demon_sub = reg.interner_mut().intern("Demon");
    back_subtypes.0.insert(back_demon_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
            power: Some(PtValue::Fixed(9)),
            toughness: Some(PtValue::Fixed(9)),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Build human filter for sacrifice_other cost
    let human_filter = ObjectFilter::creature().with_subtypes_any(vec![human_sub]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: Sacrifice a Human: Transform this creature. (Sorcery speed)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Human: Transform this creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(human_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // Back face only: at the beginning of your upkeep, sacrifice a Human. If you can't,
            // tap this creature and it deals 9 damage to you.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_upkeep_sacrifice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1), // back face only
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn back_upkeep_sacrifice(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let human_filter = ObjectFilter::creature()
        .with_subtypes_any(vec![human])
        .controlled_by(ControllerConstraint::You);
    vec![Effect::Conditional {
        condition: Condition::ControlPermanentMatching(human_filter.clone()),
        then: Box::new(Effect::Sacrifice {
            player: trig.controller,
            filter: human_filter,
            count: 1,
        }),
        // "If you can't": tap this creature and it deals 9 damage to you.
        otherwise: Some(Box::new(Effect::Sequence(vec![
            Effect::Tap { target: trig.source },
            Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Player(trig.controller),
                amount: 9,
            },
        ]))),
    }]
}

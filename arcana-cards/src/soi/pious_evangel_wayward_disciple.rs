//! Pious Evangel // Wayward Disciple — `{2}{W}` transforming DFC (Human Cleric).
//! Front (Pious Evangel — Creature — Human Cleric, 2/2):
//!   Whenever this creature or another creature you control enters, you gain 1 life.
//!   {2}, {T}, Sacrifice another permanent: Transform this creature.
//! Back (Wayward Disciple — Creature — Human Cleric, 2/2):
//!   Whenever this creature or another creature you control dies, target opponent
//!   loses 1 life and you gain 1 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pious Evangel");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wayward Disciple");
    let back_human = reg.interner_mut().intern("Human");
    let back_cleric = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human);
    back_subtypes.0.insert(back_cleric);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): ETB of this or another creature you control → gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_one_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back (face 1): death of this or another creature you control →
            // target opponent loses 1 life and you gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: drain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            // Front-face activated ability: {2}, {T}, Sacrifice another permanent:
            // Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice another permanent: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter::permanent()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_self,
            })
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn gain_one_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}

fn drain_one(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife {
            player: *p,
            amount: 1,
        },
        Effect::GainLife {
            player: trig.controller,
            amount: 1,
        },
    ]
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

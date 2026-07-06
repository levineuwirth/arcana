//! Catapult Fodder // Catapult Captain — `{2}{B}` Zombie 1/5.
//! Front face: At the beginning of combat on your turn, if you control three
//!   or more creatures that each have toughness greater than their power,
//!   transform this creature.
//! Back face (Catapult Captain): {2}{B}, {T}, Sacrifice another creature:
//!   Target opponent loses life equal to the sacrificed creature's toughness.
//!
//! The back-face activated ability is wired on the shared CardDefinition and
//! gated to the back face via `face_gate: Some(1)`.
//!
//! Front-face transform condition "three or more creatures where toughness >
//! power" is expressed as an `intervening_if` over
//! `ObjectFilter::creature().with_pt_compare(PtCompare::ToughnessGreater)`.
//! GAP: Back-face life-loss amount "equal to the sacrificed creature's
//!      toughness" — the sacrificed creature is gone by resolution time and
//!      ActivationContext exposes no sacrificed-object accessor; 1 life loss
//!      is emitted as a placeholder (amount materially wrong).

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, PtCompare, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Catapult Fodder");
    let zombie_sub = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Catapult Captain");
    let back_zombie_sub = reg.interner_mut().intern("Zombie");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_zombie_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(5)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: at beginning of combat on your turn, (conditionally) transform.
            // Intervening-if "if you control 3+ creatures where toughness > power"
            // expressed via with_pt_compare(ToughnessGreater).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_three_toughness_over_power),
                effect: front_combat_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: {2}{B}, {T}, Sacrifice another creature: Target opponent loses life.
            // Wired on the shared CardDefinition, gated to the back face (face_gate: Some(1)).
            // GAP: life loss amount should equal sacrificed creature's toughness (not expressible
            //      — sacrificed object gone by resolution, no ActivationContext accessor).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}, Sacrifice another creature: Target opponent loses life equal to the sacrificed creature's toughness.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                // "target opponent" — using target_player(); no built-in opponent restriction.
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: back_activated,
            })
    )
}

/// "if you control three or more creatures that each have toughness greater
/// than their power" — power-vs-toughness compare on each creature you control.
fn iif_three_toughness_over_power(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_at_least(
        state,
        you,
        &ObjectFilter::creature().with_pt_compare(PtCompare::ToughnessGreater),
        3,
    )
}

fn front_combat_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn back_activated(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: life loss should equal sacrificed creature's toughness, which cannot
    //      be queried after the sacrifice cost is paid. Emitting 1 as placeholder.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    // GAP: dynamic amount (sacrificed creature's toughness) not expressible;
    //      effect body is correct structure but amount is wrong.
    vec![
        // GAP: amount should be sacrificed creature's toughness at cost payment time.
        Effect::LoseLife { player: *p, amount: 1 },
    ]
}

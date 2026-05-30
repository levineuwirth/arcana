//! Catapult Fodder // Catapult Captain — `{2}{B}` Zombie 1/5.
//! Front face: At the beginning of combat on your turn, if you control three
//!   or more creatures that each have toughness greater than their power,
//!   transform this creature.
//! Back face (Catapult Captain): {2}{B}, {T}, Sacrifice another creature:
//!   Target opponent loses life equal to the sacrificed creature's toughness.
//!
//! GAP: Front-face transform condition "three or more creatures where
//!      toughness > power" cannot be expressed in TriggerCondition; the
//!      trigger fires unconditionally at the beginning of combat.
//! GAP: Back-face activated ability "loses life equal to the sacrificed
//!      creature's toughness" — the sacrificed creature is gone by resolution
//!      time and its toughness cannot be queried; 1 life loss is emitted as
//!      a placeholder (materially wrong — the whole effect should be GAP'd
//!      but a fixed small amount is less disruptive).
//! GAP: back-face-only activated ability not auto-installed on transform.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
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
            // GAP: "if you control 3+ creatures where toughness > power" not expressible;
            //      fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_combat_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: {2}{B}, {T}, Sacrifice another creature: Target opponent loses life.
            // GAP: back-face-only ability not auto-installed on transform.
            // GAP: life loss amount should equal sacrificed creature's toughness (not expressible).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}, {T}, Sacrifice another creature: Target opponent loses life equal to the sacrificed creature's toughness.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(ObjectFilter::creature()),
                    ..ActivationCost::default()
                },
                // "target opponent" — using target_player(); no built-in opponent restriction.
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back face only
                effect: back_activated,
            })
    )
}

fn front_combat_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only fire if you control 3+ creatures where toughness > power.
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

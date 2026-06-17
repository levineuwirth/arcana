//! Linvala, Shield of Sea Gate — `{1}{W}{U}` 3/3 Legendary Angel Wizard with Flying.
//! "At the beginning of combat on your turn, if you have a full party, choose target
//! nonland permanent an opponent controls. Until your next turn, it can't attack or
//! block, and its activated abilities can't be activated."
//! "Sacrifice Linvala: Choose hexproof or indestructible. Creatures you control gain
//! that ability until end of turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Linvala, Shield of Sea Gate");
    let angel = reg.interner_mut().intern("Angel");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: intervening-if "if you have a full party" not expressible with the
            // available conditions:: helpers — trigger fires unconditionally.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: lock_down_permanent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // GAP: "Choose hexproof or indestructible. Creatures you control gain that
            // ability until end of turn." — modal keyword choice + mass keyword grant not
            // expressible with the available Effect set.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice Linvala: Choose hexproof or indestructible. Creatures you control gain that ability until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_ability,
            }),
    )
}

fn lock_down_permanent(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // Fidelity gap: oracle is "until your next turn" + "activated abilities can't be
    // activated"; we express the can't-attack/can't-block portion at end-of-turn duration.
    vec![
        Effect::ForbidAttacking { target: *id, duration: Duration::EndOfTurn },
        Effect::ForbidBlocking { target: *id, duration: Duration::EndOfTurn },
    ]
}

fn grant_ability(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: choose-one keyword (hexproof/indestructible) granted to all creatures you
    // control — no mass keyword-grant Effect available.
    Vec::new()
}

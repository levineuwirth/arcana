//! Assaultron Dominator — `{1}{R}` 2/2 Artifact Creature — Robot.
//! "When this creature enters, you get {E}{E}. Whenever an artifact
//! creature you control attacks, you may pay {E}. If you do, put your
//! choice of a +1/+1, first strike, or trample counter on that creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Assaultron Dominator");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attack_pay_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_energy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainEnergy { player: trig.controller, amount: 2 }]
}

fn attack_pay_energy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}. If you do, put your choice of a +1/+1, first
    // strike, or trample counter on that creature." Energy is not a payable
    // cost (OptionalPaymentKind has Mana/Life/Sacrifice/Discard but no energy
    // spend), and the choose-a-counter-kind payoff has no primitive. Omitted.
    Vec::new()
}

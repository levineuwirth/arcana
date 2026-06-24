//! Robobrain War Mind — `{3}{U}` */5 Artifact Creature — Robot.
//! "Robobrain War Mind's power is equal to the number of cards in your
//!  hand."
//! "When this creature enters, you get an amount of {E} equal to the
//!  number of artifact creatures you control."
//! "Whenever this creature attacks, you may pay {E}{E}{E}. If you do,
//!  draw a card."
//!
//! Power is recorded as `*` (Star); the CDA "power = cards in hand" is
//! wired at Layer 7a via `ContinuousEffect::self_pt_cda` on a
//! `SelfEntersBattlefield` trigger (only power is `*`, so the compute
//! returns the printed fixed toughness 5 as the second tuple element).
//! The ETB energy gain is wired fully with a dynamic amount. The attacks
//! trigger is wired but its body is GAP'd: OptionalPayment only supports
//! Mana / Life, not an energy spend.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Robobrain War Mind");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
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
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attacks_may_pay,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Robobrain War Mind's power is equal to the number of cards in your
/// hand" — install the self-CDA at Layer 7a. Only power is `*`, so the
/// compute returns the printed fixed toughness (5).
fn install_cda(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            hand_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

/// Power = cards in your hand; toughness fixed 5.
fn hand_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = script::hand_size(s, who) as i32;
    (n, 5)
}

fn etb_energy(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: n,
    }]
}

fn attacks_may_pay(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "you may pay {E}{E}{E}. If you do, draw a card." —
    // OptionalPayment supports only Mana / Life, not an energy spend.
    Vec::new()
}

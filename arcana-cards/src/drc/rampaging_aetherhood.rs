//! Rampaging Aetherhood — `{4}{G}` 4/4 green Snake Hydra with Trample
//! and Ward {2}.
//!
//! "At the beginning of your upkeep, you get an amount of {E} equal to
//! this creature's power. Then you may pay one or more {E}. If you do,
//! put that many +1/+1 counters on this creature." — the energy GAIN
//! (dynamic, equal to this creature's power) is expressible via
//! `Effect::GainEnergy` with a `script::power_of` amount. The "may pay
//! one or more {E}, put that many +1/+1 counters" rider is NOT
//! expressible: `OptionalPaymentKind` has only Mana/Life (no energy
//! payment), and "that many" depends on a variable energy spend the
//! engine can't model. That clause is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rampaging Aetherhood");
    let snake = reg.interner_mut().intern("Snake");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_gain_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_gain_energy(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you get an amount of {E} equal to this creature's power"
    let n = script::power_of(state, trig.source).max(0) as u32;
    // GAP: "Then you may pay one or more {E}. If you do, put that many
    // +1/+1 counters on this creature." — OptionalPaymentKind has no
    // energy variant, and "that many" is a variable-spend amount the
    // engine can't model. Only the energy gain is emitted.
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: n,
    }]
}

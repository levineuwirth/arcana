//! Inversion Behemoth — `{2}{C}{C}` 2/9 colorless Creature — Eldrazi.
//! "At the beginning of combat on your turn, switch the power and toughness
//! of each of any number of target creatures until end of turn."
//! PhaseBegins{Combat, You} trigger with TargetCount::Any; effect resolves
//! by reading each target's current power/toughness and emitting SetBasePT
//! with values swapped.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inversion Behemoth");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{C}{C}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(9)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: switch_pt_all_targets,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Any,
                    controller: None,
                }],
            }),
    )
}

fn switch_pt_all_targets(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets.targets.iter().filter_map(|t| {
        if let TargetChoice::Object(id) = t {
            let p = script::power_of(state, *id);
            let t_val = script::toughness_of(state, *id);
            Some(Effect::SetBasePT {
                target: *id,
                power: t_val,
                toughness: p,
                duration: Duration::EndOfTurn,
            })
        } else {
            None
        }
    }).collect()
}

//! Canopy Gargantuan — `{5}{G}{G}` 7/7 Dragon with Flying and Ward {2}.
//! "At the beginning of your upkeep, put a number of +1/+1 counters on each
//!  other creature you control equal to that creature's toughness."
//!
//! Flying + Ward {2} are base keywords. The upkeep trigger enumerates each
//! other creature you control and adds +1/+1 counters equal to that creature's
//! own toughness (per-id dynamic amount, built as a Sequence of AddCounters).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Canopy Gargantuan");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_counters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn upkeep_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects = Vec::new();
    for id in ids {
        if id == trig.source {
            continue; // "each OTHER creature you control"
        }
        let t = script::toughness_of(state, id).max(0) as u32;
        if t > 0 {
            effects.push(Effect::AddCounters {
                target: id,
                kind: CounterKind::PlusOnePlusOne,
                count: t,
            });
        }
    }
    if effects.len() <= 1 {
        effects
    } else {
        vec![Effect::Sequence(effects)]
    }
}

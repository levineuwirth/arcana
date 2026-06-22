//! Betor, Ancestor's Voice — `{2}{W}{B}{G}` 3/5 Legendary Creature —
//! Spirit Dragon. Flying, lifelink.
//! "At the beginning of your end step, put a number of +1/+1 counters
//! on up to one other target creature you control equal to the amount
//! of life you gained this turn. Return up to one target creature card
//! with mana value less than or equal to the amount of life you lost
//! this turn from your graveyard to the battlefield."
//!
//! Flying + lifelink are base keywords. The end-step trigger carries
//! two targets: (1) up to one other creature you control receives a
//! number of +1/+1 counters equal to life gained this turn (dynamic
//! amount via script::life_gained_this_turn); (2) up to one creature
//! card in your graveyard is returned to the battlefield. The
//! "mana value ≤ life lost this turn" cap on the reanimation target
//! can't be enforced declaratively at target-selection time (dynamic
//! cmc bound) — see GAP.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Betor, Ancestor's Voice");
    let spirit = reg.interner_mut().intern("Spirit");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: betor_end_step,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![
                // up to one OTHER target creature you control
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
                // up to one target creature card in your graveyard
                // GAP: "mana value ≤ life lost this turn" — dynamic cmc
                // bound can't be enforced at target-selection time.
                TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::UpTo(1),
                    controller: None,
                },
            ],
        }),
    )
}

fn betor_end_step(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = Vec::new();
    let gained = script::life_gained_this_turn(state, trig.controller);

    // Counters on the chosen creature (target 0), if any.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        if gained > 0 {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: gained,
            });
        }
    }

    // Reanimate the chosen graveyard creature card (target 1), if any.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.get(1) {
        effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
    }

    effects
}

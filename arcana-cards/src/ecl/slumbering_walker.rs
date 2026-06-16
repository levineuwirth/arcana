//! Slumbering Walker — `{3}{W}{W}` 4/7 white Giant Warrior.
//! "This creature enters with two -1/-1 counters on it."
//! "At the beginning of your end step, you may remove a counter from this
//! creature. When you do, return target creature card with power 2 or less
//! from your graveyard to the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slumbering Walker");
    let giant = reg.interner_mut().intern("Giant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: "enters with two -1/-1 counters on it" — no enters-with-counters
    //      Effect in this surface.
    let gy_creature = ObjectFilter::creature().with_max_power(2);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: reanimate_small,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: gy_creature,
                },
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn reanimate_small(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // GAP (fidelity): the "you may remove a counter" is an optional cost
    //      gating the reflexive return; modeled here as removing one -1/-1
    //      counter then unconditionally returning the target.
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::MinusOneMinusOne,
            count: 1,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
    ]
}

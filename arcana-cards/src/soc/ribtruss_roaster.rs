//! Ribtruss Roaster — `{4}{G}` 3/3 Troll Druid with Devour 1.
//! "At the beginning of your end step, create a number of 1/1 black and green
//! Pest creature tokens equal to the number of +1/+1 counters on this
//! creature. They have 'When this token dies, you gain 1 life.'"

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ribtruss Roaster");
    let troll = reg.interner_mut().intern("Troll");
    let druid = reg.interner_mut().intern("Druid");
    let _pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Devour(1)],
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
            effect: end_step_pests,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_pests(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let pest = reg.interner().lookup("Pest").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pest);
    // GAP: the token's "When this token dies, you gain 1 life" granted
    // triggered ability is not authored on the TokenDefinition (abilities
    // left empty); the 1/1 black-and-green Pest body is created faithfully.
    (0..n)
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: pest,
                colors: ColorSet::black() | ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}

//! Lucky the Pizza Dog — `{1}{G}` 2/2 Legendary Dog.
//!
//! Oracle:
//! * Whenever you cast a Cat, Dog, or Hero spell, create a Food token.
//! * At the beginning of each end step, if you gained life this turn, put
//!   a +1/+1 counter on Lucky.
//!
//! "Food" in the Scryfall keyword line is reminder/templating only (the
//! Food token itself is minted by the first trigger), so the keyword line
//! is empty. The end-step trigger carries an intervening-if for
//! "if you gained life this turn".

use arcana_core::conditions;
use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lucky the Pizza Dog");
    let dog = reg.interner_mut().intern("Dog");
    let cat = reg.interner_mut().intern("Cat");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: arcana_core::types::SupertypeSet(
            arcana_core::types::SupertypeSet::LEGENDARY,
        ),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let cat_dog_hero = arcana_core::targets::ObjectFilter::new()
        .with_subtypes_any(vec![cat, dog, hero]);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(cat_dog_hero),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_food,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_you_gained_life),
                effect: grow_lucky,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_food(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}

fn if_you_gained_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_gained_life_this_turn(s, you)
}

fn grow_lucky(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

//! Jenny Flint — `{1}{U}{R}` 2/2 Legendary Human Detective.
//! Partner with Madame Vastra.
//! First strike.
//! Training.
//! Whenever you sacrifice a Clue or Food, put a +1/+1 counter on another
//! target creature you control.
//!
//! First strike is a base keyword. Partner with and Partner are not
//! emittable KeywordAbilities — GAP'd. Training (its "attacks with another
//! creature of greater power → +1/+1 counter" trigger) has no expressible
//! "greater power" attack condition — GAP'd.
//! The Clue/Food sacrifice trigger is wired: it puts a +1/+1 counter on a
//! target creature you control.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jenny Flint");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let clue = reg.interner_mut().intern("Clue");
    let food = reg.interner_mut().intern("Food");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: Partner with Madame Vastra / Partner — not emittable keywords.
    // GAP: Training — no expressible "attacks with a greater-power creature"
    // condition.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: ObjectFilter::permanent()
                    .with_subtypes_any(vec![clue, food])
                    .controlled_by(ControllerConstraint::You),
            },
            intervening_if: None,
            effect: sac_clue_or_food,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn sac_clue_or_food(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

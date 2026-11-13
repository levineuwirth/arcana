//! Voracious Hydra — `{X}{G}{G}` 0/1 Hydra with Trample.
//!
//! Trample is a base keyword. `EntersWithSpec::CountersFromX` adds X +1/+1
//! counters as the Hydra enters, using the cast's X.
//!
//! Its entry trigger chooses between doubling its +1/+1 counters and
//! fighting a target creature its controller doesn't control. Modes are
//! selected at stack placement; only the fight mode targets.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerMode,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voracious Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX { kind: CounterKind::PlusOnePlusOne })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_modes(1, vec![
                TriggerMode {
                    description: "Double the +1/+1 counters".into(),
                    target_requirements: Vec::new(),
                    effect: double_counters,
                },
                TriggerMode {
                    description: "Fight target creature you don't control".into(),
                    target_requirements: vec![TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent)),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    }],
                    effect: fight,
                },
            ]),
    )
}

// The engine dispatches the chosen clause; this ordinary callback is unused.
fn etb_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}

fn double_counters(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(source) = state.objects.get(trig.source).filter(|o| o.zone.is_battlefield())
        else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source, kind: CounterKind::PlusOnePlusOne,
        count: source.count_counters(CounterKind::PlusOnePlusOne),
    }]
}

fn fight(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    match trig.targets.targets.first() {
        Some(arcana_core::targets::TargetChoice::Object(target)) =>
            vec![Effect::Fight { a: trig.source, b: *target }],
        _ => Vec::new(),
    }
}

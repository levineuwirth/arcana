//! Phylactery Lich — `{B}{B}{B}` 5/5 Zombie with Indestructible.
//! "As this creature enters, put a phylactery counter on an artifact you
//!  control."
//! "When you control no permanents with phylactery counters on them,
//!  sacrifice this creature." (state trigger — GAP)
//!
//! The enters-with-counter is modeled as a targeted ETB trigger placing a
//! named "phylactery" counter on a target artifact you control. The
//! state-based "when you control no permanents with phylactery counters,
//! sacrifice" trigger has no expressible TriggerCondition and is GAP'd.

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
    let name = reg.interner_mut().intern("Phylactery Lich");
    let zombie = reg.interner_mut().intern("Zombie");
    let _phylactery = reg.interner_mut().intern("phylactery");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP (state trigger): "When you control no permanents with phylactery
    // counters on them, sacrifice this creature." — no TriggerCondition
    // models a state-based "you control none with counter X" trigger.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_phylactery_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_phylactery_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(kind) = reg.interner().lookup("phylactery").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: *id, kind, count: 1 }]
}

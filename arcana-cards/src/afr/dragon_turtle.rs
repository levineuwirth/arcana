//! Dragon Turtle — `{1}{U}{U}` 3/5 blue Dragon Turtle with Flash.
//!
//! Drag Below — When this creature enters, tap it and up to one target
//! creature an opponent controls. They don't untap during their
//! controllers' next untap steps.
//!
//! The "don't untap during next untap step" rider is modeled with a Stun
//! counter on each tapped creature (CR 122.1c: a Stun counter is removed
//! the next time the permanent would untap, skipping that untap), which is
//! the engine's faithful representation of "doesn't untap next untap step".

use arcana_core::effects::Effect;
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
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dragon Turtle");
    let dragon = reg.interner_mut().intern("Dragon");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    subtypes.0.insert(turtle);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: drag_below,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn drag_below(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut out = vec![
        Effect::Tap { target: trig.source },
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Stun,
            count: 1,
        },
    ];
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        out.push(Effect::Tap { target: *id });
        out.push(Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        });
    }
    out
}

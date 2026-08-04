//! Swarm Shambler — `{G}` 0/0 Fungus Beast.
//!
//! This creature enters with a +1/+1 counter on it.
//! Whenever a creature you control with a +1/+1 counter on it becomes the
//! target of a spell an opponent controls, create a 1/1 green Insect
//! creature token.
//! {1}, {T}: Put a +1/+1 counter on this creature.
//!
//! Decomposed as: an ETB trigger (enters with a +1/+1 counter), a
//! becomes-target trigger (GAP'd — see below), and one activated ability.
//! The becomes-target trigger watches *any* creature you control with a
//! +1/+1 counter; the engine only models `SelfBecomesTarget` (the watcher
//! must be the source itself, with no counter precondition), so this
//! trigger is GAP'd rather than approximated incorrectly.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swarm Shambler");
    let fungus = reg.interner_mut().intern("Fungus");
    let beast = reg.interner_mut().intern("Beast");
    let _insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: "Whenever a creature you control with a +1/+1 counter on it
    // becomes the target of a spell an opponent controls, create a 1/1
    // green Insect token." — the engine's only becomes-target trigger is
    // SelfBecomesTarget (watcher must be the source, no counter gate), so a
    // board-wide watcher with a counter precondition is not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter_self,
            }),
    )
}

fn enters_with_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn add_counter_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

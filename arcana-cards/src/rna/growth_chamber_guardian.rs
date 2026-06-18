//! Growth-Chamber Guardian — `{1}{G}` 2/2 Elf Crab Warrior.
//! {2}{G}: Adapt 2 (put two +1/+1 counters on it if it has none — the
//!   "if it has no +1/+1 counters" gate is GAP'd; we add the counters
//!   unconditionally as there is no max-self-counter precondition).
//! Whenever one or more +1/+1 counters are put on this creature, you may
//!   search your library for a card named Growth-Chamber Guardian, reveal it,
//!   put it into your hand, then shuffle.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Growth-Chamber Guardian");
    let elf = reg.interner_mut().intern("Elf");
    let crab = reg.interner_mut().intern("Crab");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(crab);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Adapt 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_two,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: search_for_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn adapt_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if this creature has no +1/+1 counters on it" — no max-self-counter
    // precondition is available; counters are added unconditionally.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn search_for_copy(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("Growth-Chamber Guardian");
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter: ObjectFilter { name: nm, ..ObjectFilter::default() },
        reveal: true,
    }]
}

//! Realm Seekers — `{4}{G}{G}` 0/0 Creature — Elf Scout.
//! * "This creature enters with X +1/+1 counters on it, where X is the total
//!   number of cards in all players' hands." Wired as a SelfEntersBattlefield
//!   trigger that puts that many +1/+1 counters on itself (resolution-time X).
//! * "{2}{G}, Remove a +1/+1 counter from this creature: Search your library
//!   for a land card, reveal it, put it into your hand, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Realm Seekers");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}, Remove a +1/+1 counter from this creature: Search your library for a land card, reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tutor_land,
            }),
    )
}

fn enter_with_counters(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // X = total cards in all players' hands.
    let x: u32 = script::all_players(state)
        .into_iter()
        .map(|p| script::hand_size(state, p))
        .sum();
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: x,
    }]
}

fn tutor_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        reveal: true,
    }]
}

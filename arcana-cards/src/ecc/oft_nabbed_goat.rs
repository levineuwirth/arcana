//! Oft-Nabbed Goat — `{1}{B}` 0/5 Goat.
//! {1}: Draw a card. Gain control of this creature and put a -1/-1 counter on
//! it. Only your opponents may activate this ability and only as a sorcery.
//! When this creature dies, if it had one or more -1/-1 counters on it, its
//! owner draws that many cards and each other player loses that much life.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oft-Nabbed Goat");
    let goat = reg.interner_mut().intern("Goat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Only your opponents may activate this ability and only
                // as a sorcery" (activator restriction) is not expressible; the
                // draw / gain-control / counter effect is implemented.
                text: "{1}: Draw a card. Gain control of this creature and put a -1/-1 counter on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: nab,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: Some(if_has_minus_counter),
                effect: dies_payoff,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn nab(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::ChangeControl { target: ctx.source, new_controller: ctx.controller },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::MinusOneMinusOne,
            count: 1,
        },
    ]
}

fn if_has_minus_counter(
    s: &GameState,
    source: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_counters_at_least(s, source, CounterKind::MinusOneMinusOne, 1)
}

fn dies_payoff(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let id = trig.dying_object().unwrap_or(trig.source);
    let n = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::MinusOneMinusOne));
    if n == 0 {
        return Vec::new();
    }
    let mut effects = vec![Effect::DrawCards { player: trig.controller, count: n }];
    for p in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: p, amount: n });
    }
    effects
}

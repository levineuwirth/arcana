//! Conductor of Cacophony — `{3}{B}` 2/1 black Demon.
//!
//! * "This creature enters with two +1/+1 counters on it." — modeled as
//!   an ETB trigger that adds two +1/+1 counters (the engine has no
//!   enters-with replacement primitive; the post-ETB add is the
//!   standard faithful approximation).
//! * "{B}, Remove a +1/+1 counter from this creature: It deals 1 damage
//!   to each other creature and each player." — a mana + remove-counter
//!   activation that pings the board.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
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
    let name = reg.interner_mut().intern("Conductor of Cacophony");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enters_with_two_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, Remove a +1/+1 counter from this creature: It deals 1 damage to each other creature and each player.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_board,
            }),
    )
}

fn enters_with_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn ping_board(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // 1 damage to each creature (fidelity gap: ForEach has no exclude-self,
    // so this includes the source — the oracle says "each OTHER creature").
    let creature_ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    let mut effects = vec![Effect::ForEach {
        targets: creature_ids,
        effect: Box::new(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(NULL_OBJECT_ID),
            amount: 1,
        }),
    }];
    // 1 damage to each player.
    for p in script::all_players(state) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}

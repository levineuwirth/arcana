//! Ajani, Sleeper Agent — `{1}{G}{G/W/P}{W}` Legendary Planeswalker — Ajani,
//! starting loyalty 3. Colors G/W.
//!
//! +1: Reveal the top card of your library. If it's a creature or planeswalker
//!   card, put it into your hand. Otherwise, you may put it on the bottom.
//!   Modeled with `Effect::DigTopN { count: 1, filter: creature-or-planeswalker,
//!   rest: BottomRandom }` — the optional pick takes a matching card to hand,
//!   the rest go to the bottom.
//! −3: Distribute three +1/+1 counters among up to three target creatures; they
//!   gain vigilance. GAP: "distribute N counters among up to K targets" has no
//!   demonstrated Effect surface (no DistributeCounters). Ability shell declared
//!   with the correct −3 cost; effect GAP'd.
//! −6: emblem ("Whenever you cast a creature or planeswalker spell, target
//!   opponent gets two poison counters."). GAP: giving a PLAYER poison counters
//!   is not expressible (`Effect::AddCounters` targets an object, not a player).
//!   Emblem shell declared with the cast trigger; effect GAP'd.
//!
//! Compleated ({G/W/P}) is a casting/ETB-loyalty modifier not modeled by the
//! demonstrated surface; the printed starting loyalty (3) is recorded.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::{DigRest};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ajani, Sleeper Agent");
    let ajani = reg.interner_mut().intern("Ajani");
    let _emblem = reg.interner_mut().intern("Ajani, Sleeper Agent emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ajani);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G/W/P}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top card of your library. If it's a \
                       creature or planeswalker card, put it into your hand. \
                       Otherwise, you may put it on the bottom of your \
                       library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_reveal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Distribute three +1/+1 counters among up to three \
                       target creatures. They gain vigilance until end of \
                       turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_distribute,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Whenever you cast a creature \
                       or planeswalker spell, target opponent gets two poison \
                       counters.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_reveal(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DigTopN {
        player: ctx.controller,
        count: 1,
        filter: Some(ObjectFilter::new().with_types_any(
            (TypeLine::CREATURE | TypeLine::PLANESWALKER).into(),
        )),
        rest: DigRest::BottomRandom,
    }]
}

fn minus_three_distribute(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: distribute three +1/+1 counters among up to three target creatures
    //      (no DistributeCounters effect; the vigilance grant rides the same
    //      distribution choice) — ability shell declared with −3 cost.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Ajani, Sleeper Agent emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types_any(
                        (TypeLine::CREATURE | TypeLine::PLANESWALKER).into(),
                    )),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_poison,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_poison(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target opponent gets two poison counters" — giving a PLAYER poison
    //      counters is not expressible (AddCounters targets an object).
    Vec::new()
}

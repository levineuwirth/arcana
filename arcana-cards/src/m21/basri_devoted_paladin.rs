//! Basri, Devoted Paladin — `{4}{W}{W}` Legendary Planeswalker — Basri,
//! starting loyalty 5.
//!
//! * `+1`: Put a +1/+1 counter on up to one target creature; it gains
//!   vigilance until end of turn. `AddCounters` + `GrantKeyword`.
//! * `−1`: Whenever a creature attacks this turn, put a +1/+1 counter on
//!   it. GAP'd (floating "this turn" attack trigger not expressible).
//! * `−6`: Creatures you control get +2/+2 and gain flying until end of
//!   turn. The +2/+2 is expressed via `Effect::Anthem`; the board-wide
//!   "gain flying" keyword grant is GAP'd (no demonstrated surface).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Basri, Devoted Paladin");
    let basri = reg.interner_mut().intern("Basri");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a +1/+1 counter on up to one target creature. \
                       It gains vigilance until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Whenever a creature attacks this turn, put a +1/+1 \
                       counter on it.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_attack_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: Creatures you control get +2/+2 and gain flying until \
                       end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_overrun,
            }),
    )
}

fn plus_one_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    vec![
        Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_one_attack_counters(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a floating "whenever a creature attacks this turn, put a +1/+1
    // counter on it" trigger window is not expressible from the activated
    // ability surface.
    Vec::new()
}

fn minus_six_overrun(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide "gain flying until end of turn" keyword grant is not
    // expressible; the +2/+2 portion is via Anthem.
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
    }]
}

//! Basri Ket — `{1}{W}{W}` Legendary Planeswalker — Basri,
//! starting loyalty 3.
//!
//! * `+1`: Put a +1/+1 counter on up to one target creature; it gains
//!   indestructible until end of turn. Expressed via `AddCounters` +
//!   `GrantKeyword(Indestructible, EndOfTurn)`.
//! * `−2`: Whenever one or more nontoken creatures attack this turn,
//!   create that many tapped-and-attacking 1/1 Soldiers. GAP'd (a
//!   floating delayed combat trigger counting attackers is not
//!   expressible from the demonstrated activated-ability surface).
//! * `−6`: Emblem — GAP'd.

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
    let name = reg.interner_mut().intern("Basri Ket");
    let basri = reg.interner_mut().intern("Basri");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(basri);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Put a +1/+1 counter on up to one target creature. \
                       It gains indestructible until end of turn.".into(),
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
                text: "−2: Whenever one or more nontoken creatures attack this \
                       turn, create that many 1/1 white Soldier creature \
                       tokens that are tapped and attacking.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_soldiers,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−6: You get an emblem with \"At the beginning of combat \
                       on your turn, create a 1/1 white Soldier creature \
                       token, then put a +1/+1 counter on each creature you \
                       control.\"".into(),
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
                effect: minus_six_emblem,
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
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_two_soldiers(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a floating "whenever nontoken creatures attack this turn, create
    // that many tapped-and-attacking tokens" delayed combat trigger is not
    // expressible from the demonstrated activated-ability surface.
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a begin-combat token + counter trigger.
    Vec::new()
}

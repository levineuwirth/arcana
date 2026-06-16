//! Geyadrone Dihada — `{1}{U}{B}{R}` legendary planeswalker, starting loyalty 5.
//! U/B/R planeswalker (subtype Dihada).
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; CR 606.3 — sorcery-speed, controller-only,
//!   once per turn per planeswalker. CR 704.5i — 0-loyalty sacrifice SBA.
//!
//! # Scope
//!
//! * Static "Protection from permanents with corruption counters on them" —
//!   filtered protection is not in the demonstrated keyword surface;
//!   `keywords: vec![]`, GAP'd (it is not a loyalty ability either).
//! * `+1`: Each opponent loses 2 life and you gain 2 life; put a corruption
//!   counter on up to one other target creature or planeswalker — expressed.
//! * `−3`: Gain control of target creature or planeswalker until end of turn,
//!   untap it, put a corruption counter on it, and grant haste — expressed.
//! * `−7`: Gain control of each permanent with a corruption counter on it —
//!   GAP (a board-wide per-each control sweep over a counter filter is not
//!   expressible from the demonstrated single-target effect surface).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Geyadrone Dihada");
    let dihada = reg.interner_mut().intern("Dihada");
    // Intern the corruption counter name so resolvers can look it up.
    let _corruption = reg.interner_mut().intern("corruption");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dihada);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: "Protection from permanents with corruption counters on them"
        // is a filtered protection static not in the demonstrated surface.
        keywords: vec![],
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each opponent loses 2 life and you gain 2 life. Put \
                       a corruption counter on up to one other target creature \
                       or planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any((TypeLine::CREATURE | TypeLine::PLANESWALKER).into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_drain_corrupt,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Gain control of target creature or planeswalker \
                       until end of turn. Untap it and put a corruption counter \
                       on it. It gains haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types_any((TypeLine::CREATURE | TypeLine::PLANESWALKER).into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_steal,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Gain control of each permanent with a corruption \
                       counter on it.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_mass_control,
            }),
    )
}

/// `+1: Each opponent loses 2 life and you gain 2 life. Put a corruption
/// counter on up to one other target creature or planeswalker.`
fn plus_one_drain_corrupt(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, ctx.controller) {
        out.push(Effect::LoseLife { player: opp, amount: 2 });
    }
    out.push(Effect::GainLife { player: ctx.controller, amount: 2 });
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        if let Some(corruption) = reg.interner().lookup("corruption") {
            out.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::Named(corruption),
                count: 1,
            });
        }
    }
    out
}

/// `−3: Gain control of target creature or planeswalker until end of turn.
/// Untap it, put a corruption counter on it, and it gains haste until EOT.`
fn minus_three_steal(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    let mut out = vec![
        Effect::ChangeControlEot { target: id, new_controller: ctx.controller },
        Effect::Untap { target: id },
    ];
    if let Some(corruption) = reg.interner().lookup("corruption") {
        out.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::Named(corruption),
            count: 1,
        });
    }
    out.push(Effect::GrantKeyword {
        target: id,
        keyword: KeywordAbility::Haste,
        duration: Duration::EndOfTurn,
    });
    out
}

/// `−7: Gain control of each permanent with a corruption counter on it.`
fn minus_seven_mass_control(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a board-wide per-each control sweep filtered by counter presence
    // is not expressible from the demonstrated single-target effect surface.
    Vec::new()
}

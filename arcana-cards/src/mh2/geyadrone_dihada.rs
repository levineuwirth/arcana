//! Geyadrone Dihada — `{1}{U}{B}{R}` Legendary Planeswalker — Dihada.
//!
//! Static: "Protection from permanents with corruption counters on them"
//! — Protection is NOT in the usable keyword surface for this class, so
//! `keywords: vec![]` and the static is GAP'd.
//!
//! Loyalty abilities:
//! * `+1`: Each opponent loses 2 life and you gain 2 life. Put a
//!   corruption counter on up to one other target creature or
//!   planeswalker. (The "up to one OTHER" self-exclusion is not
//!   expressible; modeled as up-to-one target creature-or-planeswalker.)
//! * `−3`: Gain control of target creature or planeswalker until end of
//!   turn. Untap it and put a corruption counter on it. It gains haste
//!   until end of turn.
//! * `−7`: Gain control of each permanent with a corruption counter on
//!   it.
//!
//! Corruption counters are modeled as `CounterKind::Named("corruption")`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
        loyalty: Some(5),
        keywords: vec![], // GAP: Protection from permanents with corruption counters
        ..Default::default()
    };

    let creature_or_pw = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    };
    let creature_or_pw_one = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each opponent loses 2 life and you gain 2 life. Put a \
                       corruption counter on up to one other target creature or \
                       planeswalker.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![creature_or_pw],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Gain control of target creature or planeswalker until \
                       end of turn. Untap it and put a corruption counter on it. \
                       It gains haste until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![creature_or_pw_one],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Gain control of each permanent with a corruption \
                       counter on it.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

fn corruption_kind(reg: &CardRegistry) -> CounterKind {
    let sym = reg.interner().lookup("corruption").expect("corruption interned");
    CounterKind::Named(sym)
}

fn plus_one(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![
        Effect::GainLife { player: ctx.controller, amount: 2 },
    ];
    // Each opponent loses 2 life.
    for p in state.opponents_of(ctx.controller) {
        effects.push(Effect::LoseLife { player: p, amount: 2 });
    }
    if let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() {
        effects.push(Effect::AddCounters {
            target: *id,
            kind: corruption_kind(reg),
            count: 1,
        });
    }
    effects
}

fn minus_three(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::ChangeControlEot { target: *id, new_controller: ctx.controller },
        Effect::Untap { target: *id },
        Effect::AddCounters { target: *id, kind: corruption_kind(reg), count: 1 },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn minus_seven(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter {
        has_counter: Some(corruption_kind(reg)),
        ..ObjectFilter::permanent()
    };
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::ChangeControl { target: id, new_controller: ctx.controller })
        .collect()
}

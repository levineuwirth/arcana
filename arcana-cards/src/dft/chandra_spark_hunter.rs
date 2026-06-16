//! Chandra, Spark Hunter — `{3}{R}` Legendary Planeswalker — Chandra,
//! starting loyalty 4. Mono-red.
//!
//! Oracle:
//! At the beginning of combat on your turn, choose up to one target Vehicle you
//!     control. Until end of turn, it becomes an artifact creature and gains
//!     haste.
//! +2: You may sacrifice an artifact or discard a card. If you do, draw a card.
//! 0: Create a 3/2 colorless Vehicle artifact token with crew 1.
//! −7: You get an emblem with "Whenever an artifact you control enters, this
//!     emblem deals 3 damage to any target."
//!
//! # Scope
//! * Static combat trigger — modeled: at begin-combat on your turn, the chosen
//!   Vehicle gains the CREATURE type and Haste until end of turn (up-to-one
//!   target Vehicle you control).
//! * +2 — GAP: "you may sacrifice/discard; if you do, draw" optional-cost
//!   conditional is not expressible (only Mana/Life OptionalPaymentKind).
//! * 0 — partial: create a 3/2 colorless Vehicle artifact token. GAP: the
//!   "crew 1" ability is not in the demonstrated keyword surface.
//! * −7 — GAP: emblem creation not in Effect catalog.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Spark Hunter");
    let sub = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let vehicle_sub = reg.interner_mut().intern("Vehicle");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    let vehicle_target = TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent()
                .with_subtype_sym(vehicle_sub)
                .controlled_by(ControllerConstraint::You),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: begin_combat_animate,
                trigger_zones: vec![],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![vehicle_target],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: You may sacrifice an artifact or discard a card. If \
                       you do, draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Create a 3/2 colorless Vehicle artifact token with \
                       crew 1.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_vehicle_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Whenever an artifact you \
                       control enters, this emblem deals 3 damage to any \
                       target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_gap,
            }),
    )
}

/// Static: at begin combat on your turn, chosen Vehicle becomes an artifact
/// creature and gains haste until end of turn.
fn begin_combat_animate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let id = *id;
    vec![
        Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

/// `+2` — GAP: optional sacrifice/discard conditional draw.
fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice an artifact or discard a card; if you do, draw".
    Vec::new()
}

/// `0:` create a 3/2 colorless Vehicle artifact token (crew GAP'd).
fn zero_vehicle_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vehicle_sub = reg.interner().lookup("Vehicle").unwrap_or_default();
    let mut t_subtypes = SubtypeSet::default();
    t_subtypes.0.insert(vehicle_sub);
    // GAP: the "crew 1" ability is not expressible.
    let token = TokenDefinition {
        name: vehicle_sub,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: t_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token,
    }]
}

/// `−7` — GAP: emblem creation not in Effect catalog.
fn minus_seven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not in the demonstrated Effect surface.
    Vec::new()
}

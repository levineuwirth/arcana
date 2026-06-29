//! Kaito, Dancing Shadow — `{2}{U}{B}` Legendary Planeswalker — Kaito.
//! Starting loyalty 3.
//! Triggered: whenever creatures you control deal combat damage to a player,
//!   you may return one to hand; if you do, activate loyalty abilities
//!   twice this turn. (GAP: both rider effects)
//! +1: Up to one target creature can't attack or block until your next turn.
//! 0: Draw a card.
//! −2: Create a 2/2 colorless Drone artifact creature token with deathtouch
//!   and "When this token leaves the battlefield, each opponent loses 2 life
//!   and you gain 2 life." (token death trigger GAP'd)

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaito, Dancing Shadow");
    let kaito = reg.interner_mut().intern("Kaito");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kaito);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        loyalty: Some(3),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: on_combat_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature can't attack or block until your next turn.".into(),
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
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Draw a card.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Create a 2/2 colorless Drone artifact creature token with deathtouch.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_create_drone,
            }),
    )
}

fn on_combat_damage(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may return one of those creatures to its owner's hand; if you do,
    // activate loyalty abilities of Kaito twice this turn" — optional return from
    // a set of combat-damage dealers is not expressible; double-activation has no Effect
    Vec::new()
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new(); // "up to one" — zero targets is valid
    };
    vec![
        Effect::ForbidAttacking {
            target: *id,
            duration: Duration::UntilYourNextTurn(ctx.controller),
        },
        Effect::ForbidBlocking {
            target: *id,
            duration: Duration::UntilYourNextTurn(ctx.controller),
        },
    ]
}

fn zero_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn minus_two_create_drone(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let drone_name = reg.interner().lookup("Drone").unwrap_or_default();
    let mut drone_subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Drone") {
        drone_subtypes.0.insert(s);
    }
    vec![
        Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: drone_name,
                colors: ColorSet::new(),
                types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
                subtypes: drone_subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Deathtouch],
                abilities: vec![],
            },
        },
        // GAP: "When this token leaves the battlefield, each opponent loses 2 life
        // and you gain 2 life" — triggered ability embedded in TokenDefinition.abilities
        // is not expressible via the current API
    ]
}

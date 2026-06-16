//! Liliana, the Last Hope — `{1}{B}{B}` Legendary Planeswalker — Liliana, starting loyalty 3.
//! +1: Up to one target creature gets -2/-1 until your next turn. IMPLEMENTED (Effect::Pump with
//!     negative power/toughness, Duration::UntilYourNextTurn).
//! −2: Mill two cards, then you may return a creature card from your graveyard to your hand.
//!     PARTIAL — the mill is implemented; the may-return-from-graveyard needs a graveyard target
//!     and is GAP'd.
//! −7: You get an emblem with "At your end step, create X 2/2 black Zombie tokens, where X is two
//!     plus the number of Zombies you control." IMPLEMENTED as a triggered emblem creating a FIXED
//!     2 Zombie tokens; the dynamic-X portion is GAP'd.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, the Last Hope");
    let sub = reg.interner_mut().intern("Liliana");
    let zombie = reg.interner_mut().intern("Zombie");
    let _emblem = reg.interner_mut().intern("Liliana, the Last Hope emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let _ = zombie;

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets -2/-1 until your next turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_shrink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Mill two cards, then you may return a creature card from your graveyard \
                       to your hand."
                    .into(),
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
                effect: minus_two_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"At the beginning of your end step, create X \
                       2/2 black Zombie creature tokens, where X is two plus the number of \
                       Zombies you control.\""
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

/// `+1` — up to one target creature gets -2/-1 until your next turn.
fn plus_one_shrink(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: -1,
        duration: Duration::UntilYourNextTurn(ctx.controller),
        keywords: vec![],
    }]
}

/// `−2` — mill two; the may-return-creature-from-graveyard rider is GAP'd.
fn minus_two_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then you may return a creature card from your graveyard to your hand" needs a
    // graveyard target (no any-graveyard sentinel in this surface). Only the mill is emitted.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−7` — triggered emblem: at each of your end steps, create Zombie tokens.
fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Liliana, the Last Hope emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_make_zombies,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

/// Emblem trigger — create Zombie tokens. GAP: dynamic X ("two plus the number of Zombies you
/// control") is not expressible; a FIXED 2 tokens are created.
fn emblem_make_zombies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(zombie);
    let make = || Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: zombie,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes.clone(),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    };
    vec![make(), make()]
}

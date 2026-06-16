//! Liliana, the Last Hope — `{1}{B}{B}` Legendary Planeswalker — Liliana,
//! starting loyalty 3. Colors B.
//!
//! +1: Up to one target creature gets -2/-1 until your next turn.
//! −2: Mill two cards, then you may return a creature card from your graveyard
//!   to your hand. (Mill modeled faithfully; GAP the optional graveyard return.)
//! −7: You get an emblem with "At the beginning of your end step, create X 2/2
//!   black Zombie creature tokens, where X is two plus the number of Zombies you
//!   control." Triggered emblem (StepBegins End, You); the effect counts Zombies
//!   you control and creates 2 + that many 2/2 black Zombie tokens.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.
//! * CR 113.3c — entering with loyalty counters.
//!
//! # Scope / GAPs
//! * −2: the optional "return a creature card from your graveyard to your hand"
//!   needs an any-graveyard target (no sentinel in the demonstrated surface), so
//!   only the Mill 2 is modeled. GAP.

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, the Last Hope");
    let liliana = reg.interner_mut().intern("Liliana");
    let _zombie = reg.interner_mut().intern("Zombie");
    let _emblem = reg.interner_mut().intern("Liliana, the Last Hope emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(liliana);

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

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets -2/-1 until your next \
                       turn.".into(),
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
                text: "-2: Mill two cards, then you may return a creature card \
                       from your graveyard to your hand.".into(),
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
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"At the beginning of your end \
                       step, create X 2/2 black Zombie creature tokens, where X \
                       is two plus the number of Zombies you control.\"".into(),
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

/// `+1: Up to one target creature gets -2/-1 until your next turn.`
fn plus_one(
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

/// `-2: Mill two cards, then you may return a creature card from your graveyard
/// to your hand.`
fn minus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the optional "return a creature card from your graveyard to your
    //      hand" needs an any-graveyard target sentinel that the demonstrated
    //      surface doesn't provide. Only the Mill 2 is modeled.
    vec![Effect::Mill {
        player: ctx.controller,
        count: 2,
    }]
}

/// `-7: You get an emblem with "...create X 2/2 black Zombie tokens..."`
fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Liliana, the Last Hope emblem")
        .expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
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

/// Emblem end-step trigger: create (2 + Zombies you control) 2/2 black Zombies.
fn emblem_make_zombies(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie").expect("Zombie interned");
    let filter = script::subtype_filter(reg, "Zombie")
        .controlled_by(ControllerConstraint::You);
    let zombies = script::count_matching(state, &filter, trig.controller);
    let x = 2 + zombies;

    let mut st = SubtypeSet::default();
    st.0.insert(zombie);
    let token = TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };

    let mut effects = Vec::new();
    for _ in 0..x {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        });
    }
    effects
}

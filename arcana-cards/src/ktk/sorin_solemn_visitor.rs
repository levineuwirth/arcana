//! Sorin, Solemn Visitor — `{2}{W}{B}` Legendary Planeswalker — Sorin, starting
//! loyalty 4. Colors W/B.
//!
//! +1: Until your next turn, creatures you control get +1/+0 and gain lifelink.
//!   The +1/+0 is modeled via `Effect::Anthem` (UntilYourNextTurn). GAP: the
//!   team-wide lifelink grant has no demonstrated effect.
//! −2: Create a 2/2 black Vampire creature token with flying.
//! −6: emblem ("At the beginning of each opponent's upkeep, that player
//!   sacrifices a creature of their choice."). Triggered emblem: StepBegins
//!   (Upkeep, Opponent) → that player (the active player at resolution)
//!   sacrifices a creature.

use arcana_core::effects::{Effect, EmblemDefinition, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
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
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin, Solemn Visitor");
    let sorin = reg.interner_mut().intern("Sorin");
    let _vampire = reg.interner_mut().intern("Vampire");
    let _emblem = reg.interner_mut().intern("Sorin, Solemn Visitor emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, creatures you control get \
                       +1/+0 and gain lifelink.".into(),
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
                effect: plus_one_anthem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 2/2 black Vampire creature token with \
                       flying.".into(),
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
                effect: minus_two_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"At the beginning of each \
                       opponent's upkeep, that player sacrifices a creature of \
                       their choice.\"".into(),
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

fn plus_one_anthem(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "and gain lifelink" — no team-wide keyword-grant effect; the +1/+0
    //      half is modeled as an anthem until your next turn.
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 1,
        toughness: 0,
        duration: Duration::UntilYourNextTurn(ctx.controller),
    }]
}

fn minus_two_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let vampire = reg.interner().lookup("Vampire").expect("Vampire interned");
    let mut st = SubtypeSet::default();
    st.0.insert(vampire);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: vampire,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: st,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

fn minus_six_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Sorin, Solemn Visitor emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: emblem_sacrifice,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_sacrifice(
    state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "That player" = the opponent whose upkeep it is (the active player).
    vec![Effect::Sacrifice {
        player: state.active_player(),
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}

//! Kiora, the Crashing Wave — `{2}{G}{U}` Legendary Planeswalker — Kiora, starting loyalty 2.
//!
//! +1: Until your next turn, prevent all damage that would be dealt to and dealt
//!   by target permanent an opponent controls. Modeled with `Effect::PreventDamage`
//!   on the target (incoming damage). GAP: "dealt BY" the permanent (outgoing) is
//!   not pinnable to a single object id via the source-filtered surface, and the
//!   "until your next turn" duration is unavailable (ReplacementDuration has only
//!   EndOfTurn / Permanent / WhileSourceOnBattlefield) — EndOfTurn is used as the
//!   closest approximation.
//! −1: Draw a card. You may play an additional land this turn.
//! −5: You get an emblem with "At the beginning of your end step, create a 9/9
//!   blue Kraken creature token."

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
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
    let name = reg.interner_mut().intern("Kiora, the Crashing Wave");
    let kiora = reg.interner_mut().intern("Kiora");
    let _kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kiora);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(2),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, prevent all damage that would be \
                       dealt to and dealt by target permanent an opponent \
                       controls.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_prevent,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Draw a card. You may play an additional land this \
                       turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_draw_land,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: You get an emblem with \"At the beginning of your end \
                       step, create a 9/9 blue Kraken creature token.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_emblem,
            }),
    )
}

fn plus_one_prevent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    // GAP: "dealt by" (outgoing) and the "until your next turn" duration are not
    //      expressible; incoming damage prevention modeled at EndOfTurn.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(*id),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn minus_one_draw_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::PlayExtraLand { player: ctx.controller, amount: 1 },
    ]
}

fn minus_five_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Kiora, the Crashing Wave").expect("name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_make_kraken,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_make_kraken(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kraken = reg.interner().lookup("Kraken").expect("Kraken interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(kraken);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: kraken,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(9)),
            toughness: Some(PtValue::Fixed(9)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

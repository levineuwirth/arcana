//! Jace, Unraveler of Secrets — `{3}{U}{U}` Legendary Planeswalker — Jace,
//! starting loyalty 5.
//!
//! +1: Scry 1, then draw a card.
//! −2: Return target creature to its owner's hand.
//! −8: You get an emblem with "Whenever an opponent casts their first spell
//!   each turn, counter that spell." Modeled via `CreateEmblem` with an
//!   opponent-spell-cast trigger. The "first spell each turn" gating
//!   (event-history) and the counter of the just-cast spell are NOT
//!   expressible — there is no accessor for the triggering spell's object
//!   id from a `PendingTrigger` — so the trigger effect is GAP'd. The
//!   emblem shell + trigger condition are modeled.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::mana::ManaCost;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, Unraveler of Secrets");
    let jace = reg.interner_mut().intern("Jace");
    let _emblem_name = reg.interner_mut().intern("Jace, Unraveler of Secrets emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Scry 1, then draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_scry_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Return target creature to its owner's hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever an opponent casts \
                       their first spell each turn, counter that spell.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

fn plus_one_scry_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Scry { player: ctx.controller, count: 1 },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}

fn minus_two_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}

fn minus_eight_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Jace, Unraveler of Secrets emblem")
        .expect("emblem name interned at register");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: emblem_counter_spell,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_counter_spell(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "first spell each turn" is an event-history gate not expressible
    //      here, and there is no PendingTrigger accessor for the just-cast
    //      spell's object id to feed Effect::Counter { target }.
    Vec::new()
}

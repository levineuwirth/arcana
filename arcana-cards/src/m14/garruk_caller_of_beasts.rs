//! Garruk, Caller of Beasts — `{4}{G}{G}` Legendary Planeswalker — Garruk,
//! starting loyalty 4. Mono-green.
//!
//! +1: Reveal the top five cards of your library. Put all creature cards into
//!   your hand and the rest on the bottom in any order. GAP: a multi-card
//!   "take all creatures" reveal/sort is not expressible (DigTopN takes one).
//! −3: You may put a green creature card from your hand onto the battlefield
//!   (`Effect::PutFromHandOntoBattlefield` over green creature cards).
//! −7: emblem ("Whenever you cast a creature spell, you may search your library
//!   for a creature card, put it onto the battlefield, then shuffle."). Triggered
//!   emblem: SpellCast(creature, You) → `Effect::TutorToBattlefield` (search +
//!   put onto battlefield + shuffle).

use arcana_core::effects::{Effect, EmblemDefinition};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk, Caller of Beasts");
    let garruk = reg.interner_mut().intern("Garruk");
    let _emblem = reg.interner_mut().intern("Garruk, Caller of Beasts emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top five cards of your library. Put all \
                       creature cards revealed this way into your hand and the \
                       rest on the bottom of your library in any order.".into(),
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
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: You may put a green creature card from your hand \
                       onto the battlefield.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_put,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Whenever you cast a creature \
                       spell, you may search your library for a creature card, \
                       put it onto the battlefield, then shuffle.\"".into(),
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

fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top five, put ALL creatures to hand, rest on bottom in any
    //      order — multi-card take is not expressible by the single-pick DigTopN.
    Vec::new()
}

fn minus_three_put(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: ObjectFilter::creature().with_colors(ColorSet::green()),
        tapped: false,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Garruk, Caller of Beasts emblem").expect("emblem interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: emblem_tutor,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_tutor(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::CREATURE.into()),
        tapped: false,
    }]
}

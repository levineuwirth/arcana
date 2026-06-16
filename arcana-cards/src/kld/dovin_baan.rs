//! Dovin Baan — `{2}{W}{U}` Legendary Planeswalker — Dovin, starting loyalty 3.
//!
//! +1: Until your next turn, up to one target creature gets -3/-0 and its
//!     activated abilities can't be activated. (The -3/-0 pump until your next
//!     turn is implemented; the "its activated abilities can't be activated"
//!     rider GAPs — no ability-lock primitive in the demonstrated surface.)
//! −1: You gain 2 life and draw a card.
//! −7: You get an emblem with "Your opponents can't untap more than two
//!     permanents during their untap steps." (GAP — the untap-restriction
//!     static is not expressible via anthem/keyword/filtered builders. Emblem
//!     shell created with the interned name.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::effects::EmblemDefinition;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dovin Baan");
    let dovin = reg.interner_mut().intern("Dovin");
    let _emblem = reg.interner_mut().intern("Dovin Baan emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dovin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature gets \
                       -3/-0 and its activated abilities can't be activated."
                    .into(),
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
                effect: plus_one_weaken,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: You gain 2 life and draw a card.".into(),
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
                effect: minus_one_gain_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"Your opponents can't untap \
                       more than two permanents during their untap steps.\""
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

/// `+1: up to one target creature gets -3/-0 until your next turn.`
fn plus_one_weaken(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // -3/-0 until your next turn. The "its activated abilities can't be
    // activated" rider GAPs (no ability-lock primitive).
    vec![Effect::Pump {
        target: *id,
        power: -3,
        toughness: 0,
        duration: Duration::UntilYourNextTurn(ctx.controller),
        keywords: vec![],
    }]
}

/// `−1: You gain 2 life and draw a card.`
fn minus_one_gain_draw(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::GainLife {
            player: ctx.controller,
            amount: 2,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}

/// `−7: emblem — untap-restriction static GAP'd; shell created.`
fn minus_seven_emblem(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Dovin Baan emblem")
        .expect("emblem name interned");
    // GAP: "your opponents can't untap more than two permanents during their
    // untap steps" is an untap-count restriction not expressible via the
    // anthem/keyword/filtered static builders. Emblem shell created.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}

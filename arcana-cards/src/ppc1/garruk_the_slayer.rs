//! Garruk the Slayer — Legendary Planeswalker — Garruk (B/G, no mana cost).
//! 0: Put a 2/2 green Wolf creature token onto the battlefield.
//! +4: Target Wolf creature gets +1/+0 and gains deathtouch until end of turn.
//! −10: Destroy target creature. Put loyalty counters on Garruk the Slayer
//!   equal to that creature's toughness.
//! −25: Destroy all creatures Garruk the Slayer doesn't control.
//!
//! GAP: the −10 "put loyalty counters equal to that creature's toughness"
//!   rider is a dynamic (resolution-time) amount; only the destroy is emitted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Garruk the Slayer");
    let garruk = reg.interner_mut().intern("Garruk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(garruk);

    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(3),
        ..Default::default()
    };

    let wolf = reg.interner_mut().intern("Wolf");

    reg.register(
        CardDefinition::new(name, chars)
            // 0: Put a 2/2 green Wolf creature token onto the battlefield.
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Put a 2/2 green Wolf creature token onto the battlefield.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_make_wolf,
            })
            // +4: Target Wolf creature gets +1/+0 and gains deathtouch until end of turn.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+4: Target Wolf creature gets +1/+0 and gains deathtouch until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().with_subtype_sym(wolf),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_four_pump_wolf,
            })
            // −10: Destroy target creature. Put loyalty counters equal to its toughness.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Destroy target creature. Put loyalty counters on Garruk the Slayer equal to that creature's toughness.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_destroy,
            })
            // −25: Destroy all creatures Garruk the Slayer doesn't control.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-25: Destroy all creatures Garruk the Slayer doesn't control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 25)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_twentyfive_wrath,
            }),
    )
}

fn zero_make_wolf(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wolf = reg.interner().lookup("Wolf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: wolf,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn plus_four_pump_wolf(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Deathtouch],
    }]
}

fn minus_ten_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "put loyalty counters equal to that creature's toughness" is a
    // dynamic resolution-time amount; only the destroy is emitted.
    vec![Effect::DestroyPermanent { target: *id }]
}

fn minus_twentyfive_wrath(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        ctx.controller,
    )
    .into_iter()
    .map(|id| Effect::DestroyPermanent { target: id })
    .collect()
}

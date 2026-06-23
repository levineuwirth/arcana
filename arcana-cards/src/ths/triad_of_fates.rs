//! Triad of Fates — `{2}{W}{B}` 3/3 Legendary Human Wizard.
//!
//! Oracle:
//! * {1}, {T}: Put a fate counter on another target creature.
//! * {W}, {T}: Exile target creature that has a fate counter on it,
//!   then return it to the battlefield under its owner's control.
//! * {B}, {T}: Exile target creature that has a fate counter on it.
//!   Its controller draws two cards.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triad of Fates");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let _fate = reg.interner_mut().intern("fate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Hoisted: these intern via &mut reg, which can't be borrowed again
    // while reg.register(..) holds its own &mut self borrow.
    let fate_req_blink = fate_counter_target(reg);
    let fate_req_draw = fate_counter_target(reg);

    reg.register(
        CardDefinition::new(name, chars)
            // {1}, {T}: Put a fate counter on another target creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}: Put a fate counter on another target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_fate_counter,
            })
            // {W}, {T}: Exile a fate-countered creature, return it.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}: Exile target creature that has a fate \
                       counter on it, then return it to the battlefield \
                       under its owner's control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![fate_req_blink],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: blink_fate_creature,
            })
            // {B}, {T}: Exile a fate-countered creature; controller draws 2.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}, {T}: Exile target creature that has a fate \
                       counter on it. Its controller draws two cards."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![fate_req_draw],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_fate_draw,
            }),
    )
}

/// A target requirement matching "creature that has a fate counter on it".
fn fate_counter_target(reg: &mut CardRegistry) -> TargetRequirement {
    let fate = reg.interner_mut().intern("fate");
    TargetRequirement {
        filter: TargetFilter::Permanent(ObjectFilter {
            has_counter: Some(CounterKind::Named(fate)),
            ..ObjectFilter::creature()
        }),
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn put_fate_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let fate = reg
        .interner()
        .lookup("fate")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters {
        target: *id,
        kind: fate,
        count: 1,
    }]
}

fn blink_fate_creature(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Exile the creature. GAP: "then return it to the battlefield under
    // its owner's control" — exile re-ids the object, so the immediate
    // return cannot be sequenced on the old id with these primitives.
    vec![Effect::ExilePermanent { target: *id }]
}

fn exile_fate_draw(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let controller = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::ExilePermanent { target: *id },
        Effect::DrawCards {
            player: controller,
            count: 2,
        },
    ]
}

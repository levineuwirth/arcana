//! Olivia Voldaren — `{2}{B}{R}` 3/3 Legendary Creature — Vampire.
//! Flying.
//! {1}{R}: Olivia Voldaren deals 1 damage to another target creature. That
//!   creature becomes a Vampire in addition to its other types. Put a +1/+1
//!   counter on Olivia Voldaren.
//! {3}{B}{B}: Gain control of target Vampire for as long as you control Olivia
//!   Voldaren.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Olivia Voldaren");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let vampire_filter = ObjectFilter::creature().with_subtype_sym(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Olivia Voldaren deals 1 damage to another target creature. That creature becomes a Vampire in addition to its other types. Put a +1/+1 counter on Olivia Voldaren.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ping_and_grow,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}: Gain control of target Vampire for as long as you control Olivia Voldaren.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(vampire_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_control_vampire,
            }),
    )
}

fn ping_and_grow(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "that creature becomes a Vampire in addition to its other types" —
    // AddType adds card types, not creature subtypes; there is no AddSubtype
    // effect. The damage and the +1/+1 counter on Olivia are expressed.
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(*id),
            amount: 1,
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}

fn gain_control_vampire(
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
    // FIDELITY GAP: "for as long as you control Olivia Voldaren" — ChangeControl
    // is a permanent gain-control; there is no control-change keyed to the
    // source remaining on the battlefield. The gain-control itself is faithful.
    vec![Effect::ChangeControl {
        target: *id,
        new_controller: ctx.controller,
    }]
}

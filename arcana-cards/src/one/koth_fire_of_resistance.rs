//! Koth, Fire of Resistance — `{2}{R}{R}` Legendary Planeswalker — Koth,
//! starting loyalty 5. Mono-red.
//!
//! +2: Search your library for a basic Mountain card, reveal it, put it into
//!   your hand, then shuffle (`Effect::Search` library → hand, reveal).
//! −3: Koth deals damage to target creature equal to the number of Mountains
//!   you control (amount computed at resolution via `script::count_matching`).
//! −7: emblem ("Whenever a Mountain you control enters, this emblem deals 4
//!   damage to any target."). Triggered emblem: ZoneChange (Mountain you control
//!   → battlefield) → DealDamage 4 to the chosen any-target.

use arcana_core::effects::{Effect, EmblemDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koth, Fire of Resistance");
    let koth = reg.interner_mut().intern("Koth");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _emblem = reg.interner_mut().intern("Koth, Fire of Resistance emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(koth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Search your library for a basic Mountain card, \
                       reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_search,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Koth deals damage to target creature equal to the \
                       number of Mountains you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: You get an emblem with \"Whenever a Mountain you \
                       control enters, this emblem deals 4 damage to any \
                       target.\"".into(),
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

fn mountain_filter(reg: &CardRegistry) -> ObjectFilter {
    let mountain = reg.interner().lookup("Mountain").expect("Mountain interned");
    ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_subtype_sym(mountain)
        .with_supertypes(SupertypeSet(SupertypeSet::BASIC))
}

fn plus_two_search(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Search {
        player: ctx.controller,
        zone: Zone::Library(ctx.controller),
        filter: mountain_filter(reg),
        destination: Zone::Hand(ctx.controller),
        reveal: true,
    }]
}

fn minus_three_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    let mountains = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::LAND.into())
            .with_subtype_sym(reg.interner().lookup("Mountain").expect("Mountain interned"))
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount: mountains,
    }]
}

fn minus_seven_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Koth, Fire of Resistance emblem").expect("emblem interned");
    let mountain = reg.interner().lookup("Mountain").expect("Mountain interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .with_types(TypeLine::LAND.into())
                        .with_subtype_sym(mountain)
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: emblem_burn,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }],
        },
    }]
}

fn emblem_burn(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let dt = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id))) => DamageTarget::Object(*id),
        Some(TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p))) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: NULL_OBJECT_ID, target: dt, amount: 4 }]
}

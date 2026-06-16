//! Koth, Fire of Resistance — `{2}{R}{R}` Legendary Planeswalker — Koth, loyalty 5.
//!
//! +2: Search your library for a basic Mountain card, reveal it, put it into
//!   your hand, then shuffle.
//! −3: Koth deals damage to target creature equal to the number of Mountains
//!   you control.
//! −7: You get an emblem with "Whenever a Mountain you control enters, this
//!   emblem deals 4 damage to any target."
//!
//! # Scope
//! GAP: the −7 emblem grants a triggered ability ("Whenever a Mountain you
//!   control enters, deal 4 damage to any target") whose damage source is the
//!   emblem and whose target is chosen at trigger time — not expressible from
//!   the demonstrated emblem surface here. Ability shell declared, effect empty.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koth, Fire of Resistance");
    let koth = reg.interner_mut().intern("Koth");
    let _mountain = reg.interner_mut().intern("Mountain");
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
                text: "+2: Search your library for a basic Mountain card, reveal it, put it into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Koth deals damage to target creature equal to the number of Mountains you control.".into(),
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
                text: "-7: You get an emblem with \"Whenever a Mountain you control enters, this emblem deals 4 damage to any target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_gap,
            }),
    )
}

fn plus_two_tutor(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Mountain"),
        reveal: true,
    }]
}

fn minus_three_damage(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let amount = script::count_matching(
        state,
        &script::subtype_filter(reg, "Mountain").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::DealDamage {
        source: ctx.source,
        target: DamageTarget::Object(*id),
        amount,
    }]
}

fn minus_seven_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a "Mountain enters → deal 4 to any target" trigger.
    Vec::new()
}

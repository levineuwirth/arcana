//! Nicol Bolas, Planeswalker — `{4}{U}{B}{B}{R}` Legendary Planeswalker
//! — Bolas, starting loyalty 5. U/B/R.
//!
//! Oracle text:
//! * `+3`: Destroy target noncreature permanent.
//! * `−2`: Gain control of target creature.
//! * `−9`: Nicol Bolas deals 7 damage to target player or planeswalker.
//!   That player or that planeswalker's controller discards seven cards,
//!   then sacrifices seven permanents of their choice.
//!
//! # Scope
//!
//! * `+3` destroys a target noncreature permanent.
//! * `−2` gains control of a target creature
//!   (`Effect::ChangeControl` to the controller).
//! * `−9` is a multi-part bespoke ultimate (7 damage to a player-or-PW,
//!   then THAT controller discards 7 and sacrifices 7 of their choice);
//!   the per-target controller routing and chained sweeps are not
//!   expressible — GAP'd, shell declared with the correct `−9` cost.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nicol Bolas, Planeswalker");
    let bolas = reg.interner_mut().intern("Bolas");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bolas);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{B}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+3: Destroy target noncreature permanent.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::permanent()
                        .without_types(TypeLine::CREATURE.into())),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Gain control of target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: Nicol Bolas deals 7 damage to target player or \
                       planeswalker. That player or that planeswalker's \
                       controller discards seven cards, then sacrifices \
                       seven permanents of their choice.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine,
            }),
    )
}

/// `+3`: destroy target noncreature permanent.
fn plus_three(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−2`: gain control of target creature.
fn minus_two(_s: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::ChangeControl { target: *id, new_controller: ctx.controller }]
}

/// `−9`: damage + targeted-controller discard-7-then-sacrifice-7.
fn minus_nine(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: bespoke ultimate — 7 damage to a player-or-planeswalker, then
    // that player (or the PW's controller) discards 7 and sacrifices 7
    // of their choice. The per-target controller routing and chained
    // sweeps are not expressible from the demonstrated surface.
    Vec::new()
}

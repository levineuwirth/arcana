//! Tezzeret the Seeker — `{3}{U}{U}` Legendary Planeswalker — Tezzeret.
//! Printed starting loyalty 4 (CR 113.3c). Color blue.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Untap up to two target artifacts. — expressible.
//! * `−X`: Search your library for an artifact card with mana value X or
//!   less, put it onto the battlefield, then shuffle. — OMITTED. The
//!   `−X` dynamic loyalty cost is not expressible (`remove_self_counter`
//!   is a fixed `u32`); per the card-class rule we omit the ability
//!   entirely rather than mis-cost it. GAP.
//! * `−5`: Artifacts you control become artifact creatures with base
//!   power and toughness 5/5 until end of turn. — expressible as a
//!   resolution-time sweep (AddType CREATURE + SetBasePT 5/5 per artifact
//!   you control).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tezzeret the Seeker");
    let tezzeret = reg.interner_mut().intern("Tezzeret");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tezzeret);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };
    // GAP: the −X tutor ability is omitted (dynamic-X loyalty cost not
    // expressible).

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to two target artifacts.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: Artifacts you control become artifact creatures with \
                       base power and toughness 5/5 until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_animate,
            }),
    )
}

/// `+1`: untap up to two target artifacts.
fn plus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|c| match c {
            TargetChoice::Object(id) => Some(Effect::Untap { target: *id }),
            _ => None,
        })
        .collect()
}

/// `−5`: animate your artifacts as 5/5s until end of turn.
fn minus_five_animate(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, ctx.controller);
    let mut effects = Vec::new();
    for id in ids {
        effects.push(Effect::AddType {
            target: id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        });
        effects.push(Effect::SetBasePT {
            target: id,
            power: 5,
            toughness: 5,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}

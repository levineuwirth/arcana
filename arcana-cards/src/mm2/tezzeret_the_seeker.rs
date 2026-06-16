//! Tezzeret the Seeker — `{3}{U}{U}` Legendary Planeswalker — Tezzeret,
//! starting loyalty 4.
//!
//! Oracle text:
//! * `+1`: Untap up to two target artifacts.
//! * `−X`: Search your library for an artifact card with mana value X or
//!   less, put it onto the battlefield, then shuffle. (GAP, see below.)
//! * `−5`: Artifacts you control become artifact creatures with base power
//!   and toughness 5/5 until end of turn.
//!
//! # Rules references
//!
//! * CR 606 — loyalty abilities; the engine enforces sorcery-speed,
//!   stack-empty, controller-only, once-per-turn-per-PW activation and the
//!   0-loyalty state-based sacrifice (CR 704.5i).
//! * CR 606 dynamic-X — `−X` is modeled with `remove_loyalty_x: true`; the
//!   engine fans out one activation per X in `1..=loyalty` and threads the
//!   chosen X through `ctx.x_value`.
//!
//! # Scope
//!
//! * `+1` untaps up to two target artifacts (modeled fully).
//! * `−X` uses the dynamic-X loyalty cost shell, but the EFFECT
//!   (library-tutor-to-battlefield with a mana-value-X bound) is not
//!   expressible from the demonstrated `Effect` surface — `Effect::Reanimate`
//!   is graveyard/zone reanimation, not a filtered library search-to-play.
//!   GAP'd: the effect returns an empty vec while still paying the correct
//!   dynamic-X loyalty cost.
//! * `−5` animates each artifact you control: AddType(CREATURE) +
//!   SetBasePT(5,5) until end of turn, per controlled-artifact id.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
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
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−X: Search your library for an artifact card with mana \
                       value X or less, put it onto the battlefield, then \
                       shuffle."
                    .into(),
                cost: ActivationCost {
                    remove_loyalty_x: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_x_tutor,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: Artifacts you control become artifact creatures \
                       with base power and toughness 5/5 until end of turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
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
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::Untap { target: *id });
        }
    }
    effects
}

/// `−X`: search library for an artifact of mana value X or less, put onto the
/// battlefield, then shuffle.
fn minus_x_tutor(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: library tutor-to-battlefield with a mana-value-X bound is not
    // expressible from the demonstrated Effect surface. Effect::Reanimate is
    // graveyard/zone reanimation, not a filtered library search-to-play. The
    // dynamic-X loyalty cost is still paid via remove_loyalty_x.
    Vec::new()
}

/// `−5`: each artifact you control becomes an artifact creature with base P/T
/// 5/5 until end of turn.
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

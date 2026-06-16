//! Nissa, Ascended Animist — `{3}{G}{G}{G/P}{G/P}` Legendary Planeswalker —
//! Nissa, starting loyalty 7 — colors G.
//!
//! Compleated keyword is ignored here: the life-payment-reduces-starting-
//! loyalty cast modifier is not modeled (GAP). Loyalty is fixed at 7.
//!
//! Oracle text:
//! * `+1`: Create an X/X green Phyrexian Horror creature token, where X is
//!   Nissa's loyalty. — dynamic token P/T (token power/toughness must be a
//!   fixed value); GAP.
//! * `−1`: Destroy target artifact or enchantment. — `DestroyPermanent` with
//!   an artifact-or-enchantment target filter.
//! * `−7`: Until end of turn, creatures you control get +1/+1 for each Forest
//!   you control and gain trample. — dynamic `Anthem` keyed off Forests you
//!   control; the board-wide trample grant is GAP'd.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

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
    let name = reg.interner_mut().intern("Nissa, Ascended Animist");
    let nissa = reg.interner_mut().intern("Nissa");
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(
            ManaCost::parse("{3}{G}{G}{G/P}{G/P}").expect("valid cost"),
        ),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create an X/X green Phyrexian Horror creature token, \
                       where X is Nissa's loyalty.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Destroy target artifact or enchantment.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types_any(
                            (TypeLine::ARTIFACT | TypeLine::ENCHANTMENT).into(),
                        ),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Until end of turn, creatures you control get +1/+1 \
                       for each Forest you control and gain trample.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_anthem,
            }),
    )
}

/// `+1`: create an X/X Phyrexian Horror where X is loyalty.
fn plus_one_token(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a token whose power/toughness equals Nissa's loyalty is a dynamic
    // P/T; TokenDefinition P/T must be a fixed value.
    Vec::new()
}

/// `−1`: destroy target artifact or enchantment.
fn minus_one_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => *id,
        _ => return Vec::new(),
    };
    vec![Effect::DestroyPermanent { target }]
}

/// `−7`: +1/+1 per Forest to your creatures (board-wide trample GAP'd).
fn minus_seven_anthem(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "and gain trample" board-wide keyword grant is not co-emitted;
    // the dynamic +N/+N anthem (N = Forests you control) is faithful.
    let forest = match reg.interner().lookup("Forest") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_subtype_sym(forest)
            .controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: n as i32,
        toughness: n as i32,
        duration: Duration::EndOfTurn,
    }]
}

//! Aminatou, the Fateshifter — `{W}{U}{B}` Legendary Planeswalker — Aminatou,
//! starting loyalty 6.
//!
//! +1: Draw a card, then put a card from your hand on top of your library.
//!     (We model the draw; the "put a card from your hand on top" rider has no
//!      single-effect primitive — see GAP below.)
//! −1: Exile another target permanent you own, then return it to the
//!     battlefield under your control. GAP: blink (exile-and-immediate-return)
//!     of a chosen target is not a demonstrated single-effect primitive.
//! −6: Choose left or right; rotate control of nonland permanents. GAP:
//!     bespoke board-wide directional control swap.
//!
//! "Aminatou, the Fateshifter can be your commander." — reminder text only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aminatou, the Fateshifter");
    let aminatou = reg.interner_mut().intern("Aminatou");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aminatou);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card, then put a card from your hand on top of \
                       your library.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Exile another target permanent you own, then return it \
                       to the battlefield under your control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::You),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Choose left or right. Each player gains control of all \
                       nonland permanents other than Aminatou controlled by the \
                       next player in the chosen direction.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put a card from your hand on top of your library" — no
    // single-effect primitive to choose a hand card and place it on the
    // library top. We model the draw faithfully.
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn minus_one(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: blink — exile a chosen target permanent and immediately return it
    // to the battlefield under your control in one resolution is not a
    // demonstrated single-effect primitive (ExileUntilSourceLeaves and
    // ReturnFromExileToBattlefield don't compose into an instantaneous flicker
    // of a freshly-chosen target inside one resolver).
    Vec::new()
}

fn minus_six(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: bespoke directional board-wide control rotation.
    Vec::new()
}

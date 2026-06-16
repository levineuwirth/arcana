//! Inzerva, Master of Insights — `{1}{2/U}{2/R}` legendary planeswalker, starting loyalty 5.
//!
//! +2: Draw two cards, then discard a card.
//! −2: Look at the top two cards of each other player's library, put any
//!     number on the bottom and the rest on top, then Scry 2.
//! −4: You get an emblem (GAP).
//!
//! Scope: +2 draw-two-discard-one is fully expressed. For −2 only the
//! trailing "Scry 2" is expressed; the per-opponent library reordering
//! has no demonstrated Effect. The −4 emblem with a static + draw-damage
//! trigger is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inzerva, Master of Insights");
    let inzerva = reg.interner_mut().intern("Inzerva");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(inzerva);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{2/U}{2/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Draw two cards, then discard a card.".into(),
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
                effect: plus_two_draw_discard,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Look at the top two cards of each other player's \
                       library, then put any number of them on the bottom of \
                       that library and the rest on top in any order. Scry 2.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−4: You get an emblem with \"Your opponents play with \
                       their hands revealed\" and \"Whenever an opponent draws a \
                       card, this emblem deals 1 damage to them.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_emblem,
            }),
    )
}

/// `+2: Draw two cards, then discard a card.`
fn plus_two_draw_discard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ])]
}

/// `−2` — only the trailing Scry 2 is expressed.
fn minus_two_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at top two of each other player's library, reorder" not
    // expressible; only the trailing Scry 2 is emitted.
    vec![Effect::Scry { player: ctx.controller, count: 2 }]
}

/// `−4` — emblem.
fn minus_four_emblem(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: emblem with reveal-hands static + opponent-draw damage trigger.
    Vec::new()
}

//! B.O.B. (Bevy of Beebles) — `{3}{U}{U}` Legendary Planeswalker — B.O.B.,
//! starting loyalty 4 (best-effort: it enters making four Beebles and its
//! loyalty mirrors its Beeble count). Mono-blue. An Un-set planeswalker.
//!
//! Oracle:
//! As B.O.B. enters, create four 1/1 blue Beeble creature tokens.
//! The number of loyalty counters on B.O.B. is equal to the number of Beebles
//!   you control. (Create or sacrifice Beebles whenever B.O.B. gains or loses
//!   loyalty.)
//! +1: Up to X target Beebles can't be blocked this turn, where X is the
//!     number of cards in your hand.
//! −1: Draw a card.
//!
//! # Scope
//! * ETB "create four Beebles" + "loyalty = Beebles you control" — GAP: these
//!   are static/replacement abilities, not loyalty abilities, and the
//!   loyalty-mirrors-token-count linkage is not expressible. Printed loyalty
//!   is set to 4 as a best-effort.
//! * +1 — GAP: dynamic "up to X targets where X = cards in hand" is not
//!   expressible (TargetCount::X needs a supplied x_value).
//! * −1 — modeled: draw a card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("B.O.B. (Bevy of Beebles)");
    let sub = reg.interner_mut().intern("B.O.B.");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

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
                text: "+1: Up to X target Beebles can't be blocked this turn, \
                       where X is the number of cards in your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_draw,
            }),
    )
}

/// `+1` — GAP: dynamic "up to X targets" where X = cards in hand.
fn plus_one_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic-X target count (X = cards in hand).
    Vec::new()
}

/// `−1: Draw a card.`
fn minus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}

//! Nissa, Steward of Elements — `{X}{G}{U}` Legendary Planeswalker — Nissa.
//! Colors green/blue. Enters with X loyalty counters.
//!
//! GAP: printed starting loyalty is X (set by the cast's X value); a dynamic
//! starting loyalty isn't expressible (`loyalty: Some(N)` is a fixed value),
//! so it's recorded as Some(0) — the after-enter X-counter placement is a gap.
//!
//! +2: Scry 2.
//! 0: Look at the top card of your library. If it's a land card or a creature
//!    card with mana value less than or equal to the number of loyalty
//!    counters on Nissa, you may put that card onto the battlefield.
//!    GAP: conditional "look at top card, may put it onto the battlefield if
//!    it matches a loyalty-relative mana-value gate" not expressible.
//! −6: Untap up to two target lands you control. They become 5/5 Elemental
//!     creatures with flying and haste until end of turn. They're still lands.
//!     GAP: "lands become 5/5 Elemental creatures (still lands)" is a
//!     multi-layer becomes-creature effect not expressible from the surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Steward of Elements");
    let nissa = reg.interner_mut().intern("Nissa");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // GAP: starting loyalty is X — dynamic, not expressible as a fixed value.
        loyalty: Some(0),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Scry 2.".into(),
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
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Look at the top card of your library. If it's a land card or a creature card with mana value less than or equal to the number of loyalty counters on Nissa, you may put that card onto the battlefield.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}6: Untap up to two target lands you control. They become 5/5 Elemental creatures with flying and haste until end of turn. They're still lands.".into(),
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

fn plus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Scry { player: ctx.controller, count: 2 }]
}

fn zero(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: conditional look-at-top-card / may-put-onto-battlefield gated by a
    // loyalty-relative mana value is not expressible.
    Vec::new()
}

fn minus_six(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: untap target lands + "become 5/5 Elemental creatures (still lands)"
    // is a multi-layer becomes-creature effect not expressible here.
    Vec::new()
}

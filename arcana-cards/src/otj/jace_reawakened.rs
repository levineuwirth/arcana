//! Jace Reawakened — `{U}{U}` Legendary Planeswalker — Jace. Starting loyalty 5.
//!
//! Static (not a loyalty ability): "You can't cast Jace Reawakened during your
//! first, second, or third turns of the game." — a cast restriction, not
//! modeled here (no loyalty cost). The Plot keyword (Scryfall) is likewise a
//! cast-from-exile mechanic, not a printed keyword surface; GAP.
//!
//! +1: Draw a card, then discard a card.
//! +1: You may exile a nonland card with mana value 3 or less from your hand.
//!     If you do, it becomes plotted.
//!     GAP: "becomes plotted" (Plot) — cast-from-exile rider not expressible.
//! −6: Until end of turn, whenever you cast a spell, copy it. You may choose
//!     new targets for the copy.
//!     GAP: floating "whenever you cast a spell, copy it" window with
//!     new-targets choice not expressible from the demonstrated surface.

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
    let name = reg.interner_mut().intern("Jace Reawakened");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card, then discard a card.".into(),
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
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You may exile a nonland card with mana value 3 or less from your hand. If you do, it becomes plotted.".into(),
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
                effect: plus_one_plot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}6: Until end of turn, whenever you cast a spell, copy it. You may choose new targets for the copy.".into(),
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

fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn plus_one_plot(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile a nonland card mv<=3 from hand; it becomes plotted" — the
    // Plot cast-from-exile rider is not expressible.
    Vec::new()
}

fn minus_six(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: floating "whenever you cast a spell this turn, copy it (new targets
    // for the copy)" window not expressible from the demonstrated surface.
    Vec::new()
}

//! Chandra Ablaze — `{4}{R}{R}` legendary planeswalker, starting loyalty 5.
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Discard a card. If a red card is discarded this way, Chandra
//!   deals 4 damage to any target.
//! * `−2`: Each player discards their hand, then draws three cards.
//! * `−7`: Cast any number of red instant and/or sorcery cards from your
//!   graveyard without paying their mana costs.
//!
//! # Scope
//!
//! All three abilities are declared with their correct loyalty cost, but
//! their effects are GAP'd — none is expressible from the demonstrated
//! `Effect` surface:
//! * `+1` conditions damage on the COLOR of the card discarded this way;
//!   there is no demonstrated primitive to inspect a discarded card and
//!   branch on it.
//! * `−2` discards each player's ENTIRE hand (a dynamic, per-player count)
//!   then draws — the demonstrated `Discard`/`DrawCards` are fixed-count
//!   and single-player.
//! * `−7` casts cards from the graveyard for free (cast-from-graveyard /
//!   alternative-cost plumbing not in the demonstrated surface).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra Ablaze");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Discard a card. If a red card is discarded this \
                       way, Chandra deals 4 damage to any target.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Each player discards their hand, then draws three \
                       cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Cast any number of red instant and/or sorcery cards \
                       from your graveyard without paying their mana costs.".into(),
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
                effect: minus_seven,
            }),
    )
}

/// `+1`
fn plus_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: damage is conditioned on the COLOR of the card discarded this
    // way; no demonstrated primitive inspects a just-discarded card.
    Vec::new()
}

/// `−2`
fn minus_two(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: discards each player's ENTIRE hand (dynamic per-player count)
    // then draws — demonstrated Discard/DrawCards are fixed-count and
    // single-player.
    Vec::new()
}

/// `−7`
fn minus_seven(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: cast-from-graveyard without paying mana costs (alternative-cost
    // / cast-from-graveyard plumbing not in the demonstrated surface).
    Vec::new()
}

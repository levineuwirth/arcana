//! The GUI keystone path: drive a `Session`, project its view to a serializable
//! `ViewState`, and confirm it round-trips through JSON. This is exactly what a
//! web server (JSON over the wire) or a native egui app would consume.

use arcana_core::view::{view_state, ViewState};
use arcana_ai::search::RandomStatePolicy;
use arcana_ai::session::{Seat, Session, Turn};

#[test]
fn session_view_serializes_and_carries_the_perspective_hand() {
    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 7);
    let seats = vec![Seat::Human, Seat::Bot(Box::new(RandomStatePolicy::new(2)))];
    let mut session = Session::new(vec![deck.clone(), deck], &reg, seats, 1);

    // First human decision (the mulligan): build the view a frontend would render.
    let view = match session.advance() {
        Turn::AwaitingHuman { player, view, legal, .. } => {
            view_state(&view.state, &reg, player, &legal)
        }
        Turn::GameOver(r) => panic!("unexpected immediate game over: {r:?}"),
    };

    // Structure: two players, perspective P0, P0's life is 20, legal actions
    // have non-empty labels.
    assert_eq!(view.players.len(), 2);
    assert_eq!(view.perspective, 0);
    assert_eq!(view.players[0].life, 20);
    assert!(!view.legal.is_empty());
    assert!(view.legal.iter().all(|a| !a.label.is_empty()));

    // The perspective player's opening hand is populated WITH NAMES (for art
    // lookup); the opponent's hand is a count only (hidden info).
    assert_eq!(view.players[0].hand.len(), view.players[0].hand_count);
    assert!(view.players[0].hand_count >= 1);
    assert!(view.players[0].hand.iter().all(|c| !c.name.is_empty()),
        "own hand cards must resolve names");
    assert!(view.players[1].hand.is_empty(), "opponent hand is not revealed");
    assert!(view.players[1].hand_count >= 1);

    // Card-type / mana display fields are populated for the visible hand: every
    // card has a printed type line, `is_land` agrees with it, and a present mana
    // cost implies a matching nonzero mana value (lands have neither).
    for c in &view.players[0].hand {
        assert!(!c.type_line.is_empty(), "{} should have a type line", c.name);
        assert_eq!(c.is_land, c.type_line.contains("Land"),
            "is_land must agree with the type line for {}", c.name);
        if c.is_land {
            assert!(c.mana_cost.is_none() && c.mana_value == 0, "a land has no mana cost");
        }
        if let Some(cost) = &c.mana_cost {
            assert!(cost.starts_with('{'), "rendered cost is pip-formatted: {cost}");
        }
    }
    // Floating mana pools start empty, and with no permanents in play there is
    // no available mana yet (the gauge reads zero across the board).
    assert_eq!(view.players[0].mana_pool, 0);
    assert_eq!(view.players[1].mana_pool, 0);
    assert_eq!(view.players[0].available_mana.total, 0);
    assert_eq!(view.players[1].available_mana.total, 0);

    // Round-trips through JSON unchanged — the web transport contract.
    let json = serde_json::to_string(&view).expect("serialize");
    let back: ViewState = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, view);
}

//! Replay viewer + sample generator for [`GameRecord`]s — the observability
//! tool. `selfplay <out.json>` plays a random game and writes its record;
//! `replay <record.json>` re-derives and renders it turn-by-turn.

use anyhow::{Context, Result};
use arcana_core::engine::EngineYield;
use arcana_core::record::{replay_to, GameRecord};
use arcana_core::render::{render, render_oneline};
use arcana_ai::search::{play_match_recorded, RandomStatePolicy, StatePolicy};

/// Play one random self-play game and write its [`GameRecord`] to `path` (JSON).
pub fn selfplay(path: Option<&String>) -> Result<()> {
    let path = path.context("usage: arcana selfplay <out.json>")?;
    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 1);
    let mut a = RandomStatePolicy::new(1);
    let mut b = RandomStatePolicy::new(2);
    let mut slots: Vec<&mut dyn StatePolicy> = vec![&mut a, &mut b];
    let (result, record) =
        play_match_recorded(vec![deck.clone(), deck], &reg, 99, &mut slots, 4000);
    std::fs::write(path, serde_json::to_string_pretty(&record)?)
        .with_context(|| format!("writing {path}"))?;
    println!("Wrote {path}: {} actions, result {result:?}", record.actions.len());
    Ok(())
}

/// Load a [`GameRecord`] from `path` and replay it, printing a one-line trace
/// per turn and the full final board + result.
pub fn run(path: Option<&String>) -> Result<()> {
    let path = path.context("usage: arcana replay <record.json>")?;
    let json = std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?;
    let record: GameRecord = serde_json::from_str(&json).context("parsing GameRecord")?;
    let reg = arcana_cards::build_catalog();

    println!("Replay: {} players, seed {}, {} actions, result {:?}\n",
        record.decks.len(), record.seed, record.actions.len(), record.result);

    // One line per turn transition.
    let mut last_turn = 0;
    for n in 0..=record.actions.len() {
        let (state, _) = replay_to(&record, &reg, n);
        if state.turn.turn_number != last_turn {
            last_turn = state.turn.turn_number;
            println!("{}", render_oneline(&state));
        }
    }

    let (final_state, yld) = replay_to(&record, &reg, record.actions.len());
    println!("\n{}", render(&final_state, &reg));
    if let EngineYield::GameOver(r) = yld {
        println!("RESULT: {r:?}");
    }
    Ok(())
}

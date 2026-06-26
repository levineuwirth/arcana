//! Interactive human-play frontend — a thin CLI over the UI-agnostic
//! [`arcana_ai::session::Session`]. All game logic lives in the session; this
//! file only renders the view and reads a menu choice from stdin, so a future
//! GUI can reuse the exact same core.

use std::io::Write as _;

use anyhow::{Context, Result};
use arcana_core::render::{render, render_action, render_for};
use arcana_ai::search::{
    FlatMonteCarloPolicy, MaterialValue, PimcPolicy, RandomStatePolicy, ValueMcPolicy,
};
use arcana_ai::session::{Seat, Session, Turn};

/// Build a seat from a kind string. `human` is interactive; everything else is
/// a bot. `snappy`/`bot` (default) is short-rollout value-MC — sub-second moves;
/// `pimc` is stronger but slower; `mc` is perfect-info flat-MC; `random` is
/// trivial.
fn make_seat(kind: &str, seed: u64, decks: &[Vec<arcana_core::types::CardId>]) -> Result<Seat> {
    Ok(match kind {
        "human" => Seat::Human,
        "snappy" | "bot" => Seat::Bot(Box::new(
            ValueMcPolicy::with_budget(Box::new(MaterialValue), seed, 6, 25, 10))),
        "pimc" => Seat::Bot(Box::new(PimcPolicy::new(seed, decks.to_vec()))),
        "mc" => Seat::Bot(Box::new(FlatMonteCarloPolicy::new(seed))),
        "random" => Seat::Bot(Box::new(RandomStatePolicy::new(seed))),
        other => anyhow::bail!("unknown seat kind {other:?} (human|snappy|pimc|mc|random)"),
    })
}

fn arg_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

/// `play [--p0 KIND] [--p1 KIND] [--seed N]` — interactive game. Defaults:
/// P0 human, P1 snappy bot. KIND ∈ human|snappy|pimc|mc|random.
pub fn play(args: &[String]) -> Result<()> {
    let p0 = arg_value(args, "--p0").unwrap_or("human").to_string();
    let p1 = arg_value(args, "--p1").unwrap_or("snappy").to_string();
    let seed: u64 = match arg_value(args, "--seed") {
        Some(s) => s.parse().with_context(|| format!("--seed must be an integer, got {s:?}"))?,
        None => 1,
    };

    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 7);
    let decks = vec![deck.clone(), deck.clone()];
    let seats = vec![
        make_seat(&p0, seed * 2 + 1, &decks)?,
        make_seat(&p1, seed * 2 + 2, &decks)?,
    ];
    println!("Arcana — P0: {p0}  vs  P1: {p1}  (seed {seed})\n");

    let mut session = Session::new(decks, &reg, seats, seed);
    loop {
        match session.advance() {
            Turn::GameOver(result) => {
                println!("\n{}", render(session.state(), &reg));
                println!("\n=== GAME OVER: {result:?} ===");
                return Ok(());
            }
            Turn::AwaitingHuman { player, view, legal, context } => {
                println!("\n{}", render_for(&view.state, &reg, view.perspective));
                println!("── P{player} to decide: {context:?} ──");
                for (i, a) in legal.iter().enumerate() {
                    println!("  [{i}] {}", render_action(a, &view.state, &reg));
                }
                let Some(choice) = read_choice(legal.len())? else {
                    println!("(quit)");
                    return Ok(());
                };
                session.apply(legal[choice].clone());
            }
        }
    }
}

/// Prompt until a valid `0..len` index is entered. `Ok(None)` on EOF / `q`.
fn read_choice(len: usize) -> Result<Option<usize>> {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            return Ok(None); // EOF
        }
        let t = line.trim();
        if t == "q" || t == "quit" {
            return Ok(None);
        }
        match t.parse::<usize>() {
            Ok(i) if i < len => return Ok(Some(i)),
            _ => println!("  enter a number 0..{} (or q to quit)", len - 1),
        }
    }
}

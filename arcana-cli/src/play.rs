//! Interactive human-play frontend — a thin CLI over the UI-agnostic
//! [`arcana_ai::session::Session`]. All game logic lives in the session; this
//! file only renders the view and reads choices from stdin, so a future GUI can
//! reuse the exact same core (and the same combat-builder helpers).

use std::io::Write as _;

use anyhow::{Context, Result};
use arcana_core::actions::{Action, DecisionContext};
use arcana_core::combat::{
    attacker_options, blocker_options, match_attack, match_block, AttackerDeclaration,
    BlockerDeclaration, DefendingEntity,
};
use arcana_core::registry::CardRegistry;
use arcana_core::render::{render, render_action, render_for, render_object_brief};
use arcana_core::state::GameState;
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
                let Some(action) = prompt_action(&view.state, &reg, &legal, &context)? else {
                    println!("(quit)");
                    return Ok(());
                };
                session.apply(action);
            }
        }
    }
}

/// Present the decision and return the chosen action (`None` = quit). Combat
/// declarations use an incremental builder; everything else is a numbered menu.
fn prompt_action(
    state: &GameState, reg: &CardRegistry, legal: &[Action], context: &DecisionContext,
) -> Result<Option<Action>> {
    match context {
        DecisionContext::DeclareAttackers => choose_attackers(state, reg, legal),
        DecisionContext::DeclareBlockers => choose_blockers(state, reg, legal),
        _ => choose_from_menu(state, reg, legal),
    }
}

/// Generic numbered menu over the legal actions.
fn choose_from_menu(
    state: &GameState, reg: &CardRegistry, legal: &[Action],
) -> Result<Option<Action>> {
    for (i, a) in legal.iter().enumerate() {
        println!("  [{i}] {}", render_action(a, state, reg));
    }
    Ok(read_index(legal.len())?.map(|i| legal[i].clone()))
}

/// Incremental attacker declaration: pick which creatures attack (and, when an
/// attacker has more than one legal defender, which one), then match to a legal
/// declaration. Re-prompts on an illegal combination.
fn choose_attackers(
    state: &GameState, reg: &CardRegistry, legal: &[Action],
) -> Result<Option<Action>> {
    let opts = attacker_options(legal);
    if opts.is_empty() {
        return Ok(match_attack(legal, &[])); // nothing can attack
    }
    loop {
        println!("Your creatures that can attack:");
        for (i, (id, _)) in opts.iter().enumerate() {
            println!("  [{i}] {}", render_object_brief(state, reg, *id));
        }
        println!("Enter attacker numbers (space-separated; empty = attack with none):");
        let Some(line) = read_line()? else { return Ok(None) };
        let Some(picks) = parse_indices(&line, opts.len()) else {
            println!("  invalid — use numbers 0..{}", opts.len() - 1);
            continue;
        };
        let mut decls = Vec::new();
        let mut quit = false;
        for i in picks {
            let (id, defs) = &opts[i];
            let defending = if defs.len() == 1 {
                defs[0]
            } else {
                match choose_defender(state, reg, *id, defs)? {
                    Some(d) => d,
                    None => { quit = true; break; }
                }
            };
            decls.push(AttackerDeclaration { attacker: *id, defending });
        }
        if quit { return Ok(None); }
        match match_attack(legal, &decls) {
            Some(action) => return Ok(Some(action)),
            None => println!("  that attack isn't legal here — try again"),
        }
    }
}

/// When an attacker can attack more than one entity, pick which.
fn choose_defender(
    state: &GameState, reg: &CardRegistry, attacker: ObjectIdAlias, defs: &[DefendingEntity],
) -> Result<Option<DefendingEntity>> {
    println!("  {} can attack:", render_object_brief(state, reg, attacker));
    for (j, d) in defs.iter().enumerate() {
        println!("    [{j}] {}", defender_label(state, reg, d));
    }
    Ok(read_index(defs.len())?.map(|j| defs[j]))
}

fn defender_label(state: &GameState, reg: &CardRegistry, d: &DefendingEntity) -> String {
    match d {
        DefendingEntity::Player(p) => format!("Player P{p}"),
        DefendingEntity::Planeswalker(id) => format!("PW {}", render_object_brief(state, reg, *id)),
        DefendingEntity::Battle(id) => format!("Battle {}", render_object_brief(state, reg, *id)),
    }
}

/// Incremental blocker declaration: for each of your creatures that can block,
/// pick an attacker to block (or skip), then match to a legal declaration.
fn choose_blockers(
    state: &GameState, reg: &CardRegistry, legal: &[Action],
) -> Result<Option<Action>> {
    let opts = blocker_options(legal);
    if opts.is_empty() {
        return Ok(match_block(legal, &[])); // nothing can block
    }
    loop {
        let mut decls = Vec::new();
        let mut quit = false;
        for (blocker, attackers) in &opts {
            println!("{} can block:", render_object_brief(state, reg, *blocker));
            for (j, atk) in attackers.iter().enumerate() {
                println!("  [{j}] {}", render_object_brief(state, reg, *atk));
            }
            println!("  block which attacker? (number, or . to not block)");
            let Some(line) = read_line()? else { quit = true; break };
            let t = line.trim();
            if t == "." || t.is_empty() { continue; }
            match t.parse::<usize>() {
                Ok(j) if j < attackers.len() =>
                    decls.push(BlockerDeclaration { blocker: *blocker, blocking: attackers[j] }),
                _ => { println!("  invalid — skipping this blocker"); }
            }
        }
        if quit { return Ok(None); }
        match match_block(legal, &decls) {
            Some(action) => return Ok(Some(action)),
            None => println!("  that block isn't legal here — try again"),
        }
    }
}

// --- input helpers ----------------------------------------------------------

type ObjectIdAlias = arcana_core::objects::ObjectId;

/// Read one line; `Ok(None)` on EOF or `q`/`quit`.
fn read_line() -> Result<Option<String>> {
    print!("> ");
    std::io::stdout().flush().ok();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line)? == 0 {
        return Ok(None);
    }
    let t = line.trim();
    if t == "q" || t == "quit" { return Ok(None); }
    Ok(Some(line))
}

/// Prompt until a valid `0..len` index is entered. `Ok(None)` on EOF / quit.
fn read_index(len: usize) -> Result<Option<usize>> {
    loop {
        let Some(line) = read_line()? else { return Ok(None) };
        match line.trim().parse::<usize>() {
            Ok(i) if i < len => return Ok(Some(i)),
            _ => println!("  enter a number 0..{} (or q to quit)", len.saturating_sub(1)),
        }
    }
}

/// Parse space-separated indices, all required to be `< len` and de-duplicated.
/// Empty input → empty selection. `None` on any out-of-range / non-numeric token.
fn parse_indices(line: &str, len: usize) -> Option<Vec<usize>> {
    let mut out: Vec<usize> = Vec::new();
    for tok in line.split_whitespace() {
        let i: usize = tok.parse().ok()?;
        if i >= len { return None; }
        if !out.contains(&i) { out.push(i); }
    }
    Some(out)
}

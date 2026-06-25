//! Replay viewer + sample generator for [`GameRecord`]s — the observability
//! tool. `selfplay <out.json> [seed] [mc|pimc|ismcts|random]` plays a self-play
//! game and writes its record; `replay <record.json>` re-derives and renders it
//! turn-by-turn; `eval [flags]` runs a search-policy-vs-random win-rate sweep.

use anyhow::{Context, Result};
use arcana_core::engine::EngineYield;
use arcana_core::record::{replay_to, GameRecord};
use arcana_core::render::{render, render_oneline};
use arcana_core::types::CardId;
use arcana_ai::search::{
    play_match_recorded, win_rate, FlatMonteCarloPolicy, IsmctsPolicy, PimcPolicy,
    RandomStatePolicy, StatePolicy,
};

/// Build a fresh [`StatePolicy`] of the named kind seeded by `seed`. `mc` =
/// perfect-info flat Monte-Carlo, `pimc` = imperfect-info determinized flat-MC,
/// `ismcts` = SO-IS-MCTS tree; anything else is progress-biased random. The
/// imperfect-info policies need the per-player `decks` to determinize.
fn make_policy(kind: &str, seed: u64, decks: &[Vec<CardId>]) -> Box<dyn StatePolicy> {
    match kind {
        "mc" => Box::new(FlatMonteCarloPolicy::new(seed)),
        "pimc" => Box::new(PimcPolicy::new(seed, decks.to_vec())),
        "ismcts" => Box::new(IsmctsPolicy::new(seed, decks.to_vec())),
        _ => Box::new(RandomStatePolicy::new(seed)),
    }
}

/// Play one self-play game and write its [`GameRecord`] to `path` (JSON).
/// `args` is the full argv; `args[2]` = path, `args[3]` = seed (default 99),
/// `args[4]` = policy kind for BOTH seats (`mc` | `pimc` | `ismcts` | `random`,
/// default `random`).
pub fn selfplay(args: &[String]) -> Result<()> {
    let path = args.get(2)
        .context("usage: arcana selfplay <out.json> [seed] [mc|pimc|ismcts|random]")?;
    let seed: u64 = match args.get(3) {
        Some(s) => s.parse().with_context(|| format!("seed must be an integer, got {s:?}"))?,
        None => 99,
    };
    let kind = args.get(4).map(String::as_str).unwrap_or("random");

    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 1);
    let decks = vec![deck.clone(), deck.clone()];
    let mut a = make_policy(kind, seed * 2 + 1, &decks);
    let mut b = make_policy(kind, seed * 2 + 2, &decks);
    let mut slots: Vec<&mut dyn StatePolicy> = vec![a.as_mut(), b.as_mut()];
    let (result, record) =
        play_match_recorded(decks, &reg, seed, &mut slots, 4000);
    std::fs::write(path, serde_json::to_string_pretty(&record)?)
        .with_context(|| format!("writing {path}"))?;
    println!(
        "Wrote {path}: seed {seed}, {kind} vs {kind}, {} actions, result {result:?}",
        record.actions.len()
    );
    Ok(())
}

/// Run a search-policy-vs-random win-rate sweep. Flags (all optional):
/// `--policy mc|pimc|ismcts` (default `mc`; `pimc`/`ismcts` are imperfect-info),
/// `--rollouts N` (MC playouts/candidate or IS-MCTS iterations, default 15),
/// `--games K` (default 30), `--cap N` (rollout step cap, default 200),
/// `--candidates N` (max actions evaluated, default 12). Mirror deck; seats
/// alternate per game.
pub fn eval(args: &[String]) -> Result<()> {
    let flag = |name: &str, default: u32| -> Result<u32> {
        match args.iter().position(|a| a == name) {
            Some(i) => {
                let v = args.get(i + 1)
                    .with_context(|| format!("{name} needs a value"))?;
                v.parse().with_context(|| format!("{name} must be an integer, got {v:?}"))
            }
            None => Ok(default),
        }
    };
    let policy = args.iter().position(|a| a == "--policy")
        .and_then(|i| args.get(i + 1)).map(String::as_str).unwrap_or("mc").to_string();
    let rollouts = flag("--rollouts", 15)?;
    let games = flag("--games", 30)?;
    let cap = flag("--cap", 200)?;
    let candidates = flag("--candidates", 12)? as usize;

    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 7);
    let decks = vec![deck.clone(), deck.clone()];
    println!(
        "eval: {policy}(budget={rollouts} cap={cap} candidates={candidates}) \
         vs random over {games} games (mirror deck, alternating seats)…"
    );
    let mk: Box<dyn Fn(u64) -> Box<dyn StatePolicy>> = match policy.as_str() {
        "pimc" => { let d = decks.clone();
            Box::new(move |s| Box::new(PimcPolicy::with_budget(s, d.clone(), rollouts, cap, candidates))) }
        "ismcts" => { let d = decks.clone();
            Box::new(move |s| Box::new(IsmctsPolicy::with_budget(s, d.clone(), rollouts, cap, candidates))) }
        _ => Box::new(move |s| Box::new(FlatMonteCarloPolicy::with_budget(s, rollouts, cap, candidates))),
    };
    let (win, rnd, draws) = win_rate(
        &deck, &deck, &reg, games, 4000, mk.as_ref(),
        &|s| Box::new(RandomStatePolicy::new(s)),
    );
    let decided = (win + rnd).max(1);
    println!(
        "{policy}={win}  random={rnd}  draws={draws}  ({policy} win share of decided: {:.0}%)",
        100.0 * win as f32 / decided as f32
    );
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

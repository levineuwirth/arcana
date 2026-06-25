//! Replay viewer + sample generator for [`GameRecord`]s — the observability
//! tool. `selfplay <out.json> [seed] [mc|random]` plays a self-play game and
//! writes its record; `replay <record.json>` re-derives and renders it
//! turn-by-turn; `eval [flags]` runs a policy-vs-policy win-rate sweep.

use anyhow::{Context, Result};
use arcana_core::engine::EngineYield;
use arcana_core::record::{replay_to, GameRecord};
use arcana_core::render::{render, render_oneline};
use arcana_ai::search::{
    play_match_recorded, win_rate, FlatMonteCarloPolicy, RandomStatePolicy, StatePolicy,
};

/// Build a fresh [`StatePolicy`] of the named kind seeded by `seed`. `"mc"`
/// uses default-budget flat Monte-Carlo; anything else is progress-biased
/// random.
fn make_policy(kind: &str, seed: u64) -> Box<dyn StatePolicy> {
    match kind {
        "mc" => Box::new(FlatMonteCarloPolicy::new(seed)),
        _ => Box::new(RandomStatePolicy::new(seed)),
    }
}

/// Play one self-play game and write its [`GameRecord`] to `path` (JSON).
/// `args` is the full argv; `args[2]` = path, `args[3]` = seed (default 99),
/// `args[4]` = policy kind for BOTH seats (`mc` | `random`, default `random`).
pub fn selfplay(args: &[String]) -> Result<()> {
    let path = args.get(2).context("usage: arcana selfplay <out.json> [seed] [mc|random]")?;
    let seed: u64 = match args.get(3) {
        Some(s) => s.parse().with_context(|| format!("seed must be an integer, got {s:?}"))?,
        None => 99,
    };
    let kind = args.get(4).map(String::as_str).unwrap_or("random");

    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 1);
    let mut a = make_policy(kind, seed * 2 + 1);
    let mut b = make_policy(kind, seed * 2 + 2);
    let mut slots: Vec<&mut dyn StatePolicy> = vec![a.as_mut(), b.as_mut()];
    let (result, record) =
        play_match_recorded(vec![deck.clone(), deck], &reg, seed, &mut slots, 4000);
    std::fs::write(path, serde_json::to_string_pretty(&record)?)
        .with_context(|| format!("writing {path}"))?;
    println!(
        "Wrote {path}: seed {seed}, {kind} vs {kind}, {} actions, result {result:?}",
        record.actions.len()
    );
    Ok(())
}

/// Run a flat-Monte-Carlo-vs-random win-rate sweep. Flags (all optional):
/// `--rollouts N` (MC playouts/candidate, default 15), `--games K` (default
/// 30), `--cap N` (rollout step cap, default 200), `--candidates N` (max
/// actions evaluated, default 12). Mirror deck; seats alternate per game.
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
    let rollouts = flag("--rollouts", 15)?;
    let games = flag("--games", 30)?;
    let cap = flag("--cap", 200)?;
    let candidates = flag("--candidates", 12)? as usize;

    let reg = arcana_cards::build_catalog();
    let deck = arcana_cards::sample_deck(&reg, 7);
    println!(
        "eval: flat-MC(rollouts={rollouts} cap={cap} candidates={candidates}) \
         vs random over {games} games (mirror deck, alternating seats)…"
    );
    let (mc, rnd, draws) = win_rate(
        &deck, &deck, &reg, games, 4000,
        &|s| Box::new(FlatMonteCarloPolicy::with_budget(s, rollouts, cap, candidates)),
        &|s| Box::new(RandomStatePolicy::new(s)),
    );
    let decided = (mc + rnd).max(1);
    println!(
        "flat-MC={mc}  random={rnd}  draws={draws}  (MC win share of decided: {:.0}%)",
        100.0 * mc as f32 / decided as f32
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

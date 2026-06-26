//! Arcana developer CLI.

mod debugger;
mod replay;
mod bench;
mod play;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selfplay") => replay::selfplay(&args),
        Some("replay") => replay::run(args.get(2)),
        Some("eval") => replay::eval(&args),
        Some("arena") => replay::arena(&args),
        Some("play") => play::play(&args),
        _ => {
            println!("Arcana CLI — developer tools");
            println!("  play [--p0 KIND] [--p1 KIND] [--seed N]            interactive game (KIND: human|snappy|pimc|mc|random)");
            println!("  selfplay <out.json> [seed] [mc|pimc|ismcts|random]  play a game, write a GameRecord");
            println!("  replay   <record.json>                              re-derive + render a recorded game");
            println!("  eval [--policy mc|pimc|ismcts --rollouts N --games K --cap N --candidates N]  search vs random win-rate");
            println!("  arena [--games K --rollouts N --cap N --candidates N]  round-robin tournament (random/flatMC/pimc)");
            Ok(())
        }
    }
}

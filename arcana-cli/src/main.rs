//! Arcana developer CLI.

mod debugger;
mod replay;
mod bench;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selfplay") => replay::selfplay(&args),
        Some("replay") => replay::run(args.get(2)),
        Some("eval") => replay::eval(&args),
        _ => {
            println!("Arcana CLI — developer tools");
            println!("  selfplay <out.json> [seed] [mc|random]  play a game, write a GameRecord");
            println!("  replay   <record.json>                  re-derive + render a recorded game");
            println!("  eval [--rollouts N --games K --cap N --candidates N]  flat-MC vs random win-rate");
            Ok(())
        }
    }
}

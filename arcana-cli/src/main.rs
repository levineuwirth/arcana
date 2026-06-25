//! Arcana developer CLI.

mod debugger;
mod replay;
mod bench;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("selfplay") => replay::selfplay(args.get(2)),
        Some("replay") => replay::run(args.get(2)),
        _ => {
            println!("Arcana CLI — developer tools");
            println!("  selfplay <out.json>     play a random game, write a GameRecord");
            println!("  replay   <record.json>  re-derive + render a recorded game");
            Ok(())
        }
    }
}

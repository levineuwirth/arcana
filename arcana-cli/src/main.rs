//! Arcana developer CLI.

mod debugger;
mod replay;
mod bench;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Arcana CLI — developer tools (stub)");
    println!("Subcommands: debugger, replay, bench");
    Ok(())
}

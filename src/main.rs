pub mod gameplay;
pub mod networking;
pub mod graphics;

use std::env;
use std::path::Path;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run /path/to/image.(png|jpg)")
    } else {
        graphics::main_loop::run(Path::new(&args[1]))?;
    }

    Ok(())
}
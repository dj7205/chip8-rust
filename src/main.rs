use std::env;

use crate::emulator::Emulator;

mod cpu;
mod emulator;


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut emulator = Emulator::initialize();
    emulator.load_game(&args[1]);
    //emulator.start_emulation();

}

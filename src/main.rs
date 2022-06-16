use std::{env, fs};

use crate::{cpu::Cpu, memory::Memory};

mod cpu;
mod instructions;
mod memory;
mod util;
fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: gameboy <file>");
        return;
    }

    let filename = &args[1];

    let _contents = fs::read(filename).expect("Error reading the given filename");

    let mut cpu = Cpu::new();

    let mut memory = Memory::new();

    let instruction = cpu.parse(&memory);

    cpu.execute(instruction, &mut memory);
}

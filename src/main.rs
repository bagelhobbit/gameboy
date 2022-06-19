use crate::{cpu::Cpu, instructions::Instruction, memory::Memory};
use std::{env, fs};

mod cpu;
mod instructions;
mod io_registers;
mod memory;
mod tile_info;
mod util;
mod vram;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: gameboy <file>");
        return;
    }

    let filename = &args[1];

    let contents = fs::read(filename).expect("Error reading the given filename");

    let mut cpu = Cpu::new();

    let mut memory = Memory::new();

    memory.rom[0..contents.len()].clone_from_slice(&contents[..]);

    loop {
        let instruction = cpu.parse(&memory);

        if instruction != Instruction::Nop {
            println!("{:?} - 0x{:0>2X?} - {:?}", cpu.program_counter, memory.read(cpu.program_counter), instruction);
            // println!("SP: {:0>4X?}", cpu.stack_pointer);
            // println!("HL: {:0>4X?}", cpu.hl());
        }

        if instruction == Instruction::Invalid {
            break;
        }

        cpu.execute(instruction, &mut memory);
    }
}

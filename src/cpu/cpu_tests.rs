#![cfg(test)]

use crate::{
    cpu::Cpu,
    instructions::{ConditionalFlag, Instruction},
    memory::Memory,
};

#[test]
fn test_di() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.interrupts_enabled = true;
    cpu.execute(Instruction::DisableInterrupts, &mut memory);

    assert_eq!(cpu.interrupts_enabled, false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_ei() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.interrupts_enabled = false;
    cpu.execute(Instruction::EnableInterrupts, &mut memory);

    assert_eq!(cpu.interrupts_enabled, true);
    assert_eq!(cpu.program_counter, 1);
}

// Jump instruction tests

#[test]
fn test_jp() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0x00;
    memory.rom[2] = 0x11;
    cpu.execute(Instruction::Jump, &mut memory);

    assert_eq!(cpu.program_counter, 0x1100);
}

#[test]
fn test_jp_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;

    cpu.execute(Instruction::JumpHL, &mut memory);

    assert_eq!(cpu.program_counter, 0x1100);
}

#[test]
fn test_jp_flags() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    let flag = ConditionalFlag::NZ;
    memory.rom[1] = 0x00;
    memory.rom[2] = 0x11;

    memory.rom[0x1101] = 0x15;
    memory.rom[0x1102] = 0x14;

    cpu.execute(Instruction::JumpConditional { flag }, &mut memory);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.is_zero = true;
    cpu.execute(Instruction::JumpConditional { flag }, &mut memory);
    assert_eq!(cpu.program_counter, 0x1103);
}

#[test]
fn test_jr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    let pc = cpu.program_counter;
    memory.rom[1] = 25;
    // -20 as a u8, should be equal to 236
    memory.rom[1 + 2 + 25] = 0b1110_1100;

    cpu.execute(Instruction::JumpRelative, &mut memory);
    assert_eq!(cpu.program_counter, pc + 2 + 25);

    let pc = cpu.program_counter;
    cpu.execute(Instruction::JumpRelative, &mut memory);
    assert_eq!(cpu.program_counter, pc + 2 - 20);
}

#[test]
fn test_jr_flags() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    let pc = cpu.program_counter;
    let flag = ConditionalFlag::NZ;
    memory.rom[1] = 25;
    memory.rom[1 + 25] = 25;

    cpu.execute(Instruction::JumpRelativeConditional { flag }, &mut memory);
    assert_eq!(cpu.program_counter, pc + 2 + 25);

    let pc = cpu.program_counter;
    cpu.is_zero = true;
    cpu.execute(Instruction::JumpRelativeConditional { flag }, &mut memory);
    assert_eq!(cpu.program_counter, pc + 2);
}

#[test]
fn test_call() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0x00;
    memory.rom[2] = 0x11;
    memory.write(0xFFFC, 0xFF);
    cpu.execute(Instruction::Call, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100);
}

#[test]
fn test_call_conditional() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);
    let flag = ConditionalFlag::NC;

    memory.rom[1] = 0x00;
    memory.rom[2] = 0x11;
    memory.write(0xFFFC, 0xFF);
    cpu.execute(Instruction::CallConditional { flag }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.is_carry = true;
    cpu.execute(Instruction::CallConditional { flag }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100 + 3);
}

#[test]
fn test_ret() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write(0xFFFC, 0x00);
    memory.write(0xFFFD, 0x11);
    cpu.stack_pointer -= 2;
    cpu.execute(Instruction::Return, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFE);
    assert_eq!(cpu.program_counter, 0x1100);
}

#[test]
fn test_ret_conditional() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    let flag = ConditionalFlag::Z;

    memory.write(0xFFFA, 0x00);
    memory.write(0xFFFB, 0x11);
    cpu.stack_pointer -= 4;
    cpu.is_zero = true;
    cpu.execute(Instruction::ReturnConditional { flag }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.is_zero = false;
    cpu.execute(Instruction::ReturnConditional { flag }, &mut memory);
    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100 + 1);
}

#[test]
fn test_reti() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.interrupts_enabled = false;
    memory.write(0xFFFC, 0x00);
    memory.write(0xFFFD, 0x11);
    cpu.stack_pointer -= 2;
    cpu.execute(Instruction::ReturnAndEnableInterrupts, &mut memory);

    assert_eq!(cpu.interrupts_enabled, true);
    assert_eq!(cpu.stack_pointer, 0xFFFE);
    assert_eq!(cpu.program_counter, 0x1100);
}

#[test]
fn test_rst_0() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write(0xFFFC, 0xFF);
    cpu.execute(Instruction::Reset0 { location: 0xE }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(memory.read(cpu.stack_pointer), 1);
    assert_eq!(cpu.program_counter, 0x20);
}

#[test]
fn test_rst_8() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write(0xFFFC, 0xFF);
    cpu.execute(Instruction::Reset8 { location: 0xD }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(memory.read(cpu.stack_pointer), 1);
    assert_eq!(cpu.program_counter, 0x18);
}

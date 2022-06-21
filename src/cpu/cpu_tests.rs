#![cfg(test)]

use crate::{
    cpu::Cpu,
    instructions::{ConditionalFlag, DoubleRegister, Instruction, Register},
    memory::Memory,
};

// 8-bit load instruction tests

#[test]
fn test_load_reg() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.b = 0x11;
    cpu.d = 0xE5;

    cpu.execute(
        Instruction::LoadReg {
            load_from: 2,
            load_into: Register::B,
        },
        &mut memory,
    );
    cpu.execute(
        Instruction::LoadReg {
            load_from: 8,
            load_into: Register::L,
        },
        &mut memory,
    );

    assert_eq!(cpu.b, cpu.d);
    assert_eq!(cpu.l, cpu.b);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_reg_8() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0xE5;

    cpu.execute(
        Instruction::LoadReg8 {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.b, 0xE5);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_reg_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xE5;

    cpu.execute(
        Instruction::LoadRegHL {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.b, 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_hl_reg() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    cpu.b = 0xE5;

    cpu.execute(
        Instruction::LoadHLReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(memory.rom[0x1100], 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_hl_8() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[1] = 0xE5;

    cpu.execute(Instruction::LoadHL8, &mut memory);

    assert_eq!(memory.rom[0x1100], 0xE5);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_a_bc() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.b = 0x11;
    cpu.c = 0x00;
    memory.rom[0x1100] = 0xE5;

    cpu.execute(Instruction::LoadABC, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_a_de() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0x11;
    cpu.e = 0x00;
    memory.rom[0x1100] = 0xE5;

    cpu.execute(Instruction::LoadADE, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_a_address() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0x11;
    memory.rom[2] = 0x00;
    memory.rom[0x1100] = 0xE5;

    cpu.execute(Instruction::LoadAAddress, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.program_counter, 3);
}

#[test]
fn test_load_bc_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xE5;
    cpu.b = 0x11;
    cpu.c = 0x00;

    cpu.execute(Instruction::LoadBCA, &mut memory);

    assert_eq!(memory.rom[cpu.bc() as usize], cpu.a);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_de_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xE5;
    cpu.d = 0x11;
    cpu.e = 0x00;

    cpu.execute(Instruction::LoadDEA, &mut memory);

    assert_eq!(memory.rom[cpu.de() as usize], cpu.a);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_address_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0xE5;
    memory.rom[1] = 0x11;
    memory.rom[2] = 0x00;

    cpu.execute(Instruction::LoadAddressA, &mut memory);

    assert_eq!(memory.rom[0x1100], cpu.a);
    assert_eq!(cpu.program_counter, 3);
}

#[test]
fn test_load_a_offset() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0x01;
    memory.io_registers[1] = 0xE5;

    cpu.execute(Instruction::LoadAOffset, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_offset_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0xE5;
    memory.rom[1] = 0x01;

    cpu.execute(Instruction::LoadOffsetA, &mut memory);

    assert_eq!(memory.io_registers[1], 0xE5);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_a_offset_c() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0x01;
    memory.io_registers[1] = 0xE5;

    cpu.execute(Instruction::LoadAOffsetC, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_offset_c_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xE5;
    cpu.c = 0x01;

    cpu.execute(Instruction::LoadOffsetCA, &mut memory);

    assert_eq!(memory.io_registers[1], 0xE5);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_inc_hl_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xE5;
    cpu.h = 0x00;
    cpu.l = 0xFF;

    cpu.execute(Instruction::LoadIncrementHLA, &mut memory);

    assert_eq!(memory.rom[0x00FF], 0xE5);
    assert_eq!(cpu.h, 1);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_inc_a_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.h = 0x00;
    cpu.l = 0xFF;
    memory.rom[0x00FF] = 0xE5;

    cpu.execute(Instruction::LoadIncrementAHL, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.h, 1);
    assert_eq!(cpu.l, 0);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_dec_hl_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xE5;
    cpu.h = 0x01;
    cpu.l = 0x00;

    cpu.execute(Instruction::LoadDecrementHLA, &mut memory);

    assert_eq!(memory.rom[0x0100], 0xE5);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0xFF);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_load_dec_a_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x01;
    cpu.l = 0x00;
    memory.rom[0x0100] = 0xE5;

    cpu.execute(Instruction::LoadDecrementAHL, &mut memory);

    assert_eq!(cpu.a, 0xE5);
    assert_eq!(cpu.h, 0);
    assert_eq!(cpu.l, 0xFF);
    assert_eq!(cpu.program_counter, 1);
}

// 16-bit load instruction tests

#[test]
fn test_load_reg_16() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 0x55;
    memory.rom[2] = 0xEE;

    cpu.execute(
        Instruction::LoadReg16 {
            register: DoubleRegister::DE,
        },
        &mut memory,
    );

    assert_eq!(cpu.d, 0xEE);
    assert_eq!(cpu.c, 0x55);
    assert_eq!(cpu.program_counter, 3);
}

#[test]
fn test_load_address_sp() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.stack_pointer = 0xABCD;
    memory.rom[1] = 0x1F;
    memory.rom[2] = 0x00;

    cpu.execute(Instruction::LoadAddressSP, &mut memory);

    assert_eq!(memory.read(0x1F00), 0xAB);
    assert_eq!(memory.read(0x1F01), 0xCD);
    assert_eq!(cpu.program_counter, 3);
}

#[test]
fn test_load_sp_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0xEE;
    cpu.l = 0x55;

    cpu.execute(Instruction::LoadSPHL, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xEE55);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_push_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.stack_pointer = 0x1102;
    cpu.a = 0xE5;
    cpu.set_zero(true);
    cpu.set_carry(true);

    cpu.execute(
        Instruction::PushReg {
            register: DoubleRegister::AF,
        },
        &mut memory,
    );

    assert_eq!(memory.read(0x1100), 0xE5);
    assert_eq!(memory.read(0x1101), 0b1001_0000);
    assert_eq!(cpu.stack_pointer, 0x1100);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_pop_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.stack_pointer = 0x1100;
    memory.rom[0x1100] = 0xE5;
    memory.rom[0x1101] = 0x5E;

    cpu.execute(
        Instruction::PopReg {
            register: DoubleRegister::HL,
        },
        &mut memory,
    );

    assert_eq!(cpu.h, 0xE5);
    assert_eq!(cpu.l, 0x5E);
    assert_eq!(cpu.stack_pointer, 0x1102);
    assert_eq!(cpu.program_counter, 1);
}

// 8-bit Arithmetic/Logic instruction tests

#[test]
fn test_overflow_addition() {
    let mut cpu = Cpu::new();

    cpu.a = 5;
    cpu.overflow_addition(10);

    assert_eq!(cpu.a, 15);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_addition_zero() {
    let mut cpu = Cpu::new();

    cpu.a = 0;
    cpu.overflow_addition(0);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_addition_half_carry() {
    let mut cpu = Cpu::new();

    cpu.a = 0b0000_1111;
    cpu.overflow_addition(1);

    assert_eq!(cpu.a, 0b0001_0000);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_addition_carry() {
    let mut cpu = Cpu::new();

    cpu.a = u8::MAX;
    cpu.overflow_addition(10);

    assert_eq!(cpu.a, 9);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
}

#[test]
fn test_overflow_addition_zero_carry() {
    let mut cpu = Cpu::new();

    cpu.a = u8::MAX;
    cpu.overflow_addition(1);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
}

#[test]
fn test_overflow_subtraction() {
    let mut cpu = Cpu::new();

    cpu.a = 15;
    let result = cpu.overflow_subtraction(10);

    assert_eq!(result, 5);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_subtraction_zero() {
    let mut cpu = Cpu::new();

    cpu.a = 0;
    let result = cpu.overflow_subtraction(0);

    assert_eq!(result, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_subtraction_half_carry() {
    let mut cpu = Cpu::new();

    cpu.a = 0b0001_0000;
    let result = cpu.overflow_subtraction(1);

    assert_eq!(result, 0b0000_1111);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
}

#[test]
fn test_overflow_subtraction_carry() {
    let mut cpu = Cpu::new();

    cpu.a = 0;
    let result = cpu.overflow_subtraction(10);

    assert_eq!(result, 246);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
}

#[test]
fn test_and_a_reg() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b0011_0011;
    cpu.b = 0b0010_1111;

    cpu.execute(
        Instruction::AndAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0b0010_0011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_and_a_reg_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    cpu.b = 0xFF;

    cpu.execute(
        Instruction::AndAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_and_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0b0011_0011;
    memory.rom[1] = 0b0010_1111;

    cpu.execute(Instruction::AndA, &mut memory);

    assert_eq!(cpu.a, 0b0010_0011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_and_a_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    memory.rom[1] = 0xFF;

    cpu.execute(Instruction::AndA, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_and_a_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b0011_0011;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b0010_1111;

    cpu.execute(Instruction::AndAHL, &mut memory);

    assert_eq!(cpu.a, 0b0010_0011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_and_a_hl_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xFF;

    cpu.execute(Instruction::AndAHL, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_xor_a_reg() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;
    cpu.b = 0b0101_0101;

    cpu.execute(
        Instruction::XorAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_xor_a_reg_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xF0;
    cpu.b = 0xF0;

    cpu.execute(
        Instruction::XorAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_xor_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0b1010_1010;
    memory.rom[1] = 0b0101_0101;

    cpu.execute(Instruction::XorA, &mut memory);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_xor_a_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0xF0;
    memory.rom[1] = 0xF0;

    cpu.execute(Instruction::XorA, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_xor_a_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b0101_0101;

    cpu.execute(Instruction::XorAHL, &mut memory);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_xor_a_hl_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0xF0;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xF0;

    cpu.execute(Instruction::XorAHL, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_or_a_reg() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;
    cpu.b = 0b1111_1111;

    cpu.execute(
        Instruction::OrAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_or_a_reg_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    cpu.b = 0;

    cpu.execute(
        Instruction::OrAReg {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_or_a() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    cpu.a = 0b1010_1010;
    memory.rom[1] = 0b1111_1111;

    cpu.execute(Instruction::OrA, &mut memory);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_or_a_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    memory.rom[1] = 0;

    cpu.execute(Instruction::OrA, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_or_a_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1111_1111;

    cpu.execute(Instruction::OrAHL, &mut memory);

    assert_eq!(cpu.a, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_or_a_hl_zero() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0;
    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0;

    cpu.execute(Instruction::OrAHL, &mut memory);

    assert_eq!(cpu.a, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_r() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.e = 0x0F;

    cpu.execute(
        Instruction::IncrementReg {
            register: Register::E,
        },
        &mut memory,
    );

    assert_eq!(cpu.e, 0x10);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_r_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0xFF;

    cpu.execute(
        Instruction::IncrementReg {
            register: Register::C,
        },
        &mut memory,
    );

    assert_eq!(cpu.c, 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0x0F;

    cpu.execute(Instruction::IncrementHL, &mut memory);

    assert_eq!(memory.rom[0x1100], 0x10);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_hl_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xFF;

    cpu.execute(Instruction::IncrementHL, &mut memory);

    assert_eq!(memory.rom[0x1100], 0);
    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_r() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.e = 0x10;

    cpu.execute(
        Instruction::DecrementReg {
            register: Register::E,
        },
        &mut memory,
    );

    assert_eq!(cpu.e, 0x0f);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_r_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0;

    cpu.execute(
        Instruction::DecrementReg {
            register: Register::C,
        },
        &mut memory,
    );

    assert_eq!(cpu.c, 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0x10;

    cpu.execute(Instruction::DecrementHL, &mut memory);

    assert_eq!(memory.rom[0x1100], 0x0F);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_hl_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0x0;

    cpu.execute(Instruction::DecrementHL, &mut memory);

    assert_eq!(memory.rom[0x1100], 0xFF);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), true);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
// Using a BCD add example of 19 + 28 = 47
// 0b0001_1001 + 0b010_1000 = 0b0100_0001 => 0b0100_0111
fn test_daa_addition() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b0100_0001;
    cpu.set_subtraction(false);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::DecimalAdjustA, &mut memory);

    assert_eq!(cpu.a, 0b0100_0111);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
// Using a BCD subtract example of 47 - 28 = 19
// 0b0100_0111 + 0b1101_1000 = 0b0001_1111 => 0b0001_1001
fn test_daa_subtraction() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b0001_1111;
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::DecimalAdjustA, &mut memory);

    assert_eq!(cpu.a, 0b0001_1001);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_cpl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;

    cpu.execute(Instruction::Complement, &mut memory);

    assert_eq!(cpu.a, 0b0101_0101);
    assert_eq!(cpu.program_counter, 1);
}

// 16-bit arithmetic/logic instruction tests

#[test]
fn test_add_hl_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x0F;
    cpu.l = 0xFF;
    cpu.d = 0x00;
    cpu.e = 0x02;

    cpu.execute(
        Instruction::AddHLReg {
            register: DoubleRegister::DE,
        },
        &mut memory,
    );

    assert_eq!(cpu.hl(), 0x1001);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_add_hl_rr_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0xFF;
    cpu.l = 0xFF;
    cpu.b = 0x00;
    cpu.c = 0x02;

    cpu.execute(
        Instruction::AddHLReg {
            register: DoubleRegister::BC,
        },
        &mut memory,
    );

    assert_eq!(cpu.hl(), 1);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0x00;
    cpu.e = 0xFF;

    cpu.execute(
        Instruction::IncrementReg16 {
            register: DoubleRegister::DE,
        },
        &mut memory,
    );

    assert_eq!(cpu.de(), 0x0100);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_inc_rr_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.b = 0xFF;
    cpu.c = 0xFF;

    cpu.execute(
        Instruction::IncrementReg16 {
            register: DoubleRegister::BC,
        },
        &mut memory,
    );

    assert_eq!(cpu.bc(), 0);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0x01;
    cpu.e = 0x00;

    cpu.execute(
        Instruction::DecrementReg16 {
            register: DoubleRegister::DE,
        },
        &mut memory,
    );

    assert_eq!(cpu.de(), 0x00FF);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_dec_rr_overflow() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.b = 0x00;
    cpu.c = 0x00;

    cpu.execute(
        Instruction::DecrementReg16 {
            register: DoubleRegister::BC,
        },
        &mut memory,
    );

    assert_eq!(cpu.bc(), 0xFFFF);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_add_sp_offset_postive() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 2;
    cpu.execute(Instruction::AddSPOffset, &mut memory);

    assert_eq!(cpu.stack_pointer, 0);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_add_sp_offset_negative() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = (-10 as i8) as u8;
    cpu.execute(Instruction::AddSPOffset, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFE - 10);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_hl_sp_offset_postive() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = 2;
    cpu.execute(Instruction::LoadHLSPOffset, &mut memory);

    assert_eq!(cpu.hl(), 0);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_load_hl_sp_offset_negative() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();
    memory.write(0xFF50, 1);

    memory.rom[1] = (-10 as i8) as u8;
    cpu.execute(Instruction::LoadHLSPOffset, &mut memory);

    assert_eq!(cpu.hl(), 0xFFFE - 10);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

// Rotate and Shift instruction tests

#[test]
fn test_rlca() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateALeft, &mut memory);

    assert_eq!(cpu.a, 0b0100_1011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_rla() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateALeftThroughCarry, &mut memory);

    assert_eq!(cpu.a, 0b0100_1010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_rrca() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateARight, &mut memory);

    assert_eq!(cpu.a, 0b1101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_rra() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateARightThroughCarry, &mut memory);

    assert_eq!(cpu.a, 0b0101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_rlc() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0b1010_0101;
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(
        Instruction::RotateLeft {
            register: Register::C,
        },
        &mut memory,
    );

    assert_eq!(cpu.c, 0b0100_1011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rlc_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0101;
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateHLLeft, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0100_1011);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0b1010_0101;
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(
        Instruction::RotateLeftThroughCarry {
            register: Register::D,
        },
        &mut memory,
    );

    assert_eq!(cpu.d, 0b0100_1010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rl_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0101;
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateHLLeftThroughCarry, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0100_1010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rrc() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.e = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(
        Instruction::RotateRight {
            register: Register::E,
        },
        &mut memory,
    );

    assert_eq!(cpu.e, 0b1101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rrc_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateHLRight, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b1101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rr() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(
        Instruction::RotateRightThroughCarry {
            register: Register::H,
        },
        &mut memory,
    );

    assert_eq!(cpu.h, 0b0101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_rr_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0101;
    cpu.set_zero(true);
    cpu.set_subtraction(true);
    cpu.set_half_carry(true);

    cpu.execute(Instruction::RotateHLRightThroughCarry, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0101_0010);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_sla() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.a = 0b1010_1010;
    cpu.execute(
        Instruction::ShiftLeftArithmetic {
            register: Register::A,
        },
        &mut memory,
    );

    assert_eq!(cpu.a, 0b0101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_sla_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_1010;
    cpu.execute(Instruction::ShiftHLLeftArithmetic, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_swap() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.b = 0xF0;
    cpu.execute(
        Instruction::Swap {
            register: Register::B,
        },
        &mut memory,
    );

    assert_eq!(cpu.b, 0x0F);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_swap_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xF0;
    cpu.execute(Instruction::SwapHL, &mut memory);

    assert_eq!(memory.rom[0x1100], 0x0F);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), false);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_sra() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.e = 0b1010_1001;
    cpu.execute(
        Instruction::ShiftRightArithmetic {
            register: Register::E,
        },
        &mut memory,
    );

    assert_eq!(cpu.e, 0b1101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_sra_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_1001;
    cpu.execute(Instruction::ShiftHLRightArithmetic, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b1101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_srl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.e = 0b1010_1001;
    cpu.execute(
        Instruction::ShiftRightLogical {
            register: Register::E,
        },
        &mut memory,
    );

    assert_eq!(cpu.e, 0b0101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_srl_hl() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_1001;
    cpu.execute(Instruction::ShiftHLRightLogical, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0101_0100);
    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), false);
    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

// Single-bit Operation instruction tests

#[test]
fn test_test_bit_true() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0b1010_0000;
    cpu.execute(
        Instruction::TestBit {
            bit: 5,
            register: Register::D,
        },
        &mut memory,
    );

    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_test_bit_false() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.d = 0b1010_0000;
    cpu.execute(
        Instruction::TestBit {
            bit: 0,
            register: Register::D,
        },
        &mut memory,
    );

    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_test_hl_bit_true() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0000;
    cpu.execute(Instruction::TestHLBit { bit: 5 }, &mut memory);

    assert_eq!(cpu.is_zero(), false);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_test_hl_bit_false() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0b1010_0000;
    cpu.execute(Instruction::TestHLBit { bit: 0 }, &mut memory);

    assert_eq!(cpu.is_zero(), true);
    assert_eq!(cpu.is_subtraction(), false);
    assert_eq!(cpu.is_half_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_set_bit() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0;
    cpu.execute(
        Instruction::SetBit {
            bit: 5,
            register: Register::C,
        },
        &mut memory,
    );

    assert_eq!(cpu.c, 0b0010_0000);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_set_hl_bit() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0;
    cpu.execute(Instruction::SetHLBit { bit: 5 }, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b0010_0000);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_reset_bit() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.c = 0xFF;
    cpu.execute(
        Instruction::ResetBit {
            bit: 5,
            register: Register::C,
        },
        &mut memory,
    );

    assert_eq!(cpu.c, 0b1101_1111);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_reset_hl_bit() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.h = 0x11;
    cpu.l = 0x00;
    memory.rom[0x1100] = 0xFF;
    cpu.execute(Instruction::ResetHLBit { bit: 5 }, &mut memory);

    assert_eq!(memory.rom[0x1100], 0b1101_1111);
    assert_eq!(cpu.program_counter, 2);
}

// CPU Control instruction tests

#[test]
fn test_ccf() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.set_carry(true);
    cpu.execute(Instruction::FlipCarryFlag, &mut memory);

    assert_eq!(cpu.is_carry(), false);
    cpu.execute(Instruction::FlipCarryFlag, &mut memory);

    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 2);
}

#[test]
fn test_scf() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.execute(Instruction::SetCarryFlag, &mut memory);

    assert_eq!(cpu.is_carry(), true);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_nop() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    cpu.execute(Instruction::Nop, &mut memory);

    assert_eq!(cpu.program_counter, 1);
}

//halt

//stop

#[test]
fn test_di() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.interrupt_enable_register = true;
    cpu.execute(Instruction::DisableInterrupts, &mut memory);

    assert_eq!(memory.interrupt_enable_register, false);
    assert_eq!(cpu.program_counter, 1);
}

#[test]
fn test_ei() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.interrupt_enable_register = false;
    cpu.execute(Instruction::EnableInterrupts, &mut memory);

    assert_eq!(memory.interrupt_enable_register, true);
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
    memory.rom[1] = 0x11;
    memory.rom[2] = 0x00;

    memory.rom[0x1101] = 0x15;
    memory.rom[0x1102] = 0x14;

    cpu.execute(Instruction::JumpConditional { flag }, &mut memory);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.set_zero(true);
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
    cpu.set_zero(true);
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
    assert_eq!(memory.read(cpu.stack_pointer), 0);
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
    assert_eq!(memory.read(cpu.stack_pointer), 0);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.set_carry(true);
    cpu.execute(Instruction::CallConditional { flag }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100 + 3);
}

#[test]
fn test_ret() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write16(0xFFFC, 0x1100);
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

    memory.write16(0xFFFA, 0x1100);
    cpu.stack_pointer -= 4;
    cpu.set_zero(true);
    cpu.execute(Instruction::ReturnConditional { flag }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100);

    cpu.set_zero(false);
    cpu.execute(Instruction::ReturnConditional { flag }, &mut memory);
    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(cpu.program_counter, 0x1100 + 1);
}

#[test]
fn test_reti() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write(0xFFFF, 0);
    memory.write16(0xFFFC, 0x1100);
    cpu.stack_pointer -= 2;
    cpu.execute(Instruction::ReturnAndEnableInterrupts, &mut memory);

    assert_eq!(memory.read(0xFFFF), 1);
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
    assert_eq!(memory.read(cpu.stack_pointer), 0);
    assert_eq!(cpu.program_counter, 20);
}

#[test]
fn test_rst_8() {
    let mut cpu = Cpu::new();
    let mut memory = Memory::new();

    memory.write(0xFFFC, 0xFF);
    cpu.execute(Instruction::Reset8 { location: 0xD }, &mut memory);

    assert_eq!(cpu.stack_pointer, 0xFFFC);
    assert_eq!(memory.read(cpu.stack_pointer), 0);
    assert_eq!(cpu.program_counter, 18);
}

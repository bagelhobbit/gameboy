use crate::{
    alu_result::AluResult,
    instructions::{ConditionalFlag, DoubleRegister, Instruction, Register},
    util::*,
};

mod cpu_test_cpu_instrs;
mod cpu_tests;

pub trait CpuBus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, val: u8);
}

#[derive(Debug, Default)]
pub struct Cpu {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a: u8,
    pub is_zero: bool,
    pub is_subtraction: bool,
    pub is_half_carry: bool,
    pub is_carry: bool,
    pub stack_pointer: u16,
    pub program_counter: u16,
    halt: bool,
    interrupts_enabled: bool,
    pub debug: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            a: 0,
            is_zero: false,
            is_subtraction: false,
            is_half_carry: false,
            is_carry: false,
            stack_pointer: 0xFFFE,
            program_counter: 0,
            halt: false,
            interrupts_enabled: true,
            debug: false,
        }
    }

    pub fn bc(&self) -> u16 {
        combine_bytes(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        combine_bytes(self.d, self.e)
    }

    pub fn hl(&self) -> u16 {
        combine_bytes(self.h, self.l)
    }

    fn increment_hl(&mut self) {
        if self.hl() == u16::MAX {
            self.h = 0;
            self.l = 0;
        } else {
            let result = self.hl() + 1;
            self.h = get_upper_byte(result);
            self.l = get_lower_byte(result);
        }
    }

    fn decrement_hl(&mut self) {
        if self.hl() == 0 {
            self.h = u8::MAX;
            self.l = u8::MAX;
        } else {
            let result = self.hl() - 1;
            self.h = get_upper_byte(result);
            self.l = get_lower_byte(result);
        }
    }

    pub fn flags_to_byte(&self) -> u8 {
        let binary_string = format!(
            "{}{}{}{}0000",
            (self.is_zero) as u8,
            (self.is_subtraction) as u8,
            (self.is_half_carry) as u8,
            (self.is_carry) as u8
        );
        u8::from_str_radix(&binary_string, 2).unwrap()
    }

    fn byte_to_flags(&mut self, byte: u8) {
        self.is_zero = (byte & 0b1000_0000) != 0;
        self.is_subtraction = (byte & 0b0100_0000) != 0;
        self.is_half_carry = (byte & 0b0010_0000) != 0;
        self.is_carry = (byte & 0b001_0000) != 0;
    }

    pub fn parse(&mut self, cpu_bus: &mut impl CpuBus) -> Instruction {
        let instruction = cpu_bus.read(self.program_counter);

        match (get_upper_bits(instruction), get_lower_bits(instruction)) {
            (0x0, 0x0) => Instruction::Nop,
            (0x0, 0x2) => Instruction::LoadBCA,
            (0x0, 0x7) => Instruction::RotateALeft,
            (0x0, 0x8) => Instruction::LoadAddressSP,
            (0x0, 0xA) => Instruction::LoadABC,
            (0x0, 0xF) => Instruction::RotateARight,
            (0x1, 0x0) => Instruction::Stop,
            (0x1, 0x2) => Instruction::LoadDEA,
            (0x1, 0x7) => Instruction::RotateALeftThroughCarry,
            (0x1, 0x8) => Instruction::JumpRelative,
            (0x1, 0xA) => Instruction::LoadADE,
            (0x1, 0xF) => Instruction::RotateARightThroughCarry,
            (0x2, 0x0) => Instruction::JumpRelativeConditional {
                flag: ConditionalFlag::NZ,
            },
            (0x2, 0x2) => Instruction::LoadIncrementHLA,
            (0x2, 0x7) => Instruction::DecimalAdjustA,
            (0x2, 0x8) => Instruction::JumpRelativeConditional {
                flag: ConditionalFlag::Z,
            },
            (0x2, 0xA) => Instruction::LoadIncrementAHL,
            (0x2, 0xF) => Instruction::Complement,
            (0x3, 0x0) => Instruction::JumpRelativeConditional {
                flag: ConditionalFlag::NC,
            },
            (0x3, 0x2) => Instruction::LoadDecrementHLA,
            (0x3, 0x4) => Instruction::IncrementHL,
            (0x3, 0x5) => Instruction::DecrementHL,
            (0x3, 0x6) => Instruction::LoadHL8,
            (0x3, 0x7) => Instruction::SetCarryFlag,
            (0x3, 0x8) => Instruction::JumpRelativeConditional {
                flag: ConditionalFlag::C,
            },
            (0x3, 0xA) => Instruction::LoadDecrementAHL,
            (0x3, 0xF) => Instruction::FlipCarryFlag,
            (0x4, 0x6) => Instruction::LoadRegHL {
                register: Register::B,
            },
            (0x4, 0xE) => Instruction::LoadRegHL {
                register: Register::C,
            },
            (0x4, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, // Duplicate entry to pad LD R, (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::LoadReg {
                        dst: Register::B,
                        src: registers[reg as usize],
                    }
                } else {
                    Instruction::LoadReg {
                        dst: Register::C,
                        src: registers[reg as usize % 8],
                    }
                }
            }
            (0x5, 0x6) => Instruction::LoadRegHL {
                register: Register::D,
            },
            (0x5, 0xE) => Instruction::LoadRegHL {
                register: Register::E,
            },
            (0x5, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, // Duplicate entry to pad LD R, (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::LoadReg {
                        dst: Register::D,
                        src: registers[reg as usize],
                    }
                } else {
                    Instruction::LoadReg {
                        dst: Register::E,
                        src: registers[reg as usize % 8],
                    }
                }
            }
            (0x6, 0x6) => Instruction::LoadRegHL {
                register: Register::H,
            },
            (0x6, 0xE) => Instruction::LoadRegHL {
                register: Register::L,
            },
            (0x6, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, // Duplicate entry to pad LD R, (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::LoadReg {
                        dst: Register::H,
                        src: registers[reg as usize],
                    }
                } else {
                    Instruction::LoadReg {
                        dst: Register::L,
                        src: registers[reg as usize % 8],
                    }
                }
            }
            (0x7, 0x6) => Instruction::Halt,
            (0x7, 0xE) => Instruction::LoadRegHL {
                register: Register::A,
            },
            (0x7, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, // Duplicate entry to pad HALT (0x76)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::LoadHLReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::LoadReg {
                        dst: Register::A,
                        src: registers[reg as usize % 8],
                    }
                }
            }
            (0x8, 0x6) => Instruction::AddAHL,
            (0x8, 0xE) => Instruction::AddCarryAHL,
            (0x8, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad ADD A,(HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::AddAReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::AddCarryAReg {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x9, 0x6) => Instruction::SubtractAHL,
            (0x9, 0xE) => Instruction::SubtractAHLCarry,
            (0x9, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad SUB (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::SubtractAReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::SubtractARegCarry {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xA, 0x6) => Instruction::AndAHL,
            (0xA, 0xE) => Instruction::XorAHL,
            (0xA, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad AND/XOR (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::AndAReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::XorAReg {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xB, 0x6) => Instruction::OrAHL,
            (0xB, 0xE) => Instruction::CompareAHL,
            (0xB, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad OR/CP (HL)
                    Register::A,
                ];
                if reg <= 7 {
                    Instruction::OrAReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::CompareAReg {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xC, 0x0) => Instruction::ReturnConditional {
                flag: ConditionalFlag::NZ,
            },
            (0xC, 0x2) => Instruction::JumpConditional {
                flag: ConditionalFlag::NZ,
            },
            (0xC, 0x3) => Instruction::Jump,
            (0xC, 0x4) => Instruction::CallConditional {
                flag: ConditionalFlag::NZ,
            },
            (0xC, 0x6) => Instruction::AddA,
            (0xC, 0x8) => Instruction::ReturnConditional {
                flag: ConditionalFlag::Z,
            },
            (0xC, 0x9) => Instruction::Return,
            (0xC, 0xA) => Instruction::JumpConditional {
                flag: ConditionalFlag::Z,
            },
            (0xC, 0xB) => self.parse_prefix(cpu_bus),
            (0xC, 0xC) => Instruction::CallConditional {
                flag: ConditionalFlag::Z,
            },
            (0xC, 0xD) => Instruction::Call,
            (0xC, 0xE) => Instruction::AddCarryA,
            (0xD, 0x0) => Instruction::ReturnConditional {
                flag: ConditionalFlag::NC,
            },
            (0xD, 0x2) => Instruction::JumpConditional {
                flag: ConditionalFlag::NC,
            },
            (0xD, 0x3) => Instruction::CallConditional {
                flag: ConditionalFlag::NZ,
            },
            (0xD, 0x4) => Instruction::CallConditional {
                flag: ConditionalFlag::NC,
            },
            (0xD, 0x6) => Instruction::SubtractA,
            (0xD, 0x8) => Instruction::ReturnConditional {
                flag: ConditionalFlag::C,
            },
            (0xD, 0x9) => Instruction::ReturnAndEnableInterrupts,
            (0xD, 0xA) => Instruction::JumpConditional {
                flag: ConditionalFlag::C,
            },
            (0xD, 0xC) => Instruction::CallConditional {
                flag: ConditionalFlag::C,
            },
            (0xD, 0xE) => Instruction::SubtractACarry,
            (0xE, 0x0) => Instruction::LoadOffsetA,
            (0xE, 0x2) => Instruction::LoadOffsetCA,
            (0xE, 0x6) => Instruction::AndA,
            (0xE, 0x8) => Instruction::AddSPOffset,
            (0xE, 0x9) => Instruction::JumpHL,
            (0xE, 0xA) => Instruction::LoadAddressA,
            (0xE, 0xE) => Instruction::XorA,
            (0xF, 0x0) => Instruction::LoadAOffset,
            (0xF, 0x2) => Instruction::LoadAOffsetC,
            (0xF, 0x3) => Instruction::DisableInterrupts,
            (0xF, 0x6) => Instruction::OrA,
            (0xF, 0x8) => Instruction::LoadHLSPOffset,
            (0xF, 0x9) => Instruction::LoadSPHL,
            (0xF, 0xA) => Instruction::LoadAAddress,
            (0xF, 0xB) => Instruction::EnableInterrupts,
            (0xF, 0xE) => Instruction::CompareA,
            (reg, 0x1) => {
                if reg < 4 {
                    let registers = [
                        DoubleRegister::BC,
                        DoubleRegister::DE,
                        DoubleRegister::HL,
                        DoubleRegister::SP,
                    ];
                    Instruction::LoadReg16 {
                        register: registers[reg as usize],
                    }
                } else {
                    let registers = [
                        DoubleRegister::BC,
                        DoubleRegister::DE,
                        DoubleRegister::HL,
                        DoubleRegister::AF,
                    ];
                    let index = reg as usize - 0xC;
                    Instruction::PopReg {
                        register: registers[index],
                    }
                }
            }
            (reg, 0x3) => {
                let registers = [
                    DoubleRegister::BC,
                    DoubleRegister::DE,
                    DoubleRegister::HL,
                    DoubleRegister::SP,
                ];
                Instruction::IncrementReg16 {
                    register: registers[reg as usize],
                }
            }
            (reg, 0x4) => {
                let registers = [Register::B, Register::D, Register::H];
                Instruction::IncrementReg {
                    register: registers[reg as usize],
                }
            }
            (reg, 0x5) => {
                if reg < 4 {
                    let registers = [Register::B, Register::D, Register::H];
                    Instruction::DecrementReg {
                        register: registers[reg as usize],
                    }
                } else {
                    let registers = [
                        DoubleRegister::BC,
                        DoubleRegister::DE,
                        DoubleRegister::HL,
                        DoubleRegister::AF,
                    ];
                    let index = reg as usize - 0xC;
                    Instruction::PushReg {
                        register: registers[index],
                    }
                }
            }
            (location, 0x6) => {
                // handle LD (HL),d8 elsewhere
                let registers = [Register::B, Register::D, Register::H];
                Instruction::LoadReg8 {
                    register: registers[location as usize],
                }
            }
            (location, 0x7) => Instruction::Reset0 { location },
            (reg, 0x9) => {
                let registers = [
                    DoubleRegister::BC,
                    DoubleRegister::DE,
                    DoubleRegister::HL,
                    DoubleRegister::SP,
                ];
                Instruction::AddHLReg {
                    register: registers[reg as usize],
                }
            }
            (reg, 0xB) => {
                let registers = [
                    DoubleRegister::BC,
                    DoubleRegister::DE,
                    DoubleRegister::HL,
                    DoubleRegister::SP,
                ];
                Instruction::DecrementReg16 {
                    register: registers[reg as usize],
                }
            }
            (reg, 0xC) => {
                let registers = [Register::C, Register::E, Register::L, Register::A];
                Instruction::IncrementReg {
                    register: registers[reg as usize],
                }
            }
            (reg, 0xD) => {
                let registers = [Register::C, Register::E, Register::L, Register::A];
                Instruction::DecrementReg {
                    register: registers[reg as usize],
                }
            }
            (reg, 0xE) => {
                let registers = [Register::C, Register::E, Register::L, Register::A];
                Instruction::LoadReg8 {
                    register: registers[reg as usize % 4],
                }
            }
            (location, 0xF) => Instruction::Reset8 { location },
            _ => Instruction::Invalid,
        }
    }

    fn parse_prefix(&mut self, cpu_bus: &mut impl CpuBus) -> Instruction {
        let instruction = cpu_bus.read(self.program_counter + 1);

        let registers = [
            Register::B,
            Register::C,
            Register::D,
            Register::E,
            Register::H,
            Register::L,
            Register::A, //Duplicate entry to pad (HL) instructions
            Register::A,
        ];

        match (get_upper_bits(instruction), get_lower_bits(instruction)) {
            (0x0, 0x6) => Instruction::RotateHLLeft,
            (0x0, 0xE) => Instruction::RotateHLRight,
            (0x0, reg) => {
                if reg < 8 {
                    Instruction::RotateLeft {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::RotateRight {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x1, 0x6) => Instruction::RotateHLLeftThroughCarry,
            (0x1, 0xE) => Instruction::RotateHLRightThroughCarry,
            (0x1, reg) => {
                if reg < 8 {
                    Instruction::RotateLeftThroughCarry {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::RotateRightThroughCarry {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x2, 0x6) => Instruction::ShiftHLLeftArithmetic,
            (0x2, 0xE) => Instruction::ShiftHLRightArithmetic,
            (0x2, reg) => {
                if reg < 8 {
                    Instruction::ShiftLeftArithmetic {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ShiftRightArithmetic {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x3, 0x6) => Instruction::SwapHL,
            (0x3, 0xE) => Instruction::ShiftHLRightLogical,
            (0x3, reg) => {
                if reg < 8 {
                    Instruction::Swap {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ShiftRightLogical {
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x4, 0x6) => Instruction::TestHLBit { bit: 0 },
            (0x4, 0xE) => Instruction::TestHLBit { bit: 1 },
            (0x4, reg) => {
                if reg < 8 {
                    Instruction::TestBit {
                        bit: 0,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::TestBit {
                        bit: 1,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x5, 0x6) => Instruction::TestHLBit { bit: 2 },
            (0x5, 0xE) => Instruction::TestHLBit { bit: 3 },
            (0x5, reg) => {
                if reg < 8 {
                    Instruction::TestBit {
                        bit: 2,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::TestBit {
                        bit: 3,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x6, 0x6) => Instruction::TestHLBit { bit: 4 },
            (0x6, 0xE) => Instruction::TestHLBit { bit: 5 },
            (0x6, reg) => {
                if reg < 8 {
                    Instruction::TestBit {
                        bit: 4,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::TestBit {
                        bit: 5,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x7, 0x6) => Instruction::TestHLBit { bit: 6 },
            (0x7, 0xE) => Instruction::TestHLBit { bit: 7 },
            (0x7, reg) => {
                if reg < 8 {
                    Instruction::TestBit {
                        bit: 6,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::TestBit {
                        bit: 7,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x8, 0x6) => Instruction::ResetHLBit { bit: 0 },
            (0x8, 0xE) => Instruction::ResetHLBit { bit: 1 },
            (0x8, reg) => {
                if reg < 8 {
                    Instruction::ResetBit {
                        bit: 0,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ResetBit {
                        bit: 1,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0x9, 0x6) => Instruction::ResetHLBit { bit: 2 },
            (0x9, 0xE) => Instruction::ResetHLBit { bit: 3 },
            (0x9, reg) => {
                if reg < 8 {
                    Instruction::ResetBit {
                        bit: 2,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ResetBit {
                        bit: 3,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xA, 0x6) => Instruction::ResetHLBit { bit: 4 },
            (0xA, 0xE) => Instruction::ResetHLBit { bit: 5 },
            (0xA, reg) => {
                if reg < 8 {
                    Instruction::ResetBit {
                        bit: 4,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ResetBit {
                        bit: 5,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xB, 0x6) => Instruction::ResetHLBit { bit: 6 },
            (0xB, 0xE) => Instruction::ResetHLBit { bit: 7 },
            (0xB, reg) => {
                if reg < 8 {
                    Instruction::ResetBit {
                        bit: 6,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::ResetBit {
                        bit: 7,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xC, 0x6) => Instruction::SetHLBit { bit: 0 },
            (0xC, 0xE) => Instruction::SetHLBit { bit: 1 },
            (0xC, reg) => {
                if reg < 8 {
                    Instruction::SetBit {
                        bit: 0,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::SetBit {
                        bit: 1,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xD, 0x6) => Instruction::SetHLBit { bit: 2 },
            (0xD, 0xE) => Instruction::SetHLBit { bit: 3 },
            (0xD, reg) => {
                if reg < 8 {
                    Instruction::SetBit {
                        bit: 2,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::SetBit {
                        bit: 3,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xE, 0x6) => Instruction::SetHLBit { bit: 4 },
            (0xE, 0xE) => Instruction::SetHLBit { bit: 5 },
            (0xE, reg) => {
                if reg < 8 {
                    Instruction::SetBit {
                        bit: 4,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::SetBit {
                        bit: 5,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            (0xF, 0x6) => Instruction::SetHLBit { bit: 6 },
            (0xF, 0xE) => Instruction::SetHLBit { bit: 7 },
            (0xF, reg) => {
                if reg < 8 {
                    Instruction::SetBit {
                        bit: 6,
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::SetBit {
                        bit: 7,
                        register: registers[reg as usize % 8],
                    }
                }
            }
            _ => Instruction::Invalid,
        }
    }

    pub fn execute(&mut self, instruction: Instruction, cpu_bus: &mut impl CpuBus) {
        match instruction {
            Instruction::Invalid => todo!(),
            // 8-bit load instructions
            Instruction::LoadReg { dst, src } => {
                let value = match src {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                match dst {
                    Register::B => self.b = value,
                    Register::C => self.c = value,
                    Register::D => self.d = value,
                    Register::E => self.e = value,
                    Register::H => self.h = value,
                    Register::L => self.l = value,
                    Register::A => self.a = value,
                }

                self.program_counter += 1;
            }
            Instruction::LoadReg8 { register } => {
                let data = cpu_bus.read(self.program_counter + 1);

                match register {
                    Register::B => self.b = data,
                    Register::C => self.c = data,
                    Register::D => self.d = data,
                    Register::E => self.e = data,
                    Register::H => self.h = data,
                    Register::L => self.l = data,
                    Register::A => self.a = data,
                }

                self.program_counter += 2;
            }
            Instruction::LoadRegHL { register } => {
                let data = cpu_bus.read(self.hl());

                match register {
                    Register::B => self.b = data,
                    Register::C => self.c = data,
                    Register::D => self.d = data,
                    Register::E => self.e = data,
                    Register::H => self.h = data,
                    Register::L => self.l = data,
                    Register::A => self.a = data,
                }

                self.program_counter += 1;
            }
            Instruction::LoadHLReg { register } => {
                let address = self.hl();

                match register {
                    Register::B => cpu_bus.write(address, self.b),
                    Register::C => cpu_bus.write(address, self.c),
                    Register::D => cpu_bus.write(address, self.d),
                    Register::E => cpu_bus.write(address, self.e),
                    Register::H => cpu_bus.write(address, self.h),
                    Register::L => cpu_bus.write(address, self.l),
                    Register::A => cpu_bus.write(address, self.a),
                }

                self.program_counter += 1;
            }
            Instruction::LoadHL8 => {
                let data = cpu_bus.read(self.program_counter + 1);
                cpu_bus.write(self.hl(), data);
                self.program_counter += 2;
            }
            Instruction::LoadABC => {
                self.a = cpu_bus.read(self.bc());
                self.program_counter += 1;
            }
            Instruction::LoadADE => {
                self.a = cpu_bus.read(self.de());
                self.program_counter += 1;
            }
            Instruction::LoadAAddress => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                self.a = cpu_bus.read(address);
                self.program_counter += 3;
            }
            Instruction::LoadBCA => {
                cpu_bus.write(self.bc(), self.a);
                self.program_counter += 1;
            }
            Instruction::LoadDEA => {
                cpu_bus.write(self.de(), self.a);
                self.program_counter += 1;
            }
            Instruction::LoadAddressA => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                cpu_bus.write(address, self.a);
                self.program_counter += 3;
            }
            Instruction::LoadAOffset => {
                let offset = cpu_bus.read(self.program_counter + 1) as u16;
                self.a = cpu_bus.read(0xFF00 + offset);
                self.program_counter += 2;
            }
            Instruction::LoadOffsetA => {
                let offset = cpu_bus.read(self.program_counter + 1) as u16;
                cpu_bus.write(0xFF00 + offset, self.a);
                self.program_counter += 2;
            }
            Instruction::LoadAOffsetC => {
                self.a = cpu_bus.read(0xFF00 + self.c as u16);
                self.program_counter += 1;
            }
            Instruction::LoadOffsetCA => {
                cpu_bus.write(0xFF00 + self.c as u16, self.a);
                self.program_counter += 1;
            }
            Instruction::LoadIncrementHLA => {
                cpu_bus.write(self.hl(), self.a);
                self.increment_hl();
                self.program_counter += 1;
            }
            Instruction::LoadIncrementAHL => {
                self.a = cpu_bus.read(self.hl());
                self.increment_hl();
                self.program_counter += 1;
            }
            Instruction::LoadDecrementHLA => {
                cpu_bus.write(self.hl(), self.a);
                self.decrement_hl();
                self.program_counter += 1;
            }
            Instruction::LoadDecrementAHL => {
                self.a = cpu_bus.read(self.hl());
                self.decrement_hl();
                self.program_counter += 1;
            }

            // 16-bit load instructions
            Instruction::LoadReg16 { register } => {
                let lower = cpu_bus.read(self.program_counter + 1);
                let upper = cpu_bus.read(self.program_counter + 2);

                match register {
                    DoubleRegister::BC => {
                        self.b = upper;
                        self.c = lower;
                    }
                    DoubleRegister::DE => {
                        self.d = upper;
                        self.e = lower;
                    }
                    DoubleRegister::HL => {
                        self.h = upper;
                        self.l = lower;
                    }
                    DoubleRegister::SP => {
                        self.stack_pointer = combine_bytes(upper, lower);
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.program_counter += 3;
            }
            Instruction::LoadAddressSP => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                cpu_bus.write(address, get_lower_byte(self.stack_pointer));
                cpu_bus.write(address + 1, get_upper_byte(self.stack_pointer));
                self.program_counter += 3;
            }
            Instruction::LoadSPHL => {
                self.stack_pointer = self.hl();
                self.program_counter += 1;
            }
            Instruction::PushReg { register } => {
                self.stack_pointer -= 2;

                match register {
                    DoubleRegister::BC => {
                        cpu_bus.write(self.stack_pointer, self.c);
                        cpu_bus.write(self.stack_pointer + 1, self.b);
                    }
                    DoubleRegister::DE => {
                        cpu_bus.write(self.stack_pointer, self.e);
                        cpu_bus.write(self.stack_pointer + 1, self.d);
                    }
                    DoubleRegister::HL => {
                        cpu_bus.write(self.stack_pointer, self.l);
                        cpu_bus.write(self.stack_pointer + 1, self.h);
                    }
                    DoubleRegister::AF => {
                        cpu_bus.write(self.stack_pointer, self.flags_to_byte());
                        cpu_bus.write(self.stack_pointer + 1, self.a);
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.program_counter += 1;
            }
            Instruction::PopReg { register } => {
                let lower = cpu_bus.read(self.stack_pointer);
                let upper = cpu_bus.read(self.stack_pointer + 1);

                match register {
                    DoubleRegister::BC => {
                        self.b = upper;
                        self.c = lower;
                    }
                    DoubleRegister::DE => {
                        self.d = upper;
                        self.e = lower;
                    }
                    DoubleRegister::HL => {
                        self.h = upper;
                        self.l = lower;
                    }
                    DoubleRegister::AF => {
                        self.a = upper;
                        self.byte_to_flags(lower);
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.stack_pointer += 2;
                self.program_counter += 1;
            }

            // 8-bit Arithmetic/Logic instructions
            Instruction::AddAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.wrapped_addition(value);
                self.program_counter += 1;
            }
            Instruction::AddA => {
                let value = cpu_bus.read(self.program_counter + 1);
                self.wrapped_addition(value);
                self.program_counter += 2;
            }
            Instruction::AddAHL => {
                let value = cpu_bus.read(self.hl());
                self.wrapped_addition(value);
                self.program_counter += 1;
            }
            Instruction::AddCarryAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.wrapped_addition_carry(value, self.is_carry);
                self.program_counter += 1;
            }
            Instruction::AddCarryA => {
                let value = cpu_bus.read(self.program_counter + 1);
                self.wrapped_addition_carry(value, self.is_carry);
                self.program_counter += 2;
            }
            Instruction::AddCarryAHL => {
                let value = cpu_bus.read(self.hl());
                self.wrapped_addition_carry(value, self.is_carry);
                self.program_counter += 1;
            }
            Instruction::SubtractAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.a = self.wrapped_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractA => {
                let value = cpu_bus.read(self.program_counter + 1);
                self.a = self.wrapped_subtraction(value);
                self.program_counter += 2;
            }
            Instruction::SubtractAHL => {
                let value = cpu_bus.read(self.hl());
                self.a = self.wrapped_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractARegCarry { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.a = self.wrapped_subtraction_carry(value, self.is_carry);
                self.program_counter += 1;
            }
            Instruction::SubtractACarry => {
                let value = cpu_bus.read(self.program_counter + 1);
                self.a = self.wrapped_subtraction_carry(value, self.is_carry);
                self.program_counter += 2;
            }
            Instruction::SubtractAHLCarry => {
                let value = cpu_bus.read(self.hl());
                self.a = self.wrapped_subtraction_carry(value, self.is_carry);
                self.program_counter += 1;
            }
            Instruction::AndAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.a &= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = true;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::AndA => {
                let value = cpu_bus.read(self.program_counter + 1);

                self.a &= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = true;
                self.is_carry = false;

                self.program_counter += 2;
            }
            Instruction::AndAHL => {
                let value = cpu_bus.read(self.hl());

                self.a &= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = true;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::XorAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.a ^= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::XorA => {
                let value = cpu_bus.read(self.program_counter + 1);

                self.a ^= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 2;
            }
            Instruction::XorAHL => {
                let value = cpu_bus.read(self.hl());

                self.a ^= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::OrAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                self.a |= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::OrA => {
                let value = cpu_bus.read(self.program_counter + 1);

                self.a |= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 2;
            }
            Instruction::OrAHL => {
                let value = cpu_bus.read(self.hl());

                self.a |= value;

                self.is_zero = self.a == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;

                self.program_counter += 1;
            }
            Instruction::CompareAReg { register } => {
                let value = match register {
                    Register::B => self.b,
                    Register::C => self.c,
                    Register::D => self.d,
                    Register::E => self.e,
                    Register::H => self.h,
                    Register::L => self.l,
                    Register::A => self.a,
                };

                _ = self.wrapped_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::CompareA => {
                let value = cpu_bus.read(self.program_counter + 1);
                _ = self.wrapped_subtraction(value);
                self.program_counter += 2;
            }
            Instruction::CompareAHL => {
                let value = cpu_bus.read(self.hl());
                _ = self.wrapped_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::IncrementReg { register } => {
                let alu = match register {
                    Register::B => {
                        let alu = AluResult::from_add(self.b, 1);
                        self.b = alu.result;
                        alu
                    }
                    Register::C => {
                        let alu = AluResult::from_add(self.c, 1);
                        self.c = alu.result;
                        alu
                    }
                    Register::D => {
                        let alu = AluResult::from_add(self.d, 1);
                        self.d = alu.result;
                        alu
                    }
                    Register::E => {
                        let alu = AluResult::from_add(self.e, 1);
                        self.e = alu.result;
                        alu
                    }
                    Register::H => {
                        let alu = AluResult::from_add(self.h, 1);
                        self.h = alu.result;
                        alu
                    }
                    Register::L => {
                        let alu = AluResult::from_add(self.l, 1);
                        self.l = alu.result;
                        alu
                    }
                    Register::A => {
                        let alu = AluResult::from_add(self.a, 1);
                        self.a = alu.result;
                        alu
                    }
                };

                self.is_zero = alu.result == 0;
                self.is_subtraction = false;
                self.is_half_carry = alu.half_carry;

                self.program_counter += 1;
            }
            Instruction::IncrementHL => {
                let data = cpu_bus.read(self.hl());
                let alu = AluResult::from_add(data, 1);

                self.is_zero = alu.result == 0;
                self.is_subtraction = false;
                self.is_half_carry = alu.half_carry;

                cpu_bus.write(self.hl(), alu.result);
                self.program_counter += 1;
            }
            Instruction::DecrementReg { register } => {
                let alu = match register {
                    Register::B => {
                        let alu = AluResult::from_sub(self.b, 1);
                        self.b = alu.result;
                        alu
                    }
                    Register::C => {
                        let alu = AluResult::from_sub(self.c, 1);
                        self.c = alu.result;
                        alu
                    }
                    Register::D => {
                        let alu = AluResult::from_sub(self.d, 1);
                        self.d = alu.result;
                        alu
                    }
                    Register::E => {
                        let alu = AluResult::from_sub(self.e, 1);
                        self.e = alu.result;
                        alu
                    }
                    Register::H => {
                        let alu = AluResult::from_sub(self.h, 1);
                        self.h = alu.result;
                        alu
                    }
                    Register::L => {
                        let alu = AluResult::from_sub(self.l, 1);
                        self.l = alu.result;
                        alu
                    }
                    Register::A => {
                        let alu = AluResult::from_sub(self.a, 1);
                        self.a = alu.result;
                        alu
                    }
                };

                self.is_zero = alu.result == 0;
                self.is_subtraction = true;
                self.is_half_carry = alu.half_carry;

                self.program_counter += 1;
            }
            Instruction::DecrementHL => {
                let data = cpu_bus.read(self.hl());
                let alu = AluResult::from_sub(data, 1);

                self.is_zero = alu.result == 0;
                self.is_subtraction = true;
                self.is_half_carry = alu.half_carry;

                cpu_bus.write(self.hl(), alu.result);
                self.program_counter += 1;
            }
            Instruction::DecimalAdjustA => {
                let mut correction: u16 = 0;

                if self.is_half_carry || (!self.is_subtraction && (self.a & 0x0F) > 9) {
                    correction += 0x06;
                }

                if self.is_carry || (!self.is_subtraction && self.a > 0x99) {
                    correction += 0x60;
                    self.is_carry = true;
                }

                if self.is_subtraction {
                    let result = self.a as i16 - correction as i16;
                    self.a = (result & 0x00FF) as u8;
                } else {
                    let result = self.a as u16 + correction;
                    self.a = (result & 0x00FF) as u8;
                }

                self.is_zero = self.a == 0;
                self.is_half_carry = false;
                self.program_counter += 1;
            }
            Instruction::Complement => {
                self.a ^= 0xFF;
                self.is_subtraction = true;
                self.is_half_carry = true;
                self.program_counter += 1;
            }

            // 16-bit Arithmetic/Logic instructions
            Instruction::AddHLReg { register } => {
                let value = match register {
                    DoubleRegister::BC => self.bc(),
                    DoubleRegister::DE => self.de(),
                    DoubleRegister::HL => self.hl(),
                    DoubleRegister::SP => self.stack_pointer,
                    _ => panic!("Invalid Instruction"),
                };

                let alu = AluResult::from_add(self.l, get_lower_byte(value));
                self.l = alu.result;

                let alu = AluResult::from_adc(self.h, get_upper_byte(value), alu.carry);

                self.is_subtraction = false;
                self.is_half_carry = alu.half_carry;
                self.is_carry = alu.carry;

                self.h = alu.result;
                self.program_counter += 1;
            }
            Instruction::IncrementReg16 { register } => {
                match register {
                    DoubleRegister::BC => {
                        if self.bc() == u16::MAX {
                            self.b = 0;
                            self.c = 0;
                        } else {
                            let result = self.bc() + 1;
                            self.b = get_upper_byte(result);
                            self.c = get_lower_byte(result);
                        }
                    }
                    DoubleRegister::DE => {
                        if self.de() == u16::MAX {
                            self.d = 0;
                            self.e = 0;
                        } else {
                            let result = self.de() + 1;
                            self.d = get_upper_byte(result);
                            self.e = get_lower_byte(result);
                        }
                    }
                    DoubleRegister::HL => self.increment_hl(),
                    DoubleRegister::SP => {
                        if self.stack_pointer == u16::MAX {
                            self.stack_pointer = 0;
                        } else {
                            self.stack_pointer += 1;
                        }
                    }
                    _ => panic!("Invalid Instruction"),
                };
                self.program_counter += 1;
            }
            Instruction::DecrementReg16 { register } => {
                match register {
                    DoubleRegister::BC => {
                        if self.bc() == 0 {
                            self.b = u8::MAX;
                            self.c = u8::MAX;
                        } else {
                            let result = self.bc() - 1;
                            self.b = get_upper_byte(result);
                            self.c = get_lower_byte(result);
                        }
                    }
                    DoubleRegister::DE => {
                        if self.de() == 0 {
                            self.d = u8::MAX;
                            self.e = u8::MAX;
                        } else {
                            let result = self.de() - 1;
                            self.d = get_upper_byte(result);
                            self.e = get_lower_byte(result);
                        }
                    }
                    DoubleRegister::HL => self.decrement_hl(),
                    DoubleRegister::SP => {
                        if self.stack_pointer == 0 {
                            self.stack_pointer = u16::MAX;
                        } else {
                            self.stack_pointer -= 1;
                        }
                    }
                    _ => panic!("Invalid Instruction"),
                };
                self.program_counter += 1;
            }
            Instruction::AddSPOffset => {
                let offset = cpu_bus.read(self.program_counter + 1);

                self.is_zero = false;
                self.is_subtraction = false;

                let alu = AluResult::from_add(get_lower_byte(self.stack_pointer), offset);

                self.is_half_carry = alu.half_carry;
                self.is_carry = alu.carry;

                self.stack_pointer = self.stack_pointer.wrapping_add(offset as i8 as u16);
                self.program_counter += 2;
            }
            Instruction::LoadHLSPOffset => {
                let offset = cpu_bus.read(self.program_counter + 1);

                self.is_zero = false;
                self.is_subtraction = false;

                let alu = AluResult::from_add(get_lower_byte(self.stack_pointer), offset);

                self.is_half_carry = alu.half_carry;
                self.is_carry = alu.carry;

                let sp = self.stack_pointer.wrapping_add(offset as i8 as u16);
                self.h = get_upper_byte(sp);
                self.l = get_lower_byte(sp);
                self.program_counter += 2;
            }

            // Rotate and Shift instructions
            Instruction::RotateALeft => {
                self.a = self.rotate_left(self.a);

                self.is_zero = false;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 1;
            }
            Instruction::RotateALeftThroughCarry => {
                self.a = self.rotate_left_through_carry(self.a);

                self.is_zero = false;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 1;
            }
            Instruction::RotateARight => {
                self.a = self.rotate_right(self.a);

                self.is_zero = false;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 1;
            }
            Instruction::RotateARightThroughCarry => {
                self.a = self.rotate_right_through_carry(self.a);

                self.is_zero = false;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 1;
            }
            Instruction::RotateLeft { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_left(self.b);
                        self.is_zero = self.b == 0;
                    }
                    Register::C => {
                        self.c = self.rotate_left(self.c);
                        self.is_zero = self.c == 0;
                    }
                    Register::D => {
                        self.d = self.rotate_left(self.d);
                        self.is_zero = self.d == 0;
                    }
                    Register::E => {
                        self.e = self.rotate_left(self.e);
                        self.is_zero = self.e == 0;
                    }
                    Register::H => {
                        self.h = self.rotate_left(self.h);
                        self.is_zero = self.h == 0;
                    }
                    Register::L => {
                        self.l = self.rotate_left(self.l);
                        self.is_zero = self.l == 0;
                    }
                    Register::A => {
                        self.a = self.rotate_left(self.a);
                        self.is_zero = self.a == 0;
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateHLLeft => {
                let data = cpu_bus.read(self.hl());
                let result = self.rotate_left(data);
                cpu_bus.write(self.hl(), result);

                self.is_zero = result == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateLeftThroughCarry { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_left_through_carry(self.b);
                        self.is_zero = self.b == 0;
                    }
                    Register::C => {
                        self.c = self.rotate_left_through_carry(self.c);
                        self.is_zero = self.c == 0;
                    }
                    Register::D => {
                        self.d = self.rotate_left_through_carry(self.d);
                        self.is_zero = self.d == 0;
                    }
                    Register::E => {
                        self.e = self.rotate_left_through_carry(self.e);
                        self.is_zero = self.e == 0;
                    }
                    Register::H => {
                        self.h = self.rotate_left_through_carry(self.h);
                        self.is_zero = self.h == 0;
                    }
                    Register::L => {
                        self.l = self.rotate_left_through_carry(self.l);
                        self.is_zero = self.l == 0;
                    }
                    Register::A => {
                        self.a = self.rotate_left_through_carry(self.a);
                        self.is_zero = self.a == 0;
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateHLLeftThroughCarry => {
                let data = cpu_bus.read(self.hl());
                let result = self.rotate_left_through_carry(data);
                cpu_bus.write(self.hl(), result);

                self.is_zero = result == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateRight { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_right(self.b);
                        self.is_zero = self.b == 0;
                    }
                    Register::C => {
                        self.c = self.rotate_right(self.c);
                        self.is_zero = self.c == 0;
                    }
                    Register::D => {
                        self.d = self.rotate_right(self.d);
                        self.is_zero = self.d == 0;
                    }
                    Register::E => {
                        self.e = self.rotate_right(self.e);
                        self.is_zero = self.e == 0;
                    }
                    Register::H => {
                        self.h = self.rotate_right(self.h);
                        self.is_zero = self.h == 0;
                    }
                    Register::L => {
                        self.l = self.rotate_right(self.l);
                        self.is_zero = self.l == 0;
                    }
                    Register::A => {
                        self.a = self.rotate_right(self.a);
                        self.is_zero = self.a == 0;
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateHLRight => {
                let data = cpu_bus.read(self.hl());
                let result = self.rotate_right(data);
                cpu_bus.write(self.hl(), result);

                self.is_zero = result == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateRightThroughCarry { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_right_through_carry(self.b);
                        self.is_zero = self.b == 0;
                    }
                    Register::C => {
                        self.c = self.rotate_right_through_carry(self.c);
                        self.is_zero = self.c == 0;
                    }
                    Register::D => {
                        self.d = self.rotate_right_through_carry(self.d);
                        self.is_zero = self.d == 0;
                    }
                    Register::E => {
                        self.e = self.rotate_right_through_carry(self.e);
                        self.is_zero = self.e == 0;
                    }
                    Register::H => {
                        self.h = self.rotate_right_through_carry(self.h);
                        self.is_zero = self.h == 0;
                    }
                    Register::L => {
                        self.l = self.rotate_right_through_carry(self.l);
                        self.is_zero = self.l == 0;
                    }
                    Register::A => {
                        self.a = self.rotate_right_through_carry(self.a);
                        self.is_zero = self.a == 0;
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::RotateHLRightThroughCarry => {
                let data = cpu_bus.read(self.hl());
                let result = self.rotate_right_through_carry(data);
                cpu_bus.write(self.hl(), result);

                self.is_zero = result == 0;
                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::ShiftLeftArithmetic { register } => {
                match register {
                    Register::B => {
                        self.b = self.shift_left(self.b);
                    }
                    Register::C => {
                        self.c = self.shift_left(self.c);
                    }
                    Register::D => {
                        self.d = self.shift_left(self.d);
                    }
                    Register::E => {
                        self.e = self.shift_left(self.e);
                    }
                    Register::H => {
                        self.h = self.shift_left(self.h);
                    }
                    Register::L => {
                        self.l = self.shift_left(self.l);
                    }
                    Register::A => {
                        self.a = self.shift_left(self.a);
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::ShiftHLLeftArithmetic => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.shift_left(data));

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::Swap { register } => {
                match register {
                    Register::B => {
                        self.b = self.swap(self.b);
                    }
                    Register::C => {
                        self.c = self.swap(self.c);
                    }
                    Register::D => {
                        self.d = self.swap(self.d);
                    }
                    Register::E => {
                        self.e = self.swap(self.e);
                    }
                    Register::H => {
                        self.h = self.swap(self.h);
                    }
                    Register::L => {
                        self.l = self.swap(self.l);
                    }
                    Register::A => {
                        self.a = self.swap(self.a);
                    }
                }

                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;
                self.program_counter += 2;
            }
            Instruction::SwapHL => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.swap(data));

                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = false;
                self.program_counter += 2;
            }
            Instruction::ShiftRightArithmetic { register } => {
                match register {
                    Register::B => self.b = self.shift_right_arithmetic(self.b),
                    Register::C => self.c = self.shift_right_arithmetic(self.c),
                    Register::D => self.d = self.shift_right_arithmetic(self.d),
                    Register::E => self.e = self.shift_right_arithmetic(self.e),
                    Register::H => self.h = self.shift_right_arithmetic(self.h),
                    Register::L => self.l = self.shift_right_arithmetic(self.l),
                    Register::A => self.a = self.shift_right_arithmetic(self.a),
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::ShiftHLRightArithmetic => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.shift_right_arithmetic(data));

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::ShiftRightLogical { register } => {
                match register {
                    Register::B => self.b = self.shift_right_logical(self.b),
                    Register::C => self.c = self.shift_right_logical(self.c),
                    Register::D => self.d = self.shift_right_logical(self.d),
                    Register::E => self.e = self.shift_right_logical(self.e),
                    Register::H => self.h = self.shift_right_logical(self.h),
                    Register::L => self.l = self.shift_right_logical(self.l),
                    Register::A => self.a = self.shift_right_logical(self.a),
                }

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }
            Instruction::ShiftHLRightLogical => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.shift_right_logical(data));

                self.is_subtraction = false;
                self.is_half_carry = false;

                self.program_counter += 2;
            }

            // Single-bit operation instructions
            Instruction::TestBit { bit, register } => {
                match register {
                    Register::B => self.test_bit(bit, self.b),
                    Register::C => self.test_bit(bit, self.c),
                    Register::D => self.test_bit(bit, self.d),
                    Register::E => self.test_bit(bit, self.e),
                    Register::H => self.test_bit(bit, self.h),
                    Register::L => self.test_bit(bit, self.l),
                    Register::A => self.test_bit(bit, self.a),
                }

                self.is_subtraction = false;
                self.is_half_carry = true;
                self.program_counter += 2;
            }
            Instruction::TestHLBit { bit } => {
                let data = cpu_bus.read(self.hl());
                self.test_bit(bit, data);

                self.is_subtraction = false;
                self.is_half_carry = true;
                self.program_counter += 2;
            }
            Instruction::SetBit { bit, register } => {
                match register {
                    Register::B => self.b = self.set_bit(bit, self.b),
                    Register::C => self.c = self.set_bit(bit, self.c),
                    Register::D => self.d = self.set_bit(bit, self.d),
                    Register::E => self.e = self.set_bit(bit, self.e),
                    Register::H => self.h = self.set_bit(bit, self.h),
                    Register::L => self.l = self.set_bit(bit, self.l),
                    Register::A => self.a = self.set_bit(bit, self.a),
                }

                self.program_counter += 2;
            }
            Instruction::SetHLBit { bit } => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.set_bit(bit, data));

                self.program_counter += 2;
            }
            Instruction::ResetBit { bit, register } => {
                match register {
                    Register::B => self.b = self.reset_bit(bit, self.b),
                    Register::C => self.c = self.reset_bit(bit, self.c),
                    Register::D => self.d = self.reset_bit(bit, self.d),
                    Register::E => self.e = self.reset_bit(bit, self.e),
                    Register::H => self.h = self.reset_bit(bit, self.h),
                    Register::L => self.l = self.reset_bit(bit, self.l),
                    Register::A => self.a = self.reset_bit(bit, self.a),
                }

                self.program_counter += 2;
            }
            Instruction::ResetHLBit { bit } => {
                let data = cpu_bus.read(self.hl());
                cpu_bus.write(self.hl(), self.reset_bit(bit, data));

                self.program_counter += 2;
            }

            // CPU Control instructions
            Instruction::FlipCarryFlag => {
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = !self.is_carry;
                self.program_counter += 1;
            }
            Instruction::SetCarryFlag => {
                self.is_subtraction = false;
                self.is_half_carry = false;
                self.is_carry = true;
                self.program_counter += 1;
            }
            Instruction::Nop => {
                self.program_counter += 1;
            }
            Instruction::Halt => {
                self.halt = true;
            }
            Instruction::Stop => todo!(),
            Instruction::DisableInterrupts => {
                self.interrupts_enabled = false;
                self.program_counter += 1;
            }
            Instruction::EnableInterrupts => {
                self.interrupts_enabled = true;
                self.program_counter += 1;
            }

            // Jump instructions
            Instruction::Jump => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);
                self.program_counter = combine_bytes(high, low);
            }
            Instruction::JumpHL => {
                self.program_counter = self.hl();
            }
            Instruction::JumpConditional { flag } => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);

                let predicate = match flag {
                    ConditionalFlag::NZ => !self.is_zero,
                    ConditionalFlag::Z => self.is_zero,
                    ConditionalFlag::NC => !self.is_carry,
                    ConditionalFlag::C => self.is_carry,
                };

                if predicate {
                    self.program_counter = combine_bytes(high, low);
                } else {
                    self.program_counter += 3;
                }
            }
            Instruction::JumpRelative => {
                let offset = cpu_bus.read(self.program_counter + 1) as i8;

                self.program_counter += 2;

                if offset > 0 {
                    self.program_counter += offset as u16;
                } else {
                    self.program_counter -= offset.unsigned_abs() as u16;
                }
            }
            Instruction::JumpRelativeConditional { flag } => {
                let offset = cpu_bus.read(self.program_counter + 1) as i8;

                let predicate = match flag {
                    ConditionalFlag::NZ => !self.is_zero,
                    ConditionalFlag::Z => self.is_zero,
                    ConditionalFlag::NC => !self.is_carry,
                    ConditionalFlag::C => self.is_carry,
                };

                self.program_counter += 2;

                if predicate {
                    if offset > 0 {
                        self.program_counter += offset as u16;
                    } else {
                        self.program_counter -= offset.unsigned_abs() as u16;
                    }
                }
            }
            Instruction::Call => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);

                self.program_counter += 3;
                self.call_address(cpu_bus, combine_bytes(high, low));
            }
            Instruction::CallConditional { flag } => {
                let low = cpu_bus.read(self.program_counter + 1);
                let high = cpu_bus.read(self.program_counter + 2);

                let predicate = match flag {
                    ConditionalFlag::NZ => !self.is_zero,
                    ConditionalFlag::Z => self.is_zero,
                    ConditionalFlag::NC => !self.is_carry,
                    ConditionalFlag::C => self.is_carry,
                };

                self.program_counter += 3;

                if predicate {
                    self.call_address(cpu_bus, combine_bytes(high, low));
                }
            }
            Instruction::Return => {
                let low = cpu_bus.read(self.stack_pointer);
                let high = cpu_bus.read(self.stack_pointer + 1);
                self.program_counter = combine_bytes(high, low);
                self.stack_pointer += 2;
            }
            Instruction::ReturnConditional { flag } => {
                let predicate = match flag {
                    ConditionalFlag::NZ => !self.is_zero,
                    ConditionalFlag::Z => self.is_zero,
                    ConditionalFlag::NC => !self.is_carry,
                    ConditionalFlag::C => self.is_carry,
                };

                if predicate {
                    let low = cpu_bus.read(self.stack_pointer);
                    let high = cpu_bus.read(self.stack_pointer + 1);
                    self.program_counter = combine_bytes(high, low);
                    self.stack_pointer += 2;
                } else {
                    self.program_counter += 1;
                }
            }
            Instruction::ReturnAndEnableInterrupts => {
                let low = cpu_bus.read(self.stack_pointer);
                let high = cpu_bus.read(self.stack_pointer + 1);
                self.interrupts_enabled = true;
                self.program_counter = combine_bytes(high, low);
                self.stack_pointer += 2;
            }
            Instruction::Reset0 { location } => {
                self.program_counter += 1;
                self.call_address(cpu_bus, ((location % 4) * 0x10) as u16);
            }
            Instruction::Reset8 { location } => {
                self.program_counter += 1;
                self.call_address(cpu_bus, (((location % 4) * 0x10) + 0x8) as u16)
            }
        }

        self.check_interrupts(instruction, cpu_bus);
    }

    /// If interrutps are enabled, check if any interrupts are requested and if so call the interrupt handler
    fn check_interrupts(&mut self, instruction: Instruction, cpu_bus: &mut impl CpuBus) {
        if instruction != Instruction::EnableInterrupts && self.interrupts_enabled {
            // mask the relevant flag bits just in case
            let requested_interrupt_flags = cpu_bus.read(0xFF0F) & 0x1F;
            let enabled_interrupt_flags = cpu_bus.read(0xFFFF) & 0x1F;

            //TODO: interrupts should take 5 M-cycles, but this should ony take 2

            // we only want to process interrupts that are both requested and enabled
            if requested_interrupt_flags & enabled_interrupt_flags > 0 {
                let interrupt_flags = get_as_bits(requested_interrupt_flags);
                self.interrupts_enabled = false;
                // process interrupts in order of priority and clear the IF bit of the processed interrupt
                if interrupt_flags[7] == 1 {
                    // VBlank
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFE);
                    self.call_address(cpu_bus, 0x40);
                } else if interrupt_flags[6] == 1 {
                    // LCD STAT
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFD);
                    self.call_address(cpu_bus, 0x48);
                } else if interrupt_flags[5] == 1 {
                    // Timer
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFB);
                    self.call_address(cpu_bus, 0x50);
                } else if interrupt_flags[4] == 1 {
                    // Serial
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xF7);
                    self.call_address(cpu_bus, 0x58);
                } else if interrupt_flags[3] == 1 {
                    // Joypad
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xEF);
                    self.call_address(cpu_bus, 0x60);
                }
            }
        } else if self.halt {
            // mask the relevant flag bits just in case
            let requested_interrupt_flags = cpu_bus.read(0xFF0F) & 0x1F;
            let enabled_interrupt_flags = cpu_bus.read(0xFFFF) & 0x1F;

            //TODO: interrupts should take 5 M-cycles, but this should ony take 2

            // we only want to process interrupts that are both requested and enabled
            if requested_interrupt_flags & enabled_interrupt_flags > 0 {
                let interrupt_flags = get_as_bits(requested_interrupt_flags);
                self.interrupts_enabled = false;
                // process interrupts in order of priority and clear the IF bit of the processed interrupt
                if interrupt_flags[7] == 1 {
                    // VBlank
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFE);
                    self.halt = false;
                    self.program_counter += 1;
                } else if interrupt_flags[6] == 1 {
                    // LCD STAT
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFD);
                    self.halt = false;
                    self.program_counter += 1;
                } else if interrupt_flags[5] == 1 {
                    // Timer
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xFB);
                    self.halt = false;
                    self.program_counter += 1;
                } else if interrupt_flags[4] == 1 {
                    // Serial
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xF7);
                    self.halt = false;
                    self.program_counter += 1;
                } else if interrupt_flags[3] == 1 {
                    // Joypad
                    cpu_bus.write(0xFF0F, requested_interrupt_flags & 0xEF);
                    self.halt = false;
                    self.program_counter += 1;
                }
            }
        }
    }

    /// Adds `value` to the `A` register and sets the appropriate flags (z0hc)
    fn wrapped_addition(&mut self, value: u8) {
        let alu = AluResult::from_add(self.a, value);

        self.a = alu.result;
        self.is_zero = self.a == 0;
        self.is_subtraction = false;
        self.is_half_carry = alu.half_carry;
        self.is_carry = alu.carry;
    }

    /// Adds `value` to the `A` register and sets the appropriate flags (z0hc)
    fn wrapped_addition_carry(&mut self, value: u8, carry: bool) {
        let alu = AluResult::from_adc(self.a, value, carry);

        self.a = alu.result;
        self.is_zero = self.a == 0;
        self.is_subtraction = false;
        self.is_half_carry = alu.half_carry;
        self.is_carry = alu.carry;
    }

    /// Returns the result of subtracting `value` from the `A` register and sets the appropriate flags (z1hc)
    fn wrapped_subtraction(&mut self, value: u8) -> u8 {
        let alu = AluResult::from_sub(self.a, value);

        self.is_zero = alu.result == 0;
        self.is_subtraction = true;
        self.is_half_carry = alu.half_carry;
        self.is_carry = alu.carry;

        alu.result
    }

    /// Returns the result of subtracting `value` from the `A` register and sets the appropriate flags (z1hc)
    fn wrapped_subtraction_carry(&mut self, value: u8, carry: bool) -> u8 {
        let alu = AluResult::from_sbc(self.a, value, carry);

        self.is_zero = alu.result == 0;
        self.is_subtraction = true;
        self.is_half_carry = alu.half_carry;
        self.is_carry = alu.carry;

        alu.result
    }

    /// Returns the left rotated value and sets the carry flag
    fn rotate_left(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = msb >> 7;
        let result = (value << 1) + carry;

        self.is_carry = carry == 1;

        result
    }

    /// Returns the left rotated through carry value and sets the carry flag
    fn rotate_left_through_carry(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = if self.is_carry { 1 } else { 0 };
        let result = (value << 1) + carry;

        self.is_carry = msb != 0;

        result
    }

    /// Returns the right rotated value and sets the carry flag
    fn rotate_right(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let result = (value >> 1) + (lsb << 7);

        self.is_carry = lsb == 1;

        result
    }

    /// Returns the right rotated through carry value and sets the carry flag
    fn rotate_right_through_carry(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let carry = if self.is_carry { 0b1000_0000 } else { 0 };
        let result = (value >> 1) + carry;

        self.is_carry = lsb == 1;

        result
    }

    /// Returns the left shifted value and sets the zero and carry flags
    fn shift_left(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = msb >> 7;
        let result = value << 1;

        self.is_zero = result == 0;
        self.is_carry = carry == 1;

        result
    }

    /// Swap the upper and lower nibble of value and set the zero flag
    fn swap(&mut self, value: u8) -> u8 {
        let upper = get_upper_bits(value);
        let lower = get_lower_bits(value);
        let result = (lower << 4) + upper;

        self.is_zero = result == 0;

        result
    }

    /// Returns the arithmeticly right shifted value and sets the zero and carry flags
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let lsb = value & 0b0000_0001;
        let result = value >> 1 | msb;

        self.is_zero = result == 0;
        self.is_carry = lsb == 1;

        result
    }

    /// Returns the logically right shifted value and sets the zero and carry flags
    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let result = value >> 1;

        self.is_zero = result == 0;
        self.is_carry = lsb == 1;

        result
    }

    /// Test the bit of `value` at `position` and set the zero flag with the result
    ///
    /// Bits are indexed from 7->0, where bit 7 is the leftmost bit
    fn test_bit(&mut self, position: u8, value: u8) {
        let bits = get_as_bits(value);

        if position < 8 {
            self.is_zero = bits[7 - position as usize] == 0;
        }
    }

    /// Set the bit of `value` at `position`
    ///
    /// Bits are indexed from 7->0, where bit 7 is the leftmost bit
    fn set_bit(&mut self, position: u8, value: u8) -> u8 {
        let mut bits = get_as_bits(value);

        if position < 8 {
            bits[7 - position as usize] = 1;
            bits_to_u8(bits)
        } else {
            value
        }
    }

    /// Reset the bit of `value` at `position`
    ///
    /// Bits are indexed from 7->0, where bit 7 is the leftmost bit
    fn reset_bit(&mut self, position: u8, value: u8) -> u8 {
        let mut bits = get_as_bits(value);

        if position < 8 {
            bits[7 - position as usize] = 0;
            bits_to_u8(bits)
        } else {
            value
        }
    }

    /// Push the PC onto the stack, then set the PC to the given address
    fn call_address(&mut self, cpu_bus: &mut impl CpuBus, address: u16) {
        self.stack_pointer -= 2;
        cpu_bus.write(self.stack_pointer, get_lower_byte(self.program_counter));
        cpu_bus.write(self.stack_pointer + 1, get_upper_byte(self.program_counter));
        self.program_counter = address;
    }
}

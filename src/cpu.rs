use crate::{
    instructions::{ConditionalFlag, DoubleRegister, Instruction, Register},
    memory::Memory,
    util::*,
};

mod cpu_tests;

#[derive(Debug)]
pub struct Cpu {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub a: u8,
    pub f: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
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
            f: 0,
            stack_pointer: 0xFFFE,
            program_counter: 0,
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

    fn is_zero(&self) -> bool {
        self.f & 0b1000_0000 == 0b1000_0000
    }

    fn set_zero(&mut self, value: bool) {
        if value {
            self.f = self.f | 0b1000_0000;
        } else {
            self.f = self.f & 0b0111_1111;
        }
    }

    fn is_subtraction(&self) -> bool {
        self.f & 0b0100_0000 == 0b0100_0000
    }

    fn set_subtraction(&mut self, value: bool) {
        if value {
            self.f = self.f | 0b0100_0000;
        } else {
            self.f = self.f & 0b1011_1111;
        }
    }

    fn is_half_carry(&self) -> bool {
        self.f & 0b0010_0000 == 0b0010_0000
    }

    fn set_half_carry(&mut self, value: bool) {
        if value {
            self.f = self.f | 0b0010_0000;
        } else {
            self.f = self.f & 0b1101_1111;
        }
    }

    fn is_carry(&self) -> bool {
        self.f & 0b0001_0000 == 0b0001_0000
    }

    fn set_carry(&mut self, value: bool) {
        if value {
            self.f = self.f | 0b0001_0000;
        } else {
            self.f = self.f & 0b1110_1111;
        }
    }

    pub fn parse(&mut self, memory: &Memory) -> Instruction {
        let instruction = memory.rom[self.program_counter as usize];

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
            (0x1, 0x8) => Instruction::JumpRelative {
                flag: ConditionalFlag::None,
            },
            (0x1, 0xA) => Instruction::LoadADE,
            (0x1, 0xF) => Instruction::RotateARightThroughCarry,
            (0x2, 0x0) => Instruction::JumpRelative {
                flag: ConditionalFlag::NZ,
            },
            (0x2, 0x2) => Instruction::LoadIncrementHLA,
            (0x2, 0x7) => Instruction::DecimalAdjustA,
            (0x2, 0x8) => Instruction::JumpRelative {
                flag: ConditionalFlag::Z,
            },
            (0x2, 0xA) => Instruction::LoadIncrementAHL,
            (0x2, 0xF) => Instruction::Complement,
            (0x3, 0x0) => Instruction::JumpRelative {
                flag: ConditionalFlag::NC,
            },
            (0x3, 0x2) => Instruction::LoadDecrementHLA,
            (0x3, 0x4) => Instruction::IncrementHL,
            (0x3, 0x5) => Instruction::DecrementHL,
            (0x3, 0x6) => Instruction::LoadHL8,
            (0x3, 0x7) => Instruction::Scf,
            (0x3, 0x8) => Instruction::JumpRelative {
                flag: ConditionalFlag::C,
            },
            (0x3, 0xA) => Instruction::LoadDecrementAHL,
            (0x3, 0xF) => Instruction::Ccf,
            (0x4, 0x6) => Instruction::LoadRegHL {
                register: Register::B,
            },
            (0x4, 0xE) => Instruction::LoadRegHL {
                register: Register::C,
            },
            (0x4, reg) => {
                if reg <= 7 {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::B,
                    }
                } else {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::C,
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
                if reg <= 7 {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::D,
                    }
                } else {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::E,
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
                if reg <= 7 {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::H,
                    }
                } else {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::L,
                    }
                }
            }
            (0x7, 0x6) => Instruction::Halt,
            (0x7, 0xE) => Instruction::LoadRegHL {
                register: Register::A,
            },
            (0x7, reg) => {
                if reg <= 7 {
                    let registers = [
                        Register::B,
                        Register::C,
                        Register::D,
                        Register::E,
                        Register::H,
                        Register::L,
                        Register::A,
                    ];
                    Instruction::LoadHLReg {
                        register: registers[reg as usize],
                    }
                } else {
                    Instruction::LoadReg {
                        load_from: reg,
                        load_into: Register::A,
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
            (0xC, 0x0) => Instruction::ReturnIfNotZero,
            (0xC, 0x2) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::JumpIfNotZero { address }
            }
            (0xC, 0x3) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::Jump { address }
            }
            (0xC, 0x4) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::CallIfNotZero { address }
            }
            (0xC, 0x6) => Instruction::AddA,
            (0xC, 0x8) => Instruction::ReturnIfZero,
            (0xC, 0x9) => Instruction::Return,
            (0xC, 0xA) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::JumpIfZero { address }
            }
            (0xC, 0xB) => self.parse_prefix(memory),
            (0xC, 0xC) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::CallIfZero { address }
            }
            (0xC, 0xD) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::Call { address }
            }
            (0xC, 0xE) => Instruction::AddCarryA,
            (0xD, 0x0) => Instruction::ReturnIfNotCarry,
            (0xD, 0x2) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::JumpIfNotCarry { address }
            }
            (0xD, 0x3) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::CallIfNotCarry { address }
            }
            (0xD, 0x6) => Instruction::SubtractA,
            (0xD, 0x8) => Instruction::ReturnIfCarry,
            (0xD, 0x9) => Instruction::ReturnAndEnableInterrupts,
            (0xD, 0xA) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::JumpIfCarry { address }
            }
            (0xD, 0xC) => {
                let high = memory.rom[self.program_counter as usize + 1];
                let low = memory.rom[self.program_counter as usize + 2];

                let address = ((high as u16) << 8) + low as u16;
                Instruction::CallIfCarry { address }
            }
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
                let registers = [
                    DoubleRegister::BC,
                    DoubleRegister::DE,
                    DoubleRegister::HL,
                    DoubleRegister::AF,
                ];
                if reg < 4 {
                    Instruction::LoadReg16 {
                        register: registers[reg as usize],
                    }
                } else {
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
                if location < 3 {
                    let registers = [Register::B, Register::D, Register::H];
                    Instruction::LoadReg8 {
                        register: registers[location as usize],
                    }
                } else {
                    Instruction::Reset0 { location }
                }
            }
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

    fn parse_prefix(&mut self, memory: &Memory) -> Instruction {
        let instruction = memory.rom[self.program_counter as usize + 1];

        match (get_upper_bits(instruction), get_lower_bits(instruction)) {
            (0x0, 0x6) => Instruction::RotateHLLeft,
            (0x0, 0xE) => Instruction::RotateHLRight,
            (0x0, reg) => {
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad RL/RR (HL)
                    Register::A,
                ];
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
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad RLC/RRC (HL)
                    Register::A,
                ];
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
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad SLA/SRA (HL)
                    Register::A,
                ];
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
                let registers = [
                    Register::B,
                    Register::C,
                    Register::D,
                    Register::E,
                    Register::H,
                    Register::L,
                    Register::A, //Duplicate entry to pad SWAP/SRL (HL)
                    Register::A,
                ];
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
            _ => Instruction::Invalid,
        }
    }

    pub fn execute(&mut self, instruction: Instruction, memory: &mut Memory) {
        match instruction {
            // 8-bit load instructions
            Instruction::LoadReg {
                load_from: src,
                load_into: dst,
            } => {
                let registers = [self.b, self.c, self.d, self.e, self.h, self.l, 0, self.a];
                let index = (src % 8) as usize;

                match dst {
                    Register::B => self.b = registers[index],
                    Register::C => self.c = registers[index],
                    Register::D => self.d = registers[index],
                    Register::E => self.e = registers[index],
                    Register::H => self.h = registers[index],
                    Register::L => self.l = registers[index],
                    Register::A => self.a = registers[index],
                }

                self.program_counter += 1;
            }
            Instruction::LoadReg8 { register } => {
                let data = memory.read(self.program_counter + 1);

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
                let data = memory.read(self.hl());

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
                    Register::B => memory.write(address, self.b),
                    Register::C => memory.write(address, self.c),
                    Register::D => memory.write(address, self.d),
                    Register::E => memory.write(address, self.e),
                    Register::H => memory.write(address, self.h),
                    Register::L => memory.write(address, self.l),
                    Register::A => memory.write(address, self.a),
                }

                self.program_counter += 1;
            }
            Instruction::LoadHL8 => {
                let data = memory.read(self.program_counter + 1);
                memory.write(self.hl(), data);
                self.program_counter += 2;
            }
            Instruction::LoadABC => {
                self.a = memory.read(self.bc());
                self.program_counter += 1;
            }
            Instruction::LoadADE => {
                self.a = memory.read(self.de());
                self.program_counter += 1;
            }
            Instruction::LoadAAddress => {
                let high = memory.read(self.program_counter + 1);
                let low = memory.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                self.a = memory.read(address);
                self.program_counter += 3;
            }
            Instruction::LoadBCA => {
                memory.write(self.bc(), self.a);
                self.program_counter += 1;
            }
            Instruction::LoadDEA => {
                memory.write(self.de(), self.a);
                self.program_counter += 1;
            }
            Instruction::LoadAddressA => {
                let high = memory.read(self.program_counter + 1);
                let low = memory.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                memory.write(address, self.a);
                self.program_counter += 3;
            }
            Instruction::LoadAOffset => {
                let offset = memory.read(self.program_counter + 1) as u16;
                self.a = memory.read(0xFF00 + offset);
                self.program_counter += 2;
            }
            Instruction::LoadOffsetA => {
                let offset = memory.read(self.program_counter + 1) as u16;
                memory.write(0xFF00 + offset, self.a);
                self.program_counter += 2;
            }
            Instruction::LoadAOffsetC => {
                self.a = memory.read(0xFF00 + self.c as u16);
                self.program_counter += 1;
            }
            Instruction::LoadOffsetCA => {
                memory.write(0xFF00 + self.c as u16, self.a);
                self.program_counter += 1;
            }
            Instruction::LoadIncrementHLA => {
                memory.write(self.hl(), self.a);
                self.increment_hl();
                self.program_counter += 1;
            }
            Instruction::LoadIncrementAHL => {
                self.a = memory.read(self.hl());
                self.increment_hl();
                self.program_counter += 1;
            }
            Instruction::LoadDecrementHLA => {
                memory.write(self.hl(), self.a);
                self.decrement_hl();
                self.program_counter += 1;
            }
            Instruction::LoadDecrementAHL => {
                self.a = memory.read(self.hl());
                self.decrement_hl();
                self.program_counter += 1;
            }

            // 16-bit load instructions
            Instruction::LoadReg16 { register } => {
                let upper = memory.read(self.program_counter + 1);
                let lower = memory.read(self.program_counter + 2);

                match register {
                    DoubleRegister::BC => {
                        self.b = upper;
                        self.c = lower;
                    }
                    DoubleRegister::DE => {
                        self.d = upper;
                        self.c = lower;
                    }
                    DoubleRegister::HL => {
                        self.h = upper;
                        self.l = lower;
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.program_counter += 3;
            }
            Instruction::LoadAddressSP => {
                let high = memory.read(self.program_counter + 1);
                let low = memory.read(self.program_counter + 2);
                let address = combine_bytes(high, low);

                memory.write16(address, self.stack_pointer);
                self.program_counter += 3;
            }
            Instruction::LoadSPHL => {
                self.stack_pointer = self.hl();
                self.program_counter += 1;
            }
            Instruction::PushReg { register } => {
                self.stack_pointer -= 2;

                match register {
                    DoubleRegister::BC => memory.write16(self.stack_pointer, self.bc()),
                    DoubleRegister::DE => memory.write16(self.stack_pointer, self.de()),
                    DoubleRegister::HL => memory.write16(self.stack_pointer, self.hl()),
                    DoubleRegister::AF => {
                        memory.write16(self.stack_pointer, combine_bytes(self.a, self.f))
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.program_counter += 1;
            }
            Instruction::PopReg { register } => {
                let upper = memory.read(self.stack_pointer);
                let lower = memory.read(self.stack_pointer + 1);

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
                        self.f = lower;
                    }
                    _ => panic!("Invalid Instruction"),
                }

                self.stack_pointer += 2;
                self.program_counter += 1;
            }

            // 8-bit Arithmetic/Logic instructions
            Instruction::AddAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                self.overflow_addition(value);
                self.program_counter += 1;
            }
            Instruction::AddA => {
                let value = memory.read(self.program_counter + 1);
                self.overflow_addition(value);
                self.program_counter += 2;
            }
            Instruction::AddAHL => {
                let value = memory.read(self.hl());
                self.overflow_addition(value);
                self.program_counter += 1;
            }
            Instruction::AddCarryAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                let carry = if self.is_carry() { 1 } else { 0 };
                self.overflow_addition(value + carry);
                self.program_counter += 1;
            }
            Instruction::AddCarryA => {
                let value = memory.read(self.program_counter + 1);
                let carry = if self.is_carry() { 1 } else { 0 };
                self.overflow_addition(value + carry);
                self.program_counter += 2;
            }
            Instruction::AddCarryAHL => {
                let value = memory.read(self.hl());
                let carry = if self.is_carry() { 1 } else { 0 };
                self.overflow_addition(value + carry);
                self.program_counter += 1;
            }
            Instruction::SubtractAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                self.a = self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractA => {
                let value = memory.read(self.program_counter + 1);
                self.a = self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractAHL => {
                let value = memory.read(self.hl());
                self.a = self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractARegCarry { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                let carry = if self.is_carry() { 1 } else { 0 };
                self.a = self.overflow_subtraction(value + carry);
                self.program_counter += 1;
            }
            Instruction::SubtractACarry => {
                let value = memory.read(self.program_counter + 1);
                let carry = if self.is_carry() { 1 } else { 0 };
                self.a = self.overflow_subtraction(value + carry);
                self.program_counter += 2;
            }
            Instruction::SubtractAHLCarry => {
                let value = memory.read(self.hl());
                let carry = if self.is_carry() { 1 } else { 0 };
                self.a = self.overflow_subtraction(value + carry);
                self.program_counter += 1;
            }
            Instruction::AndAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                self.a = self.a & value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(true);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::AndA => {
                let value = memory.read(self.program_counter + 1);

                self.a = self.a & value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(true);
                self.set_carry(false);

                self.program_counter += 2;
            }
            Instruction::AndAHL => {
                let value = memory.read(self.hl());

                self.a = self.a & value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(true);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::XorAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                self.a = self.a ^ value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::XorA => {
                let value = memory.read(self.program_counter + 1);

                self.a = self.a ^ value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 2;
            }
            Instruction::XorAHL => {
                let value = memory.read(self.hl());

                self.a = self.a ^ value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::OrAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                self.a = self.a | value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::OrA => {
                let value = memory.read(self.program_counter + 1);

                self.a = self.a | value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 2;
            }
            Instruction::OrAHL => {
                let value = memory.read(self.hl());

                self.a = self.a | value;

                self.set_zero(self.a == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);

                self.program_counter += 1;
            }
            Instruction::CompareAReg { register } => {
                let value: u8;
                match register {
                    Register::B => value = self.b,
                    Register::C => value = self.c,
                    Register::D => value = self.d,
                    Register::E => value = self.e,
                    Register::H => value = self.h,
                    Register::L => value = self.l,
                    Register::A => value = self.a,
                }

                _ = self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::CompareA => {
                let value = memory.read(self.program_counter + 1);
                _ = self.overflow_subtraction(value);
                self.program_counter += 2;
            }
            Instruction::CompareAHL => {
                let value = memory.read(self.hl());
                _ = self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::IncrementReg { register } => {
                let (before, after) = match register {
                    Register::B => {
                        let before = self.b;
                        let result = self.b as u16 + 1;
                        self.b = (result & 0x00FF) as u8;
                        (before, self.b)
                    }
                    Register::C => {
                        let before = self.c;
                        let result = self.c as u16 + 1;
                        self.c = (result & 0x00FF) as u8;
                        (before, self.c)
                    }
                    Register::D => {
                        let before = self.d;
                        let result = self.d as u16 + 1;
                        self.d = (result & 0x00FF) as u8;
                        (before, self.d)
                    }
                    Register::E => {
                        let before = self.e;
                        let result = self.e as u16 + 1;
                        self.e = (result & 0x00FF) as u8;
                        (before, self.e)
                    }
                    Register::H => {
                        let before = self.h;
                        let result = self.h as u16 + 1;
                        self.h = (result & 0x00FF) as u8;
                        (before, self.h)
                    }
                    Register::L => {
                        let before = self.l;
                        let result = self.l as u16 + 1;
                        self.l = (result & 0x00FF) as u8;
                        (before, self.l)
                    }
                    Register::A => {
                        let before = self.a;
                        let result = self.a as u16 + 1;
                        self.a = (result & 0x00FF) as u8;
                        (before, self.a)
                    }
                };

                self.set_zero(after == 0);
                self.set_subtraction(false);
                self.set_half_carry((before & 0x0F) + 1 > 0x0F);

                self.program_counter += 1;
            }
            Instruction::IncrementHL => {
                let data = memory.read(self.hl());
                let result = data as u16 + 1;
                let write_data = (result & 0x00FF) as u8;

                self.set_zero(write_data == 0);
                self.set_subtraction(false);
                self.set_half_carry((data & 0x0F) + 1 > 0x0F);

                memory.write(self.hl(), write_data);
                self.program_counter += 1;
            }
            Instruction::DecrementReg { register } => {
                let (before, after) = match register {
                    Register::B => {
                        let before = self.b;
                        let result = self.b as i16 - 1;
                        self.b = (result & 0x00FF) as u8;
                        (before, self.b)
                    }
                    Register::C => {
                        let before = self.c;
                        let result = self.c as i16 - 1;
                        self.c = (result & 0x00FF) as u8;
                        (before, self.c)
                    }
                    Register::D => {
                        let before = self.d;
                        let result = self.d as i16 - 1;
                        self.d = (result & 0x00FF) as u8;
                        (before, self.d)
                    }
                    Register::E => {
                        let before = self.e;
                        let result = self.e as i16 - 1;
                        self.e = (result & 0x00FF) as u8;
                        (before, self.e)
                    }
                    Register::H => {
                        let before = self.h;
                        let result = self.h as i16 - 1;
                        self.h = (result & 0x00FF) as u8;
                        (before, self.h)
                    }
                    Register::L => {
                        let before = self.l;
                        let result = self.l as i16 - 1;
                        self.l = (result & 0x00FF) as u8;
                        (before, self.l)
                    }
                    Register::A => {
                        let before = self.a;
                        let result = self.a as i16 - 1;
                        self.a = (result & 0x00FF) as u8;
                        (before, self.a)
                    }
                };

                self.set_zero(after == 0);
                self.set_subtraction(true);
                self.set_half_carry((before & 0x0F) < 1);

                self.program_counter += 1;
            }
            Instruction::DecrementHL => {
                let data = memory.read(self.hl());
                let result = data as i16 - 1;
                let write_data = (result & 0x00FF) as u8;

                self.set_zero(write_data == 0);
                self.set_subtraction(true);
                self.set_half_carry((data & 0x0F) < 1);

                memory.write(self.hl(), write_data);
                self.program_counter += 1;
            }
            Instruction::DecimalAdjustA => {
                let mut correction: u8 = 0;

                if self.is_half_carry() || (!self.is_subtraction() && (self.a & 0x0F) > 9) {
                    correction += 0x06;
                }

                if self.is_carry() || (!self.is_subtraction() && self.a > 0x99) {
                    correction += 0x60;
                    self.set_carry(true);
                }

                if self.is_subtraction() {
                    self.a -= correction;
                } else {
                    self.a += correction;
                }

                self.set_zero(self.a == 0);
                self.set_half_carry(false);
                self.program_counter += 1;
            }
            Instruction::Complement => {
                self.a ^= 0xFF;
                self.set_subtraction(true);
                self.set_half_carry(true);
                self.program_counter += 1;
            }

            // 16-bit Arithmetic/Logic instructions
            Instruction::AddHLReg { register } => {
                let value: u16;
                match register {
                    DoubleRegister::BC => value = self.bc(),
                    DoubleRegister::DE => value = self.de(),
                    DoubleRegister::HL => value = self.hl(),
                    DoubleRegister::SP => value = self.stack_pointer,
                    _ => panic!("Invalid Instruction"),
                }

                let lower_result = self.l as u16 + get_lower_byte(value) as u16;
                let lower_carry = if lower_result > 0x00FF { 1 } else { 0 };

                self.l = get_lower_byte(lower_result);

                let upper_value = get_upper_byte(value) as u16 + lower_carry;
                let upper_result = self.h as u16 + upper_value;

                self.set_subtraction(false);
                self.set_half_carry((self.h & 0x0F) + (upper_value & 0x000F) as u8 > 0x0F);
                self.set_carry(upper_result > 0x00FF);

                self.h = get_lower_byte(upper_result);
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
                let offset = memory.rom[self.program_counter as usize + 1] as i8;

                self.set_zero(false);
                self.set_subtraction(false);

                let abs_offset = offset.abs() as u16;

                if offset > 0 {
                    let result = self.stack_pointer as u32 + offset as u32;
                    self.set_half_carry(
                        (self.stack_pointer & 0x0FFF) + (abs_offset & 0x0FFF) > 0x0FFF,
                    );
                    self.set_carry(result > 0xFFFF);
                    self.stack_pointer = (result & 0x0000_FFFF) as u16;
                } else if abs_offset > self.stack_pointer {
                    self.set_half_carry((self.stack_pointer & 0x0FFF) < (abs_offset & 0x0FFF));
                    self.set_carry(true);
                    self.stack_pointer = u16::MAX - (abs_offset - self.stack_pointer);
                } else {
                    self.set_half_carry((self.stack_pointer & 0x0FFF) < (abs_offset & 0x0FFF));
                    self.set_carry(false);
                    self.stack_pointer -= abs_offset;
                }

                self.program_counter += 2;
            }
            Instruction::LoadHLSPOffset => {
                let offset = memory.rom[self.program_counter as usize + 1] as i8;

                self.set_zero(false);
                self.set_subtraction(false);

                let abs_offset = offset.abs() as u16;

                if offset > 0 {
                    let result = self.stack_pointer as u32 + offset as u32;
                    self.set_half_carry(
                        (self.stack_pointer & 0x0FFF) + (abs_offset & 0x0FFF) > 0x0FFF,
                    );
                    self.set_carry(result > 0xFFFF);
                    let result_16 = (result & 0x0000_FFFF) as u16;
                    self.h = get_upper_byte(result_16);
                    self.l = get_lower_byte(result_16);
                } else if abs_offset > self.stack_pointer {
                    self.set_half_carry((self.stack_pointer & 0x0FFF) < (abs_offset & 0x0FFF));
                    self.set_carry(true);
                    let result = u16::MAX - (abs_offset - self.stack_pointer);
                    self.h = get_upper_byte(result);
                    self.l = get_lower_byte(result);
                } else {
                    self.set_half_carry((self.stack_pointer & 0x0FFF) < (abs_offset & 0x0FFF));
                    self.set_carry(false);
                    let result = self.stack_pointer - abs_offset;
                    self.h = get_upper_byte(result);
                    self.l = get_lower_byte(result);
                }

                self.program_counter += 2;
            }

            // Rotate and Shift instructions
            Instruction::RotateALeft => {
                self.a = self.rotate_left(self.a);

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 1;
            }
            Instruction::RotateALeftThroughCarry => {
                self.a = self.rotate_left_through_carry(self.a);

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 1;
            }
            Instruction::RotateARight => {
                self.a = self.rotate_right(self.a);

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 1;
            }
            Instruction::RotateARightThroughCarry => {
                self.a = self.rotate_right_through_carry(self.a);

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 1;
            }
            Instruction::RotateLeft { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_left(self.b);
                        self.set_zero(self.b == 0);
                    }
                    Register::C => {
                        self.c = self.rotate_left(self.c);
                        self.set_zero(self.c == 0);
                    }
                    Register::D => {
                        self.d = self.rotate_left(self.d);
                        self.set_zero(self.d == 0);
                    }
                    Register::E => {
                        self.e = self.rotate_left(self.e);
                        self.set_zero(self.e == 0);
                    }
                    Register::H => {
                        self.h = self.rotate_left(self.h);
                        self.set_zero(self.h == 0);
                    }
                    Register::L => {
                        self.l = self.rotate_left(self.l);
                        self.set_zero(self.l == 0);
                    }
                    Register::A => {
                        self.a = self.rotate_left(self.a);
                        self.set_zero(self.a == 0);
                    }
                }

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateHLLeft => {
                let data = memory.read(self.hl());
                let result = self.rotate_left(data);
                memory.write(self.hl(), result);

                self.set_zero(result == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateLeftThroughCarry { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_left_through_carry(self.b);
                        self.set_zero(self.b == 0);
                    }
                    Register::C => {
                        self.c = self.rotate_left_through_carry(self.c);
                        self.set_zero(self.c == 0);
                    }
                    Register::D => {
                        self.d = self.rotate_left_through_carry(self.d);
                        self.set_zero(self.d == 0);
                    }
                    Register::E => {
                        self.e = self.rotate_left_through_carry(self.e);
                        self.set_zero(self.e == 0);
                    }
                    Register::H => {
                        self.h = self.rotate_left_through_carry(self.h);
                        self.set_zero(self.h == 0);
                    }
                    Register::L => {
                        self.l = self.rotate_left_through_carry(self.l);
                        self.set_zero(self.l == 0);
                    }
                    Register::A => {
                        self.a = self.rotate_left_through_carry(self.a);
                        self.set_zero(self.a == 0);
                    }
                }

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateHLLeftThroughCarry => {
                let data = memory.read(self.hl());
                let result = self.rotate_left_through_carry(data);
                memory.write(self.hl(), result);

                self.set_zero(result == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateRight { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_right(self.b);
                        self.set_zero(self.b == 0);
                    }
                    Register::C => {
                        self.c = self.rotate_right(self.c);
                        self.set_zero(self.c == 0);
                    }
                    Register::D => {
                        self.d = self.rotate_right(self.d);
                        self.set_zero(self.d == 0);
                    }
                    Register::E => {
                        self.e = self.rotate_right(self.e);
                        self.set_zero(self.e == 0);
                    }
                    Register::H => {
                        self.h = self.rotate_right(self.h);
                        self.set_zero(self.h == 0);
                    }
                    Register::L => {
                        self.l = self.rotate_right(self.l);
                        self.set_zero(self.l == 0);
                    }
                    Register::A => {
                        self.a = self.rotate_right(self.a);
                        self.set_zero(self.a == 0);
                    }
                }

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateHLRight => {
                let data = memory.read(self.hl());
                let result = self.rotate_right(data);
                memory.write(self.hl(), result);

                self.set_zero(result == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateRightThroughCarry { register } => {
                match register {
                    Register::B => {
                        self.b = self.rotate_right_through_carry(self.b);
                        self.set_zero(self.b == 0);
                    }
                    Register::C => {
                        self.c = self.rotate_right_through_carry(self.c);
                        self.set_zero(self.c == 0);
                    }
                    Register::D => {
                        self.d = self.rotate_right_through_carry(self.d);
                        self.set_zero(self.d == 0);
                    }
                    Register::E => {
                        self.e = self.rotate_right_through_carry(self.e);
                        self.set_zero(self.e == 0);
                    }
                    Register::H => {
                        self.h = self.rotate_right_through_carry(self.h);
                        self.set_zero(self.h == 0);
                    }
                    Register::L => {
                        self.l = self.rotate_right_through_carry(self.l);
                        self.set_zero(self.l == 0);
                    }
                    Register::A => {
                        self.a = self.rotate_right_through_carry(self.a);
                        self.set_zero(self.a == 0);
                    }
                }

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::RotateHLRightThroughCarry => {
                let data = memory.read(self.hl());
                let result = self.rotate_right_through_carry(data);
                memory.write(self.hl(), result);

                self.set_zero(result == 0);
                self.set_subtraction(false);
                self.set_half_carry(false);

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

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::ShiftHLLeftArithmetic => {
                let data = memory.read(self.hl());
                memory.write(self.hl(), self.shift_left(data));

                self.set_subtraction(false);
                self.set_half_carry(false);

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

                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);
                self.program_counter += 2;
            }
            Instruction::SwapHL => {
                let data = memory.read(self.hl());
                memory.write(self.hl(), self.swap(data));

                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(false);
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

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::ShiftHLRightArithmetic => {
                let data = memory.read(self.hl());
                memory.write(self.hl(), self.shift_right_arithmetic(data));

                self.set_subtraction(false);
                self.set_half_carry(false);

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

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            Instruction::ShiftHLRightLogical => {
                let data = memory.read(self.hl());
                memory.write(self.hl(), self.shift_right_logical(data));

                self.set_subtraction(false);
                self.set_half_carry(false);

                self.program_counter += 2;
            }
            
            // Single-bit operation instructions
            //-----------------------------
            Instruction::Invalid => todo!(),
            Instruction::Nop => {
                self.program_counter += 1;
            }
            Instruction::Stop => todo!(),
            Instruction::JumpRelative { flag } => {
                let offset = memory.rom[self.program_counter as usize + 1] as i8;

                let predicate = match flag {
                    ConditionalFlag::None => true,
                    ConditionalFlag::NZ => !self.is_zero(),
                    ConditionalFlag::Z => self.is_zero(),
                    ConditionalFlag::NC => !self.is_carry(),
                    ConditionalFlag::C => self.is_carry(),
                };

                if predicate {
                    if offset > 0 {
                        self.program_counter += offset as u16;
                    } else {
                        self.program_counter -= offset.abs() as u16;
                    }
                } else {
                    self.program_counter += 1;
                }
            }

            //TODO
            Instruction::Scf => todo!(),
            Instruction::Ccf => todo!(),
            Instruction::Halt => todo!(),
            Instruction::ReturnIfNotZero => todo!(),
            Instruction::JumpIfNotZero { address } => todo!(),
            Instruction::Jump { address } => todo!(),
            Instruction::CallIfNotZero { address } => todo!(),
            Instruction::ReturnIfZero => todo!(),
            Instruction::Return => todo!(),
            Instruction::JumpIfZero { address } => todo!(),
            Instruction::CallIfZero { address } => todo!(),
            Instruction::Call { address } => todo!(),
            Instruction::ReturnIfNotCarry => todo!(),
            Instruction::JumpIfNotCarry { address } => todo!(),
            Instruction::CallIfNotCarry { address } => todo!(),
            Instruction::ReturnIfCarry => todo!(),
            Instruction::ReturnAndEnableInterrupts => todo!(),
            Instruction::JumpIfCarry { address } => todo!(),
            Instruction::CallIfCarry { address } => todo!(),
            Instruction::JumpHL => todo!(),
            Instruction::DisableInterrupts => todo!(),
            Instruction::EnableInterrupts => todo!(),
            Instruction::Reset0 { location } => todo!(),
            Instruction::Reset8 { location } => todo!(),
        }
    }

    /// Adds `value` to the `A` register and sets the appropriate flags (z0hc)
    fn overflow_addition(&mut self, value: u8) {
        let mut result = self.a as u16;

        result += value as u16;

        self.set_zero(result & 0x00FF == 0);
        self.set_subtraction(false);
        self.set_half_carry((self.a & 0x0F) + (value & 0x0F) > 0x0F);
        self.set_carry(result > u8::MAX as u16);

        self.a = get_lower_byte(result);
    }

    /// Returns the result of subtracting `value` from the `A` register and sets the appropriate flags (z1hc)
    fn overflow_subtraction(&mut self, value: u8) -> u8 {
        let mut result = self.a as i16;

        result -= value as i16;

        self.set_zero(self.a == value);
        self.set_subtraction(true);
        self.set_half_carry((self.a & 0x0F) < (value & 0x0F));
        self.set_carry(value > self.a);

        (result & 0x00FF) as u8
    }

    /// Returns the left rotated value and sets the carry flag
    fn rotate_left(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = msb >> 7;
        let result = (value << 1) + carry;

        self.set_carry(carry == 1);

        result
    }

    /// Returns the left rotated through carry value and sets the carry flag
    fn rotate_left_through_carry(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = if self.is_carry() { 1 } else { 0 };
        let result = (value << 1) + carry;

        self.set_carry((msb >> 7) == 1);

        result
    }

    /// Returns the right rotated value and sets the carry flag
    fn rotate_right(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let result = (value >> 1) + (lsb << 7);

        self.set_carry(lsb == 1);

        result
    }

    /// Returns the right rotated through carry value and sets the carry flag
    fn rotate_right_through_carry(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let carry = if self.is_carry() { 0b1000_0000 } else { 0 };
        let result = (value >> 1) + carry;

        self.set_carry(lsb == 1);

        result
    }

    /// Returns the left shifted value and sets the zero and carry flags
    fn shift_left(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let carry = msb >> 7;
        let result = value << 1;

        self.set_zero(result == 0);
        self.set_carry(carry == 1);

        result
    }

    /// Swap the upper and lower nibble of value and set the zero flag
    fn swap(&mut self, value: u8) -> u8 {
        let upper = get_upper_bits(value);
        let lower = get_lower_bits(value);
        let result = (lower << 4) + upper;

        self.set_zero(result == 0);

        result
    }

    /// Returns the arithmeticly right shifted value and sets the zero and carry flags
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value & 0b1000_0000;
        let lsb = value & 0b0000_0001;
        let result = value >> 1 | msb;

        self.set_zero(result == 0);
        self.set_carry(lsb == 1);

        result
    }

    /// Returns the logically right shifted value and sets the zero and carry flags
    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let lsb = value & 0b0000_0001;
        let result = value >> 1;

        self.set_zero(result == 0);
        self.set_carry(lsb == 1);

        result
    }
}

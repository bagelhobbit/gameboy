use crate::{
    instructions::{ConditionalFlag, DoubleRegister, Instruction, Register},
    memory::Memory,
    util::*,
};

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

    pub fn hl(&self) -> u16 {
        combine_bytes(self.h, self.l)
    }

    fn increment_hl(&mut self) {
        if self.l == u8::MAX {
            self.l = 0;
            self.h += 1;
        } else {
            self.l += 1;
        }
    }

    fn decrement_hl(&mut self) {
        if self.l == u8::MIN {
            self.l = u8::MAX;
            self.h -= 1;
        } else {
            self.l -= 1;
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
            (0x2, 0xF) => Instruction::Cpl,
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
                if reg <= 7 {
                    Instruction::AndAReg { reg }
                } else {
                    Instruction::XorAReg { reg }
                }
            }
            (0xB, 0x6) => Instruction::OrAHL,
            (0xB, 0xE) => Instruction::CompareAHL,
            (0xB, reg) => {
                if reg <= 7 {
                    Instruction::OrAReg { reg }
                } else {
                    Instruction::CompareAReg { reg }
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
            (0xC, 0xB) => Instruction::Prefix { op: 0 },
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
            (0xE, 0x6) => {
                let data = memory.rom[self.program_counter as usize + 1];
                Instruction::AndA { data }
            }
            (0xE, 0x8) => {
                let offset = memory.rom[self.program_counter as usize + 1] as i8;
                Instruction::AddSPOffset { offset }
            }
            (0xE, 0x9) => Instruction::JumpHL,
            (0xE, 0xA) => Instruction::LoadAddressA,
            (0xE, 0xE) => Instruction::XorA,
            (0xF, 0x0) => Instruction::LoadAOffset,
            (0xF, 0x2) => Instruction::LoadAOffsetC,
            (0xF, 0x3) => Instruction::DisableInterrupts,
            (0xF, 0x6) => {
                let data = memory.rom[self.program_counter as usize + 1];
                Instruction::OrA { data }
            }
            (0xF, 0x8) => {
                let offset = memory.rom[self.program_counter as usize + 1] as i8;
                Instruction::LoadHLSPOffset { offset }
            }
            (0xF, 0x9) => Instruction::LoadSPHL,
            (0xF, 0xA) => Instruction::LoadAAddress,
            (0xF, 0xB) => Instruction::EnableInterrupts,
            (0xF, 0xE) => {
                let data = memory.rom[self.program_counter as usize + 1];
                Instruction::CompareA { data }
            }
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
            (reg, 0x3) => Instruction::IncrementReg16 { reg },
            (reg, 0x4) => Instruction::IncrementHighReg { reg },
            (reg, 0x5) => {
                if reg < 4 {
                    Instruction::DecrementHighReg { reg }
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
            (reg, 0x9) => Instruction::AddHLReg { reg },
            (reg, 0xB) => Instruction::DecrementReg16 { reg },
            (reg, 0xC) => Instruction::IncrementLowReg { reg },
            (reg, 0xD) => Instruction::DecrementLowReg { reg },
            (reg, 0xE) => {
                let registers = [Register::C, Register::E, Register::L, Register::A];
                Instruction::LoadReg8 {
                    register: registers[(reg % 4) as usize],
                }
            }
            (location, 0xF) => Instruction::Reset8 { location },
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
                let address = combine_bytes(self.b, self.c);
                self.a = memory.read(address);
                self.program_counter += 1;
            }
            Instruction::LoadADE => {
                let address = combine_bytes(self.d, self.e);
                self.a = memory.read(address);
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
                let bc = combine_bytes(self.b, self.c);
                memory.write(bc, self.a);
                self.program_counter += 1;
            }
            Instruction::LoadDEA => {
                let de = combine_bytes(self.d, self.e);
                memory.write(de, self.a);
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
                    DoubleRegister::AF => todo!(),
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
                    DoubleRegister::BC => {
                        memory.write16(self.stack_pointer, combine_bytes(self.b, self.c))
                    }
                    DoubleRegister::DE => {
                        memory.write16(self.stack_pointer, combine_bytes(self.d, self.e))
                    }
                    DoubleRegister::HL => memory.write16(self.stack_pointer, self.hl()),
                    DoubleRegister::AF => {
                        memory.write16(self.stack_pointer, combine_bytes(self.a, self.f))
                    }
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

                self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractA => {
                let value = memory.read(self.program_counter + 1);
                self.overflow_subtraction(value);
                self.program_counter += 1;
            }
            Instruction::SubtractAHL => {
                let value = memory.read(self.hl());
                self.overflow_subtraction(value);
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
                self.overflow_subtraction(value + carry);
                self.program_counter += 1;
            }
            Instruction::SubtractACarry => {
                let value = memory.read(self.program_counter + 1);
                let carry = if self.is_carry() { 1 } else { 0 };
                self.overflow_subtraction(value + carry);
                self.program_counter += 2;
            }
            Instruction::SubtractAHLCarry => {
                let value = memory.read(self.hl());
                let carry = if self.is_carry() { 1 } else { 0 };
                self.overflow_subtraction(value + carry);
                self.program_counter += 1;
            }
            //-----------------------------
            Instruction::Invalid => todo!(),
            Instruction::Nop => {
                self.program_counter += 1;
            }
            Instruction::RotateALeft => {
                let msb = self.a & 0b1000_0000;
                let carry = msb >> 7;
                self.a = (self.a << 1) + carry;

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(carry == 1);

                self.program_counter += 1;
            }
            Instruction::RotateALeftThroughCarry => {
                let msb = self.a & 0b1000_0000;
                let carry = if self.is_carry() { 1 } else { 0 };
                self.a = (self.a << 1) + carry;

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry((msb >> 7) == 1);

                self.program_counter += 1;
            }
            Instruction::RotateARight => {
                let lsb = self.a & 0b0000_0001;
                self.a = (self.a >> 1) + (lsb << 7);

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(lsb == 1);

                self.program_counter += 1;
            }
            Instruction::RotateARightThroughCarry => {
                let lsb = self.a & 0b0000_0001;
                let carry = if self.is_carry() { 0b1000_0000 } else { 0 };
                self.a = (self.a >> 1) + carry;

                self.set_zero(false);
                self.set_subtraction(false);
                self.set_half_carry(false);
                self.set_carry(lsb == 1);

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
            Instruction::DecimalAdjustA => todo!(),
            Instruction::Cpl => todo!(),
            Instruction::IncrementHL => todo!(),
            Instruction::DecrementHL => todo!(),
            Instruction::Scf => todo!(),
            Instruction::Ccf => todo!(),
            Instruction::Halt => todo!(),
            Instruction::AndAHL => todo!(),
            Instruction::XorAHL => todo!(),
            Instruction::AndAReg { reg } => todo!(),
            Instruction::XorAReg { reg } => todo!(),
            Instruction::OrAHL => todo!(),
            Instruction::CompareAHL => todo!(),
            Instruction::OrAReg { reg } => todo!(),
            Instruction::CompareAReg { reg } => todo!(),
            Instruction::ReturnIfNotZero => todo!(),
            Instruction::JumpIfNotZero { address } => todo!(),
            Instruction::Jump { address } => todo!(),
            Instruction::CallIfNotZero { address } => todo!(),
            Instruction::ReturnIfZero => todo!(),
            Instruction::Return => todo!(),
            Instruction::Prefix { op } => todo!(),
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
            Instruction::AndA { data } => todo!(),
            Instruction::AddSPOffset { offset } => todo!(),
            Instruction::JumpHL => todo!(),
            Instruction::XorA => todo!(),
            Instruction::DisableInterrupts => todo!(),
            Instruction::OrA { data } => todo!(),
            Instruction::LoadHLSPOffset { offset } => todo!(),
            Instruction::EnableInterrupts => todo!(),
            Instruction::CompareA { data } => todo!(),
            Instruction::IncrementReg16 { reg } => todo!(),
            Instruction::IncrementHighReg { reg } => todo!(),
            Instruction::DecrementHighReg { reg } => todo!(),
            Instruction::Reset0 { location } => todo!(),
            Instruction::AddHLReg { reg } => todo!(),
            Instruction::DecrementReg16 { reg } => todo!(),
            Instruction::IncrementLowReg { reg } => todo!(),
            Instruction::DecrementLowReg { reg } => todo!(),
            Instruction::Reset8 { location } => todo!(),
        }
    }

    /// Adds `value` to the `A` register and sets the appropriate flags (z0hc)
    fn overflow_addition(&mut self, value: u8) {
        let mut result = self.a as u16;

        result += value as u16;

        self.set_subtraction(false);
        self.set_zero(result & 0x00FF == 0);
        self.set_half_carry((self.a & 0x0F) + (value & 0x0F) > 0x0F);
        self.set_carry(result > u8::MAX as u16);

        self.a = get_lower_byte(result);
    }

    /// Subtracts `value` from the `A` register and sets the appropriate flags (z1hc)
    fn overflow_subtraction(&mut self, value: u8) {
        let mut result = self.a as i16;

        result -= value as i16;

        self.set_subtraction(true);
        self.set_zero(result & 0x00FF == 0);
        self.set_half_carry((self.a & 0x0F) < (value & 0x0F));
        self.set_carry(value > self.a);

        self.a = (result & 0x00FF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(memory.rom[combine_bytes(cpu.b, cpu.c) as usize], cpu.a);
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

        assert_eq!(memory.rom[combine_bytes(cpu.d, cpu.e) as usize], cpu.a);
        assert_eq!(cpu.program_counter, 1);
    }

    #[test]
    fn test_load_address_a() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();

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

        memory.rom[1] = 0xEE;
        memory.rom[2] = 0x55;

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
        cpu.overflow_subtraction(10);

        assert_eq!(cpu.a, 5);
        assert_eq!(cpu.is_zero(), false);
        assert_eq!(cpu.is_subtraction(), true);
        assert_eq!(cpu.is_half_carry(), false);
        assert_eq!(cpu.is_carry(), false);
    }

    #[test]
    fn test_overflow_subtraction_zero() {
        let mut cpu = Cpu::new();

        cpu.a = 0;
        cpu.overflow_subtraction(0);

        assert_eq!(cpu.a, 0);
        assert_eq!(cpu.is_zero(), true);
        assert_eq!(cpu.is_subtraction(), true);
        assert_eq!(cpu.is_half_carry(), false);
        assert_eq!(cpu.is_carry(), false);
    }

    #[test]
    fn test_overflow_subtraction_half_carry() {
        let mut cpu = Cpu::new();

        cpu.a = 0b0001_0000;
        cpu.overflow_subtraction(1);

        assert_eq!(cpu.a, 0b0000_1111);
        assert_eq!(cpu.is_zero(), false);
        assert_eq!(cpu.is_subtraction(), true);
        assert_eq!(cpu.is_half_carry(), true);
        assert_eq!(cpu.is_carry(), false);
    }

    #[test]
    fn test_overflow_subtraction_carry() {
        let mut cpu = Cpu::new();

        cpu.a = 0;
        cpu.overflow_subtraction(10);

        assert_eq!(cpu.a, 246);
        assert_eq!(cpu.is_zero(), false);
        assert_eq!(cpu.is_subtraction(), true);
        assert_eq!(cpu.is_half_carry(), true);
        assert_eq!(cpu.is_carry(), true);
    }

    //------------------------------------
    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();

        cpu.execute(Instruction::Nop, &mut memory);

        assert_eq!(cpu.program_counter, 1);
    }

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
    fn test_jr() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();

        let pc = cpu.program_counter;
        let flag = ConditionalFlag::None;
        memory.rom[1] = 25;
        // -20 as a u8, should be equal to 236
        memory.rom[1 + 25] = 0b1110_1100;

        cpu.execute(Instruction::JumpRelative { flag }, &mut memory);
        assert_eq!(cpu.program_counter, pc + 25);

        let pc = cpu.program_counter;
        cpu.execute(Instruction::JumpRelative { flag }, &mut memory);
        assert_eq!(cpu.program_counter, pc - 20);
    }

    #[test]
    fn test_jr_flags() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();

        let pc = cpu.program_counter;
        let flag = ConditionalFlag::NZ;
        memory.rom[1] = 25;
        memory.rom[1 + 25] = 25;

        cpu.execute(Instruction::JumpRelative { flag }, &mut memory);
        assert_eq!(cpu.program_counter, pc + 25);

        let pc = cpu.program_counter;
        cpu.set_zero(true);
        cpu.execute(Instruction::JumpRelative { flag }, &mut memory);
        assert_eq!(cpu.program_counter, pc + 1);
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
}

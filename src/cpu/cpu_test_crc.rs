#![cfg(test)]

use super::{crc_fast::CrcFast, Cpu, CpuBus};

impl CpuHarness for Cpu {
    fn get_reg(&self, n: CpuRegister) -> u8 {
        match n {
            CpuRegister::A => self.a,
            CpuRegister::F => self.flags_to_byte(),
            CpuRegister::B => self.b,
            CpuRegister::C => self.c,
            CpuRegister::D => self.d,
            CpuRegister::E => self.e,
            CpuRegister::H => self.h,
            CpuRegister::L => self.l,
        }
    }

    fn set_reg(&mut self, n: CpuRegister, val: u8) {
        match n {
            CpuRegister::A => self.a = val,
            CpuRegister::F => self.byte_to_flags(val),
            CpuRegister::B => self.b = val,
            CpuRegister::C => self.c = val,
            CpuRegister::D => self.d = val,
            CpuRegister::E => self.e = val,
            CpuRegister::H => self.h = val,
            CpuRegister::L => self.l = val,
        }
    }

    fn run(&mut self, code: &[u8]) {
        let mut bus = TestBus(code);
        self.program_counter = 0;
        let instruction = self.parse(&mut bus);
        self.execute(instruction, &mut bus);
    }
}

pub trait CpuHarness {
    fn get_reg(&self, n: CpuRegister) -> u8;
    fn set_reg(&mut self, n: CpuRegister, val: u8);
    fn run(&mut self, code: &[u8]);
}

pub enum CpuRegister {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct TestBus<'a>(&'a [u8]);

impl<'a> CpuBus for TestBus<'a> {
    fn read(&mut self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, _: u16, _: u8) {
        todo!()
    }
}

#[test]
fn cpu_instr_05_op_rp() {
    #[rustfmt::skip]
    const INSTRS: [([u8; 3], u32); 9] = [
        ([0x0B, 0, 0], 0xa336a1c0), // DEC  BC
        ([0x1B, 0, 0], 0x2bb815be), // DEC  DE
        ([0x2B, 0, 0], 0xc2c6939f), // DEC  HL
        ([0x03, 0, 0], 0x8107c086), // INC  BC
        ([0x13, 0, 0], 0x3835750f), // INC  DE
        ([0x23, 0, 0], 0x1b0ac76b), // INC  HL
        ([0x09, 0, 0], 0x424b6806), // ADD  HL,BC
        ([0x19, 0, 0], 0x188cb464), // ADD  HL,DE
        ([0x29, 0, 0], 0x94316cfb), // ADD  HL,HL
    ];

    #[rustfmt::skip]
    const VALUES: [u16; 17] = [
        0x0000, 0x0001, 0x000f,
        0x0010, 0x001f, 0x007f,
        0x0080, 0x00ff, 0x0100,
        0x0f00, 0x1f00, 0x1000,
        0x7fff, 0x8000, 0xffff,
        0x0000, 0x0001, // Duplicate values to allow `a` to wrap.
    ];

    for (instr, expected) in INSTRS.iter() {
        let mut crc = CrcFast::new();
        let mut cpu = Cpu::new();
        cpu.interrupts_enabled = false;

        test_instr(&mut crc, &mut cpu, instr);

        assert_eq!(
            *expected,
            crc.val(),
            "Instruction 0x{:0>2x} failed",
            instr[0]
        );
    }

    fn test_instr(crc: &mut CrcFast, cpu: &mut impl CpuHarness, code: &[u8; 3]) {
        test(crc, cpu, code, 0x00);
        test(crc, cpu, code, 0x10);
        test(crc, cpu, code, 0xe0);
        test(crc, cpu, code, 0xf0);
    }

    fn test(crc: &mut CrcFast, cpu: &mut impl CpuHarness, code: &[u8; 3], f: u8) {
        for a in 0..(VALUES.len() - 2) {
            let hl = VALUES[a];

            for b in 0..(VALUES.len() - 2) {
                run_code(cpu, &VALUES[b..], code, f, hl);

                compute_checksum(crc, cpu);
            }
        }
    }

    fn run_code(cpu: &mut impl CpuHarness, values: &[u16], code: &[u8], f: u8, hl: u16) {
        // BC
        cpu.set_reg(CpuRegister::C, (values[0] >> 0) as u8);
        cpu.set_reg(CpuRegister::B, (values[0] >> 8) as u8);

        // DE
        cpu.set_reg(CpuRegister::E, (values[1] >> 0) as u8);
        cpu.set_reg(CpuRegister::D, (values[1] >> 8) as u8);

        // AF
        cpu.set_reg(CpuRegister::A, (values[2] >> 0) as u8);
        cpu.set_reg(CpuRegister::F, f);

        // HL
        cpu.set_reg(CpuRegister::L, (hl >> 0) as u8);
        cpu.set_reg(CpuRegister::H, (hl >> 8) as u8);

        cpu.run(code);
    }
}

fn compute_checksum(crc: &mut CrcFast, cpu: &impl CpuHarness) {
    crc.add(cpu.get_reg(CpuRegister::A)); // AF
    crc.add(cpu.get_reg(CpuRegister::F));

    crc.add(cpu.get_reg(CpuRegister::B)); // BC
    crc.add(cpu.get_reg(CpuRegister::C));

    crc.add(cpu.get_reg(CpuRegister::D)); // DE
    crc.add(cpu.get_reg(CpuRegister::E));

    crc.add(cpu.get_reg(CpuRegister::H)); // HL
    crc.add(cpu.get_reg(CpuRegister::L));
}

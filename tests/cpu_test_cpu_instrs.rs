use gameboy::cpu::{Cpu, CpuBus};
use smolder_tests::gb::{cpu_instrs, mem::Mem, CpuReg16, CpuReg8, CpuTestHarness};

#[test]
fn cpu_insts() {
    let mut cpu = TestCpu(Cpu::new());
    cpu.0.interrupts_enabled = false;
    cpu_instrs::test_01::special(&mut cpu);
    cpu_instrs::test_03::op_sp_hl(&mut cpu);
    cpu_instrs::test_04::op_r_imm(&mut cpu);
    cpu_instrs::test_05::op_rp(&mut cpu);
    cpu_instrs::test_06::ld_r_r(&mut cpu);
    cpu_instrs::test_08::misc_instrs(&mut cpu);
    cpu_instrs::test_09::op_r_r(&mut cpu);
    cpu_instrs::test_10::bit_ops(&mut cpu);
    cpu_instrs::test_11::op_a_hl(&mut cpu);
}

struct TestCpuBus<'a>(&'a mut Mem);

impl<'a> CpuBus for TestCpuBus<'a> {
    fn read(&mut self, address: u16) -> u8 {
        self.0.get(address)
    }

    fn write(&mut self, address: u16, val: u8) {
        self.0.set(address, val)
    }
}

struct TestCpu(Cpu);

impl CpuTestHarness for TestCpu {
    fn get_reg_8(&self, n: CpuReg8) -> u8 {
        match n {
            CpuReg8::A => self.0.a,
            CpuReg8::F => self.0.flags_to_byte(),
            CpuReg8::B => self.0.b,
            CpuReg8::C => self.0.c,
            CpuReg8::D => self.0.d,
            CpuReg8::E => self.0.e,
            CpuReg8::H => self.0.h,
            CpuReg8::L => self.0.l,
        }
    }

    fn set_reg_8(&mut self, n: CpuReg8, val: u8) {
        match n {
            CpuReg8::A => self.0.a = val,
            CpuReg8::F => self.0.byte_to_flags(val),
            CpuReg8::B => self.0.b = val,
            CpuReg8::C => self.0.c = val,
            CpuReg8::D => self.0.d = val,
            CpuReg8::E => self.0.e = val,
            CpuReg8::H => self.0.h = val,
            CpuReg8::L => self.0.l = val,
        }
    }

    fn get_reg_16(&self, n: CpuReg16) -> u16 {
        match n {
            CpuReg16::PC => self.0.program_counter,
            CpuReg16::SP => self.0.stack_pointer,
        }
    }

    fn set_reg_16(&mut self, n: CpuReg16, val: u16) {
        match n {
            CpuReg16::PC => self.0.program_counter = val,
            CpuReg16::SP => self.0.stack_pointer = val,
        }
    }

    fn run(&mut self, mem: &mut Mem) {
        let mut mem = TestCpuBus(mem);
        let instruction = self.0.parse(&mut mem);
        self.0.execute(instruction, &mut mem);
    }
}

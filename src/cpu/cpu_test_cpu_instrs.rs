#![cfg(test)]

use super::{Cpu, CpuBus};
use smolder_tests::gb::{cpu_instrs, CpuRegister, CpuTestHarness};
use std::collections::HashMap;

#[test]
fn cpu_insts() {
    let mut cpu = SmolderCpuHarness::default();
    cpu_instrs::test_all(&mut cpu);
}

#[derive(Default)]
struct TestCpuBus(HashMap<u16, u8>);

impl CpuBus for TestCpuBus {
    fn read(&mut self, address: u16) -> u8 {
        *self
            .0
            .get(&address)
            .expect("read from uninitialized memory")
    }

    fn write(&mut self, address: u16, val: u8) {
        self.0.insert(address, val);
    }
}

#[derive(Default)]
struct SmolderCpuHarness {
    cpu: Cpu,
    bus: TestCpuBus,
}

impl CpuTestHarness for SmolderCpuHarness {
    fn get_mem(&mut self, address: u16) -> u8 {
        self.bus.read(address)
    }

    fn set_mem(&mut self, address: u16, val: u8) {
        self.bus.write(address, val)
    }

    fn get_reg(&self, n: CpuRegister) -> u8 {
        match n {
            CpuRegister::A => self.cpu.a,
            CpuRegister::F => self.cpu.flags_to_byte(),
            CpuRegister::B => self.cpu.b,
            CpuRegister::C => self.cpu.c,
            CpuRegister::D => self.cpu.d,
            CpuRegister::E => self.cpu.e,
            CpuRegister::H => self.cpu.h,
            CpuRegister::L => self.cpu.l,
        }
    }

    fn set_reg(&mut self, n: CpuRegister, val: u8) {
        match n {
            CpuRegister::A => self.cpu.a = val,
            CpuRegister::F => self.cpu.byte_to_flags(val),
            CpuRegister::B => self.cpu.b = val,
            CpuRegister::C => self.cpu.c = val,
            CpuRegister::D => self.cpu.d = val,
            CpuRegister::E => self.cpu.e = val,
            CpuRegister::H => self.cpu.h = val,
            CpuRegister::L => self.cpu.l = val,
        }
    }

    fn run(&mut self) {
        self.cpu.program_counter = 0;
        let instruction = self.cpu.parse(&mut self.bus);
        self.cpu.execute(instruction, &mut self.bus);
    }
}

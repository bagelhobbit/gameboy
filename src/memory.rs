#[derive(Debug)]
pub struct Memory {
    /// 16 KiB ROM Bank 00
    /// * From cartridge, usually a fixed bank
    /// * Addressed from `0x0000` to `0x3FFF`
    pub rom: [u8; 0x3FFF],
    // 16 KiB ROM Bank 01 ~ NN
    // * From cartridge, switchable bank via mapper (if any)
    // * Addressed from `0x4000` to `0x7FFF`
    //pub switchable_rom: Option<[u8; 0x3FFF]>
    /// 8 KiB Video RAM (VRAM)
    /// * Addressed from `0x8000` to `0x9FFF`
    pub vram: [u8; 0x1FFF],
    /// 8 KiB External RAM
    /// * From cartridge, switchable bank if any
    /// * Addressed from `0xA000` to `0xBFFF`
    pub ram: [u8; 0x1FFF],
    /// 8 KiB Work RAM (WRAM)
    /// * Addressed from `0xC000` to `0xDFFF`
    // In Color GameBoy (CGB) mode, the second half (0xD000 - 0xDFFF) of this block is a switchable bank
    pub wram: [u8; 0x1FFF],
    // Mirror of C000~DDFF, Nintendo says use of this area is prohibited
    // * Addressed from `0xE000` to `0xFDFF`
    // echo_ram: [u8; 0x1DFF],
    /// Sprite Attribute Table
    /// * also Object Attribute Memory (OAM)
    /// * Addressed from `0xFE00` to `0xFE9F`
    pub sprite_attribute_table: [u8; 0x9F],
    /// I/O Registers
    /// * Addressed from `0xFF00` to `0xFF7F`
    pub io_registers: [u8; 0x7F],
    /// High RAM (HRAM)
    /// * Addressed from `0xFF80` to `0xFFFE`
    pub hram: [u8; 0x7E],
    /// Interrupt Enable Register
    /// * Addressed at `0xFFFF`
    pub interrupt_enable_register: bool,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [0; 0x3FFF],
            vram: [0; 0x1FFF],
            ram: [0; 0x1FFF],
            wram: [0; 0x1FFF],
            sprite_attribute_table: [0; 0x9F],
            io_registers: [0; 0x7F],
            hram: [0; 0x7E],
            interrupt_enable_register: true,
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        if address <= 0x3FFF {
            self.rom[address as usize]
        } else if address <= 0x7FFF {
            todo!()
        } else if address <= 0x9FFF {
            let mapped = address - 0x8000;
            self.vram[mapped as usize]
        } else if address <= 0xBFFF {
            let mapped = address - 0xA000;
            self.ram[mapped as usize]
        } else if address <= 0xDFFF {
            let mapped = address - 0xC000;
            self.wram[mapped as usize]
        } else if address <= 0xFDFF {
            //echo ram
            todo!()
        } else if address <= 0xFE9F {
            let mapped = address - 0xFE00;
            self.sprite_attribute_table[mapped as usize]
        } else if address <= 0xFEFF {
            //prohibited
            todo!()
        } else if address <= 0xFF7F {
            let mapped = address - 0xFF00;
            self.io_registers[mapped as usize]
        } else if address <= 0xFFFE {
            let mapped = address - 0xFF80;
            self.hram[mapped as usize]
        } else {
            if self.interrupt_enable_register {
                1
            } else {
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address <= 0x3FFF {
            self.rom[address as usize] = data;
        } else if address <= 0x7FFF {
            todo!()
        } else if address <= 0x9FFF {
            let mapped = address - 0x8000;
            self.vram[mapped as usize] = data;
        } else if address <= 0xBFFF {
            let mapped = address - 0xA000;
            self.ram[mapped as usize] = data;
        } else if address <= 0xDFFF {
            let mapped = address - 0xC000;
            self.wram[mapped as usize] = data;
        } else if address <= 0xFDFF {
            //echo ram
            todo!()
        } else if address <= 0xFE9F {
            let mapped = address - 0xFE00;
            self.sprite_attribute_table[mapped as usize] = data;
        } else if address <= 0xFEFF {
            //prohibited
            todo!()
        } else if address <= 0xFF7F {
            let mapped = address - 0xFF00;
            self.io_registers[mapped as usize] = data;
        } else if address <= 0xFFFE {
            let mapped = address - 0xFF80;
            self.hram[mapped as usize] = data;
        } else {
            if data == 0 {
                self.interrupt_enable_register = false;
            } else {
                self.interrupt_enable_register = true;
            }
        }
    }

    pub fn write16(&mut self, address: u16, data: u16) {
        if address <= 0x3FFF {
            self.rom[address as usize] = get_upper_byte(data);
            self.rom[address as usize + 1] = get_lower_byte(data);
        } else if address <= 0x7FFF {
            todo!()
        } else if address <= 0x9FFF {
            let mapped = address - 0x8000;
            self.vram[mapped as usize] = get_upper_byte(data);
            self.vram[mapped as usize + 1] = get_lower_byte(data);
        } else if address <= 0xBFFF {
            let mapped = address - 0xA000;
            self.ram[mapped as usize] = get_upper_byte(data);
            self.ram[mapped as usize + 1] = get_lower_byte(data);
        } else if address <= 0xDFFF {
            let mapped = address - 0xC000;
            self.wram[mapped as usize] = get_upper_byte(data);
            self.wram[mapped as usize + 1] = get_lower_byte(data);
        } else if address <= 0xFDFF {
            //echo ram
            todo!()
        } else if address <= 0xFE9F {
            let mapped = address - 0xFE00;
            self.sprite_attribute_table[mapped as usize] = get_upper_byte(data);
            self.sprite_attribute_table[mapped as usize + 1] = get_lower_byte(data);
        } else if address <= 0xFEFF {
            //prohibited
            todo!()
        } else if address <= 0xFF7F {
            let mapped = address - 0xFF00;
            self.io_registers[mapped as usize] = get_upper_byte(data);
            self.io_registers[mapped as usize + 1] = get_lower_byte(data);
        } else if address <= 0xFFFE {
            let mapped = address - 0xFF80;
            self.hram[mapped as usize] = get_upper_byte(data);
            self.hram[mapped as usize + 1] = get_lower_byte(data);
        } else {
            if data == 0 {
                self.interrupt_enable_register = false;
            } else {
                self.interrupt_enable_register = true;
            }
        }
    }
}


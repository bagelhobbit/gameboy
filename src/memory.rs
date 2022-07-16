use crate::{
    sprite_attribute::SpriteAttribute,
    tile_info::{TileInfo, TileType},
};

#[derive(Debug, PartialEq, Eq)]
enum CartridgeType {
    Rom,
    Mbc1,
}

#[derive(Debug)]
pub struct Memory {
    boot_rom: [u8; 0x100],
    use_boot_rom: bool,
    /// 16 KiB ROM Bank 00
    /// * From cartridge, usually a fixed bank
    /// * Addressed from `0x0000` to `0x3FFF`
    pub rom: [u8; 0x4000],
    ////16 KiB ROM Bank 01 ~ NN
    /// * From cartridge, switchable bank via mapper (if any)
    /// * Addressed from `0x4000` to `0x7FFF`
    switchable_rom: Vec<[u8; 0x4000]>,
    /// 8 KiB Video RAM (VRAM)
    /// * Addressed from `0x8000` to `0x9FFF`
    pub vram: [u8; 0x2000],
    /// 8 KiB External RAM
    /// * From cartridge, switchable bank if any
    /// * Addressed from `0xA000` to `0xBFFF`
    pub ram: [u8; 0x2000],
    /// 8 KiB Work RAM (WRAM)
    /// * Addressed from `0xC000` to `0xDFFF`
    // In Color GameBoy (CGB) mode, the second half (0xD000 - 0xDFFF) of this block is a switchable bank
    pub wram: [u8; 0x2000],
    /// Sprite Attribute Table
    /// * also Object Attribute Memory (OAM)
    /// * Addressed from `0xFE00` to `0xFE9F`
    pub sprite_attribute_table: [u8; 0xA0],
    /// I/O Registers
    /// * Addressed from `0xFF00` to `0xFF7F`
    pub io_registers: [u8; 0x80],
    /// High RAM (HRAM)
    /// * Addressed from `0xFF80` to `0xFFFE`
    pub hram: [u8; 0x7F],
    /// Interrupt Enable Register
    /// * Addressed at `0xFFFF`
    pub enabled_interupts: u8,
    pub interrupts_enabled: bool,
    cartridge_type: CartridgeType,
    rom_bank: u16,
    max_rom_bank: u16,
    time: u16,
    pub frame_happened: bool,
    divider_register: u32,
    timer_counter: u32,
    timer_modulo: u8,
    timer_enable: bool,
    timer_clock: u16,
    ly: u8,
    pub scy: u8,
    pub scx: u8,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            boot_rom: [0; 0x100],
            use_boot_rom: true,
            rom: [0; 0x4000],
            switchable_rom: Vec::new(),
            vram: [0; 0x2000],
            ram: [0; 0x2000],
            wram: [0; 0x2000],
            sprite_attribute_table: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            enabled_interupts: 0,
            interrupts_enabled: true,
            cartridge_type: CartridgeType::Rom,
            rom_bank: 1,
            max_rom_bank: 2,
            time: 0,
            frame_happened: false,
            divider_register: 0,
            timer_counter: 0,
            timer_modulo: 0,
            timer_enable: false,
            timer_clock: 1024,
            ly: 0,
            scy: 0,
            scx: 0,
        };

        // set default controller inputs to none (0 - pressed)
        mem.io_registers[0] = 0x0F;

        mem
    }

    pub fn using_boot_rom(&self) -> bool {
        self.use_boot_rom
    }

    /// Setup any Memory Bank Controllers (MBCs) that may exist
    pub fn setup_mbc(&mut self, cartridge_type: u8, rom_size: u8) {
        if cartridge_type == 0x00 {
            self.cartridge_type = CartridgeType::Rom;
        } else if cartridge_type == 0x01 {
            self.cartridge_type = CartridgeType::Mbc1;
        }

        match rom_size {
            0x0 => self.max_rom_bank = 2,
            0x1 => self.max_rom_bank = 4,
            0x2 => self.max_rom_bank = 8,
            0x3 => self.max_rom_bank = 16,
            0x4 => self.max_rom_bank = 32,
            0x5 => self.max_rom_bank = 64,
            0x6 => self.max_rom_bank = 128,
            0x7 => self.max_rom_bank = 256,
            _ => self.max_rom_bank = 512,
        }
    }

    pub fn load_boot_rom(&mut self, contents: &[u8]) {
        self.boot_rom[..].clone_from_slice(contents);
    }

    pub fn load_cartridge(&mut self, contents: &Vec<u8>) {
        self.rom[..].clone_from_slice(&contents[..0x4000]);

        let content_size = contents.len();
        for i in 1..self.max_rom_bank as usize {
            let mut data = [0; 0x4000];
            let start_address = 0x4000 * i;
            if content_size >= start_address + 0x4000 {
                data.clone_from_slice(&contents[start_address..(start_address + 0x4000)]);
                self.switchable_rom.push(data);
            } else if content_size > start_address {
                data[..content_size - start_address].clone_from_slice(&contents[start_address..]);
                self.switchable_rom.push(data);
            } else {
                self.switchable_rom.push(data);
            }
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        self.step();
        self.step();
        self.step();
        self.step();

        if self.use_boot_rom && address < 256 {
            self.boot_rom[address as usize]
        } else if address <= 0x3FFF {
            self.rom[address as usize]
        } else if address <= 0x7FFF {
            let mapped = address - 0x4000;
            // Bank $00 is the unswitchable rom bank so subtract one to get the correct index
            self.switchable_rom[self.rom_bank as usize - 1][mapped as usize]
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
            // Echo RAM
            // Nintendo prohibits developers from using this memory range
            let mapped = address - 0xE000;
            self.wram[mapped as usize]
        } else if address <= 0xFE9F {
            let mapped = address - 0xFE00;
            self.sprite_attribute_table[mapped as usize]
        } else if address <= 0xFEFF {
            // Nintendo indicates that this area is prohibited
            // This area returns $FF when OAM is blocked, and otherwise the behavior depends on the hardware revision.
            // On DMG, MGB, SGB, and SGB2, reads during OAM block trigger OAM corruption. Reads otherwise return $00.
            0
        } else if address <= 0xFF7F {
            match address {
                0xFF00 => self.io_registers[0],
                0xFF04 => {
                    // GB freq  4.194304 MHz
                    // DIV freq 16384 Hz
                    // GB freq(Hz) => 4194304 Hz
                    // 4194304 / 16384 = 256
                    (self.divider_register / 256) as u8
                }
                0xFF05 => (self.timer_counter / self.timer_clock as u32) as u8,
                0xFF06 => self.timer_modulo,
                0xFF07 => self.io_registers[0x07],
                0xFF42 => self.scy,
                0xFF43 => self.scx,
                0xFF44 => self.ly,
                _ => {
                    let mapped = address - 0xFF00;
                    self.io_registers[mapped as usize]
                }
            }
        } else if address <= 0xFFFE {
            let mapped = address - 0xFF80;
            self.hram[mapped as usize]
        } else {
            self.enabled_interupts
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.step();
        self.step();
        self.step();
        self.step();

        if address <= 0x3FFF {
            if address >= 0x2000 {
                println!("Bank Selection data: {:0>2x}, bank:{}", data, data & 0x1F);
                let bank = data as u16 & 0x1F;
                self.rom_bank = if bank == 0 { 1 } else { bank };
            }
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
            // Echo RAM
            // Nintendo prohibits developers from using this memory range
            let mapped = address - 0xE000;
            self.wram[mapped as usize] = data;
        } else if address <= 0xFE9F {
            let mapped = address - 0xFE00;
            self.sprite_attribute_table[mapped as usize] = data;
        } else if address <= 0xFEFF {
            // Nintendo indicates that this area is prohibited
        } else if address <= 0xFF7F {
            match address {
                // Only the top 4 bits of $FF00 are writeable, lower 4 are read only controller inputs
                0xFF00 => self.io_registers[0] = (data & 0xF0) + (self.io_registers[0] & 0x0F),
                0xFF04 => self.divider_register = 0,
                0xFF05 => self.timer_counter = data as u32 * self.timer_clock as u32,
                0xFF06 => self.timer_modulo = data,
                0xFF07 => {
                    self.io_registers[0x07] = data;
                    self.timer_enable = data & 0b0000_0100 == 0b0000_0100;
                    let clock_select = data & 0b0000_0011;
                    self.timer_clock = match clock_select {
                        0 => 1024,
                        1 => 16,
                        2 => 64,
                        _ => 256,
                    };
                }
                0xFF42 => self.scy = data,
                0xFF43 => self.scx = data,
                0xFF44 => {} //read-only value
                0xFF46 => {
                    self.dma_transfer(data);
                }
                0xFF50 => {
                    if self.use_boot_rom {
                        self.use_boot_rom = false;
                    }
                }
                _ => {
                    let mapped = address - 0xFF00;
                    self.io_registers[mapped as usize] = data;
                }
            }
        } else if address <= 0xFFFE {
            let mapped = address - 0xFF80;
            self.hram[mapped as usize] = data;
        } else {
            self.enabled_interupts = data;
        }
    }

    fn dma_transfer(&mut self, start_address: u8) {
        let base_address = start_address as u16 * 0x100;
        for address in 0..0xA0 {
            let data = self.read(base_address + address);
            self.write(0xFE00 + address, data);
        }
    }

    fn step(&mut self) {
        self.time += 1;
        self.divider_register += 1;

        if self.divider_register / 256 > 255 {
            self.divider_register = 0;
        }

        if self.timer_enable {
            self.timer_counter += 1;
            if self.timer_counter / self.timer_clock as u32 > 255 {
                self.io_registers[0x0F] |= 0b0000_0100;
            }
        }

        if self.time == 456 {
            self.time = 0;
            self.ly += 1;

            if self.ly == 144 {
                self.io_registers[0x0F] |= 0x01;
            }

            if self.ly == 154 {
                self.ly = 0;
                self.frame_happened = true;
            }
        }
    }

    pub fn vram_read_tile(&mut self, tile_type: TileType, index: u8) -> TileInfo {
        //get LCDC bit 4 to toggle indexing modes (from IO registers)
        // TODO: better way to do this...
        let lcdc4 = self.io_registers[0x40] & 0b0001_0000 == 0b0001_0000;

        match tile_type {
            TileType::Obj => {
                let address = index as usize * 16;
                let mut tile = [0; 16];
                tile.copy_from_slice(&self.vram[address..(address + 16)]);
                TileInfo { tile, tile_type }
            }
            TileType::Window | TileType::Background => {
                if lcdc4 {
                    let address = index as usize * 16;
                    let mut tile = [0; 16];
                    tile.copy_from_slice(&self.vram[address..(address + 16)]);
                    TileInfo { tile, tile_type }
                } else if index >= 128 {
                    let address = 0x0800 + ((index as usize - 128) * 16);
                    let mut tile = [0; 16];
                    tile.copy_from_slice(&self.vram[address..(address + 16)]);
                    TileInfo { tile, tile_type }
                } else {
                    let address = 0x1000 + (index as usize * 16);
                    let mut tile = [0; 16];
                    tile.copy_from_slice(&self.vram[address..(address + 16)]);
                    TileInfo { tile, tile_type }
                }
            }
        }
    }

    pub fn read_bg_tile_map(&self) -> [[u8; 32]; 32] {
        //get LCDC bit 3 to toggle BG tile map locations
        // TODO: better way to do this...
        let start_address = if self.io_registers[0x40] & 0b0000_1000 == 0b0000_1000 {
            0x9C00 - 0x8000
        } else {
            0x9800 - 0x8000
        };

        let indices = self.vram[start_address..(start_address + 0x400)].to_vec();
        let mut result = [[0; 32]; 32];

        for row in 0..32 {
            for col in 0..32 {
                result[row][col] = indices[(row * 32) + col];
            }
        }

        result
    }

    pub fn read_oam(&self) -> Vec<SpriteAttribute> {
        let mut result = Vec::new();

        for (index, _) in (0..self.sprite_attribute_table.len())
            .enumerate()
            .step_by(4)
        {
            let y = self.sprite_attribute_table[index];
            let x = self.sprite_attribute_table[index + 1];
            let tile_index = self.sprite_attribute_table[index + 2];
            let flags = self.sprite_attribute_table[index + 3];

            result.push(SpriteAttribute::new(y, x, tile_index, flags));
        }

        debug_assert_eq!(result.len(), 40);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tile_map() {
        let mut memory = Memory::new();
        let values = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        memory.vram[0x1800..(0x1800 + 32)].copy_from_slice(&values);
        memory.vram[(0x1C00 - 32)..0x1C00].copy_from_slice(&values);

        let tile_map = memory.read_bg_tile_map();

        assert_eq!(tile_map[0], values);
        assert_eq!(tile_map[31], values);
    }
}

use crate::tile_info::{TileInfo, TileType};

#[derive(Debug, PartialEq, Eq)]
enum CartridgeType {
    Rom,
    MBC1,
}

#[derive(Debug)]
pub struct Memory {
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
    // Mirror of C000~DDFF, Nintendo says use of this area is prohibited
    // * Addressed from `0xE000` to `0xFDFF`
    // echo_ram: [u8; 0x1DFF],
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
    boot_rom: [u8; 0x100],
    cartridge_type: CartridgeType,
    use_boot_rom: bool,
    time: u16,
    pub frame_happened: bool,
    ly: u8,
    pub scy: u8,
    pub scx: u8,
}

// I/O Ranges
// start    stop    Purpose
// $FF00			Joypad input
// $FF01	$FF02	Serial transfer
// $FF04	$FF07	Timer and divider
// $FF10	$FF26	Sound
// $FF30	$FF3F	Wave pattern
// $FF40	$FF4B	LCD Control, Status, Position, Scrolling, and Palettes
// $FF50		    Set to non-zero to disable boot ROM

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            rom: [0; 0x4000],
            switchable_rom: vec![[0; 0x4000]],
            vram: [0; 0x2000],
            ram: [0; 0x2000],
            wram: [0; 0x2000],
            sprite_attribute_table: [0; 0xA0],
            io_registers: [0; 0x80],
            hram: [0; 0x7F],
            enabled_interupts: 0,
            interrupts_enabled: true,
            boot_rom: [0; 0x100],
            cartridge_type: CartridgeType::Rom,
            use_boot_rom: true,
            time: 0,
            frame_happened: false,
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
        // $00	ROM ONLY
        // $01	MBC1
        // $02	MBC1+RAM
        // $03	MBC1+RAM+BATTERY
        // $05	MBC2
        // $06	MBC2+BATTERY
        // $08	ROM+RAM 1
        // $09	ROM+RAM+BATTERY 1
        // $0B	MMM01
        // $0C	MMM01+RAM
        // $0D	MMM01+RAM+BATTERY
        // $0F	MBC3+TIMER+BATTERY
        // $10	MBC3+TIMER+RAM+BATTERY 2
        // $11	MBC3
        // $12	MBC3+RAM 2
        // $13	MBC3+RAM+BATTERY 2
        // $19	MBC5
        // $1A	MBC5+RAM
        // $1B	MBC5+RAM+BATTERY
        // $1C	MBC5+RUMBLE
        // $1D	MBC5+RUMBLE+RAM
        // $1E	MBC5+RUMBLE+RAM+BATTERY
        // $20	MBC6
        // $22	MBC7+SENSOR+RUMBLE+RAM+BATTERY
        // $FC	POCKET CAMERA
        // $FD	BANDAI TAMA5
        // $FE	HuC3
        // $FF	HuC1+RAM+BATTERY
        if cartridge_type == 0x00 {
            self.cartridge_type = CartridgeType::Rom;
        } else if cartridge_type == 0x01 {
            self.cartridge_type = CartridgeType::MBC1;
        } else if cartridge_type <= 0x03 {
            // plus ram access
            self.cartridge_type = CartridgeType::MBC1;
        }
    }

    pub fn load_boot_rom(&mut self, contents: &Vec<u8>) {
        self.boot_rom[..].clone_from_slice(contents);
    }

    pub fn load_cartridge(&mut self, contents: &Vec<u8>) {
        self.rom[..].clone_from_slice(&contents[..0x4000]);
        self.switchable_rom[0][..contents.len() - 0x4000].clone_from_slice(&contents[0x4000..]);
    }

    //memory mapper
    // should this take a length as well?, or just map one address at a time
    // also might want to move this into some sort of address bus type (rename `Memory` to `AddressBus` and make arrays private?)
    pub fn read(&mut self, address: u16) -> u8 {
        self.step();
        self.step();
        self.step();
        self.step();

        if self.use_boot_rom && address < 256 {
            self.boot_rom[address as usize]
        } else if address <= 0x3FFF {
            if self.cartridge_type == CartridgeType::MBC1 {
                if address < 0x2000 {
                    self.rom[address as usize]
                } else {
                    todo!()
                }
            } else {
                self.rom[address as usize]
            }
        } else if address <= 0x7FFF {
            let mapped = address - 0x4000;
            self.switchable_rom[0][mapped as usize]
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
            self.rom[address as usize] = data;
        } else if address <= 0x7FFF {
            let mapped = address - 0x4000;
            self.switchable_rom[0][mapped as usize] = data;
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
            if self.use_boot_rom && address == 0xFF50 {
                self.use_boot_rom = false;
            }
            match address {
                // Only the top 4 bits of $FF00 are writeable, lower 4 are read only controller inputs
                0xFF00 => self.io_registers[0] = (data & 0xF0) + (self.io_registers[0] & 0x0F) ,
                0xFF42 => self.scy = data,
                0xFF43 => self.scx = data,
                0xFF44 => self.ly = data,
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

    fn step(&mut self) {
        self.time += 1;

        if self.time == 456 {
            self.time = 0;
            self.ly += 1;

            if self.ly == 154 {
                self.ly = 0;
                self.frame_happened = true;
            }
        }
    }

    pub fn vram_write_tile(&mut self, tile_info: TileInfo, index: u8) {
        //get LCDC bit 4 to toggle indexing modes (from IO registers)
        // TODO: better way to do this... (also is this even the right bit?)
        let lcdc4 = if self.io_registers[0x40] & 0b0001_0000 == 0b0001_0000 {
            true
        } else {
            false
        };

        match tile_info.tile_type {
            TileType::Obj => {
                let address = index as usize * 16;
                self.vram[address..(address + 16)].copy_from_slice(&tile_info.tile);
            }
            TileType::Window | TileType::Background => {
                if lcdc4 {
                    let address = index as usize * 16;
                    self.vram[address..(address + 16)].copy_from_slice(&tile_info.tile);
                } else {
                    if index >= 128 {
                        let address = 0x0800 + ((index as usize - 128) * 16);
                        self.vram[address..(address + 16)].copy_from_slice(&tile_info.tile);
                    } else {
                        let address = 0x1000 + (index as usize * 16);
                        self.vram[address..(address + 16)].copy_from_slice(&tile_info.tile);
                    }
                }
            }
        }
    }

    pub fn vram_read_tile(&mut self, tile_type: TileType, index: u8) -> TileInfo {
        //get LCDC bit 4 to toggle indexing modes (from IO registers)
        // TODO: better way to do this...
        let lcdc4 = if self.io_registers[0x40] & 0b0001_0000 == 0b0001_0000 {
            true
        } else {
            false
        };

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
                } else {
                    if index >= 128 {
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
    }

    pub fn read_tile_map(&self) -> [[u8; 32]; 32] {
        //get LCDC bit 6 to toggle tile map locations
        // TODO: better way to do this...
        let start_address = if self.io_registers[0x40] & 0b0100_0000 == 0b0100_0000 {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_obj_tile_info() -> TileInfo {
        let tile = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];

        TileInfo {
            tile,
            tile_type: TileType::Obj,
        }
    }

    fn get_window_tile_info() -> TileInfo {
        let tile = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];

        TileInfo {
            tile,
            tile_type: TileType::Window,
        }
    }

    fn get_bg_tile_info() -> TileInfo {
        let tile = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56,
            0x38, 0x7C,
        ];

        TileInfo {
            tile,
            tile_type: TileType::Background,
        }
    }

    #[test]
    fn test_vram_write_tile_obj() {
        let mut memory = Memory::new();
        let tile_info = get_obj_tile_info();

        memory.vram_write_tile(get_obj_tile_info(), 0);
        memory.vram_write_tile(get_obj_tile_info(), 120);
        memory.vram_write_tile(get_obj_tile_info(), 255);

        assert_eq!(memory.vram[0..16], tile_info.tile);
        assert_eq!(memory.vram[(120 * 16)..((120 * 16) + 16)], tile_info.tile);
        assert_eq!(memory.vram[(255 * 16)..((255 * 16) + 16)], tile_info.tile);
    }

    #[test]
    fn test_vram_write_tile_window_lcdc_true() {
        let mut memory = Memory::new();
        let tile_info = get_window_tile_info();

        memory.io_registers[0x40] = 0b0001_0000;

        memory.vram_write_tile(get_window_tile_info(), 0);
        memory.vram_write_tile(get_window_tile_info(), 120);
        memory.vram_write_tile(get_window_tile_info(), 135);
        memory.vram_write_tile(get_window_tile_info(), 255);

        assert_eq!(memory.vram[0..16], tile_info.tile);
        assert_eq!(memory.vram[(120 * 16)..((120 * 16) + 16)], tile_info.tile);
        assert_eq!(memory.vram[(135 * 16)..((135 * 16) + 16)], tile_info.tile);
        assert_eq!(memory.vram[(255 * 16)..((255 * 16) + 16)], tile_info.tile);
    }

    #[test]
    fn test_vram_write_tile_background_lcdc_false() {
        let mut memory = Memory::new();
        let tile_info = get_bg_tile_info();

        memory.vram_write_tile(get_bg_tile_info(), 0);
        memory.vram_write_tile(get_bg_tile_info(), 120);
        memory.vram_write_tile(get_bg_tile_info(), 135);
        memory.vram_write_tile(get_bg_tile_info(), 255);

        let address_offset = 0x1000;
        assert_eq!(
            memory.vram[address_offset..(address_offset + 16)],
            tile_info.tile
        );

        let address_offset = 0x1000 + (120 * 16);
        assert_eq!(
            memory.vram[address_offset..(address_offset + 16)],
            tile_info.tile
        );

        let address_offset = 0x800 + ((135 - 128) * 16);
        assert_eq!(
            memory.vram[address_offset..(address_offset + 16)],
            tile_info.tile
        );

        let address_offset = 0x800 + ((255 - 128) * 16);
        assert_eq!(
            memory.vram[address_offset..(address_offset + 16)],
            tile_info.tile
        );
    }

    #[test]
    fn test_vram_read_tile_backgound_lcdc_true() {
        let mut memory = Memory::new();
        let tile_info = get_bg_tile_info();

        memory.io_registers[0x40] = 0b0001_0000;

        memory.vram_write_tile(get_bg_tile_info(), 1);

        assert_eq!(memory.vram[16..32], tile_info.tile);
        assert_eq!(
            memory.vram_read_tile(TileType::Background, 1).tile,
            tile_info.tile
        );
    }

    #[test]
    fn test_read_tile_map() {
        let mut memory = Memory::new();
        let values = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        memory.vram[0x1800..(0x1800 + 32)].copy_from_slice(&values);
        memory.vram[(0x1C00 - 32)..0x1C00].copy_from_slice(&values);

        let tile_map = memory.read_tile_map();

        assert_eq!(tile_map[0], values);
        assert_eq!(tile_map[31], values);
    }
}

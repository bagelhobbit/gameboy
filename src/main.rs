use crate::{cpu::Cpu, instructions::Instruction, memory::Memory, tile_info::TileType};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
use std::{collections::HashSet, env, fs};
use util::get_as_bits;

mod alu_result;
mod cpu;
mod instructions;
mod memory;
mod sprite_attribute;
mod tile_info;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: gameboy <file>");
        return;
    }

    let filename = &args[1];

    let bios_contents = fs::read("boot.gb").expect("Error reading Boot ROM");

    let contents = fs::read(filename).expect("Error reading the given filename");

    let mut cpu = Cpu::new();

    let mut memory = Memory::new();

    memory.load_boot_rom(&bios_contents);
    memory.load_cartridge(&contents);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("GameBoy Emulator", 160 * 2, 144 * 2)
        .position_centered()
        .build()
        .unwrap();

    let (width, height) = window.size();
    let pixel_width = width / 160;
    let pixel_height = height / 144;

    let color0 = Color::RGB(0xE0, 0xF8, 0xD0);
    let color1 = Color::RGB(0x88, 0xC0, 0x70);
    let color2 = Color::RGB(0x34, 0x68, 0x56);
    let color3 = Color::RGB(0x08, 0x18, 0x20);
    let color_transparent = Color::RGBA(0, 0, 0, 0);

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut old_scancodes: HashSet<Scancode> = HashSet::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::F12),
                    ..
                } => {
                    cpu.debug = true;
                    memory.debug = true;
                }
                _ => {}
            }
        }

        let pressed_keys = pressed_keycode_set(&event_pump);
        memory.set_joypad_inputs(pressed_keys);

        for _ in 0..60 {
            let instruction = cpu.parse(&mut memory);

            if !memory.using_boot_rom() && cpu.debug {
                println!("addr={:0>4x}, a={:0>2x}, f={:0>2x}, b={:0>2x}, c={:0>2x}, d={:0>2x}, e={:0>2x}, h={:0>2x}, l={:0>2x}, sp={:0>4x}, code={:0>2x} {:?}",
                cpu.program_counter, cpu.a, cpu.flags_to_byte(), cpu.b, cpu.c, cpu.d,cpu.e, cpu.h,cpu.l, cpu.stack_pointer, memory.read(cpu.program_counter), instruction);
            }

            if instruction == Instruction::Invalid {
                panic!("Invalid Instruction");
            }

            cpu.execute(instruction, &mut memory);
        }

        if memory.frame_happened {
            canvas.clear();
            let tilemap = memory.read_bg_tile_map();

            let mut color0_rects = Vec::new();
            let mut color1_rects = Vec::new();
            let mut color2_rects = Vec::new();
            let mut color3_rects = Vec::new();
            let mut transparent_rects = Vec::new();

            let mut palette: u8;
            let palette_bits = get_as_bits(memory.io_registers[0x47]);

            let color_values = [
                (palette_bits[6] << 1) + palette_bits[7],
                (palette_bits[4] << 1) + palette_bits[5],
                (palette_bits[2] << 1) + palette_bits[3],
                (palette_bits[0] << 1) + palette_bits[1],
            ];

            // background is 18 tiles tall and 20 tiles wide
            // render an extra tile on all sides to enable partially rendering tiles from offscreen
            for y in 0..20 {
                for x in 0..22 {
                    let tile = memory.vram_read_tile(
                        TileType::Background,
                        tilemap[((memory.scy as usize / 8) + y) % 32]
                            [((memory.scx as usize / 8) + x) % 32],
                    );

                    let x_pos = x as i32 * 8;
                    let y_pos = y as i32 * 8;

                    let x_offset = memory.scx as i32 % 8;
                    let y_offset = memory.scy as i32 % 8;

                    let colors = tile.get_color_ids_from_tile();

                    for row in 0..colors.len() {
                        for col in 0..colors[0].len() {
                            let rect = Rect::new(
                                (x_pos + col as i32 - x_offset) * pixel_width as i32,
                                (y_pos + row as i32 - y_offset) * pixel_height as i32,
                                pixel_width,
                                pixel_height,
                            );

                            palette = color_values[colors[row][col] as usize];

                            if palette == 0 {
                                color0_rects.push(rect);
                            } else if palette == 1 {
                                color1_rects.push(rect);
                            } else if palette == 2 {
                                color2_rects.push(rect);
                            } else {
                                color3_rects.push(rect);
                            }
                        }
                    }
                }
            }

            for sprite in memory.read_oam() {
                if sprite.y == 0 || (memory.io_registers[0x40] & 0b0000_0100 == 0 && sprite.y <= 8)
                {
                    //LCDC bit 2 == false, use 8x8 sprite mode
                    continue;
                }

                //TODO: handle x == 0 (still effects scanline limit)
                //TODO: handle scanline limits and selection priority
                //TODO: handle bg+window over obj
                //TODO: handle 8x16 sprites

                let tile = memory.vram_read_tile(TileType::Obj, sprite.index);

                let x_pos = sprite.x as i32 - 8;
                let y_pos = sprite.y as i32 - 16;

                let mut colors = tile.get_color_ids_from_tile();

                if sprite.x_flip {
                    for col in colors.iter_mut() {
                        col.reverse();
                    }
                }

                if sprite.y_flip {
                    colors.reverse();
                }

                let palette_reg = if sprite.palette == 0 { 0x48 } else { 0x49 };
                let palette_bits = get_as_bits(memory.io_registers[palette_reg]);

                // Lower 2 bits ignored since they are transparent
                // Keep a value so indexing works as expected
                let color_values = [
                    0,
                    (palette_bits[4] << 1) + palette_bits[5],
                    (palette_bits[2] << 1) + palette_bits[3],
                    (palette_bits[0] << 1) + palette_bits[1],
                ];

                for row in 0..colors.len() {
                    for col in 0..colors[0].len() {
                        let rect = Rect::new(
                            (x_pos + col as i32) * pixel_width as i32,
                            (y_pos + row as i32) * pixel_height as i32,
                            pixel_width,
                            pixel_height,
                        );

                        palette = color_values[colors[row][col] as usize];

                        if palette == 0 {
                            transparent_rects.push(rect);
                        } else if palette == 1 {
                            color1_rects.push(rect);
                        } else if palette == 2 {
                            color2_rects.push(rect);
                        } else {
                            color3_rects.push(rect);
                        }
                    }
                }
            }

            canvas.set_draw_color(color0);
            canvas.fill_rects(&color0_rects).unwrap();

            canvas.set_draw_color(color1);
            canvas.fill_rects(&color1_rects).unwrap();

            canvas.set_draw_color(color2);
            canvas.fill_rects(&color2_rects).unwrap();

            canvas.set_draw_color(color3);
            canvas.fill_rects(&color3_rects).unwrap();

            canvas.set_draw_color(color_transparent);
            canvas.fill_rects(&transparent_rects).unwrap();

            memory.frame_happened = false;
        }

        // old_scancodes = pressed_scancode_set(&event_pump);

        canvas.present();
    }
}

fn pressed_keycode_set(event_pump: &sdl2::EventPump) -> HashSet<Keycode> {
    event_pump
        .keyboard_state()
        .pressed_scancodes()
        .filter_map(Keycode::from_scancode)
        .collect()
}

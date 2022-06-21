use crate::{cpu::Cpu, instructions::Instruction, memory::Memory};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::{env, fs};

mod cpu;
mod instructions;
mod io_registers;
mod memory;
mod tile_info;
mod util;
mod vram;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("usage: gameboy <file>");
        return;
    }

    let filename = &args[1];

    let bios_contents =
        fs::read("BIOS_Nintendo_Game_Boy_Boot_ROM_World.gb").expect("Error reading BIOS");

    let contents = fs::read(filename).expect("Error reading the given filename");

    let mut cpu = Cpu::new();

    let mut memory = Memory::new();

    let cartridge_type = contents[0x147];
    let rom_size = contents[0x148];
    memory.setup_mbc(cartridge_type, rom_size);
    memory.load_boot_rom(&bios_contents);
    memory.load_cartridge(&contents);

    println!("0x{:0>2X?} - 0x{:0>2X?}", cartridge_type, rom_size);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("GameBoy Emulator", 444, 400)
        .position_centered()
        .build()
        .unwrap();

    let (width, height) = window.size();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
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
                _ => {}
            }
        }

        // let pressed_keys = pressed_keycode_set(&event_pump);
        // let new_keys: HashSet<Keycode> =
        //     newly_pressed(&old_scancodes, &pressed_scancode_set(&event_pump))
        //         .iter()
        //         .filter_map(|&s| Keycode::from_scancode(s))
        //         .collect();

        for _ in 0..20 {
            let instruction = cpu.parse(&memory);

            println!(
                "${:0>4X?} - 0x{:0>2X?} - {:?}",
                cpu.program_counter,
                memory.read(cpu.program_counter),
                instruction
            );
            // println!("SP: {:0>4X?}", cpu.stack_pointer);
            // println!("HL: {:0>4X?}", cpu.hl());

            if instruction == Instruction::Invalid {
                break;
            }

            cpu.execute(instruction, &mut memory);
        }

        // old_scancodes = pressed_scancode_set(&event_pump);

        // let mut filled_rects = Vec::new();
        // let mut blank_rects = Vec::new();

        // for row in 0..(display_constants::HEIGHT as usize) {
        //     for col in 0..(display_constants::WIDTH as usize) {
        //         if memory.display[row][col] == 1 {
        //             filled_rects.push(Rect::new(
        //                 col as i32 * display_constants::SCALE as i32,
        //                 row as i32 * display_constants::SCALE as i32,
        //                 display_constants::SCALE,
        //                 display_constants::SCALE,
        //             ));
        //         } else {
        //             blank_rects.push(Rect::new(
        //                 col as i32 * display_constants::SCALE as i32,
        //                 row as i32 * display_constants::SCALE as i32,
        //                 display_constants::SCALE,
        //                 display_constants::SCALE,
        //             ));
        //         }
        //     }
        // }

        // canvas.set_draw_color(Color::RGB(0, 0, 0));
        // canvas.fill_rects(&filled_rects).unwrap();

        // canvas.set_draw_color(Color::RGB(0, 255, 255));
        // canvas.fill_rects(&blank_rects).unwrap();

        let pixel_width = width / 160;
        let pixel_height = height / 144;

        let color0 = Color::RGB(0x08, 0x18, 0x20);
        let color1 = Color::RGB(0x34, 0x68, 0x56);
        let color2 = Color::RGB(0x88, 0xC0, 0x70);
        let color3 = Color::RGB(0xE0, 0xF8, 0xD0);

        canvas.present();
        // std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

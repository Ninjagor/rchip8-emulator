use chip8::chip8::Chip8;
use chip8::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use renderer::init::{init_sdl, InitSdlReturn};

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;
const CYCLE: usize = 10;

const ROMS_DIR: &str = "../ROMs/";

fn main() {
    let roms_dir = Path::new(ROMS_DIR);
    let rom_files: Vec<_> = match fs::read_dir(roms_dir) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .collect(),
        Err(_) => {
            println!("Could not open the ROMs directory.");
            return;
        }
    };

    println!("Available ROMs:");
    for (index, entry) in rom_files.iter().enumerate() {
        println!("{}: {}", index + 1, entry.file_name().to_string_lossy());
    }

    println!("Enter the number of the ROM to load:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let choice: usize = match input.trim().parse::<usize>() {
        Ok(num) if num > 0 && num <= rom_files.len() => num - 1,
        _ => {
            println!("Invalid selection. Exiting.");
            return;
        }
    };

    let init: InitSdlReturn = init_sdl(WINDOW_WIDTH, WINDOW_HEIGHT);
    let sdl_context = init.sdl_context;
    let mut canvas = init.canvas;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_fonts();

    let rom_path = rom_files[choice].path();
    let mut rom = File::open(&rom_path).expect("Unable to open ROM file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).unwrap();
    chip8.load_rom(&buffer);

    'execloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'execloop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = renderer::input::key_btn_mapper(key) {
                        chip8.keypress(k, true);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = renderer::input::key_btn_mapper(key) {
                        chip8.keypress(k, false);
                    }
                }
                _ => (),
            }
        }

        for _ in 0..CYCLE {
            chip8.tick();
        }
        chip8.tick_timers();
        renderer::renderer::draw_screen(&chip8, &mut canvas);
    }
}

use chip8::chip8::Chip8;
use chip8::constants::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use renderer::init::{init_sdl, InitSdlReturn};

use std::env;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 15;
const WINDOW_WIDTH: u32 = (DISPLAY_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (DISPLAY_HEIGHT as u32) * SCALE;
const CYCLE: usize = 10;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let init: InitSdlReturn = init_sdl(WINDOW_WIDTH, WINDOW_HEIGHT);

    let sdl_context = init.sdl_context;
    let mut canvas = init.canvas;

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_fonts();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
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

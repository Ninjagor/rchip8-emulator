use sdl2::{render::Canvas, video::Window, Sdl};

pub struct InitSdlReturn {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
}

pub fn init_sdl(w: u32, h: u32) -> InitSdlReturn {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Rohit's Rust CHIP8 Emulator", w, h)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    InitSdlReturn {
        sdl_context,
        canvas,
    }
}

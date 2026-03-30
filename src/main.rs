use std::env;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window,
};

use crate::emulator::Emulator;
mod cpu;
mod emulator;
const SCALING: u32 = 20;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }
    let mut emulator = Emulator::initialize();
    emulator.load_game(&args[1]);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(&args[1], 64 * SCALING, 32 * SCALING)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
        emulator.tick();
        draw_screen(&emulator, &mut canvas)
    }
}

fn draw_screen(emulator: &Emulator, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emulator.get_screen();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            //TODO
            let x = (i % 64) as u32;
            let y = (i / 32) as u32;
            let rect = Rect::new((x*SCALING) as i32, (y * SCALING) as i32, SCALING, SCALING);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

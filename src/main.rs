mod emulib;

use emulib::emulib::*;
use std::env;
use sdl2::event::Event;

fn main() {
    let args: Vec<_> = env::args().collect();

    /*
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }
     */

    let chip8 = Emu::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("CHIP8 EMU", (SCREEN_WIDTH * (SCALE as usize)) as u32, (SCREEN_HEIGHT * (SCALE as usize)) as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'gameloop;
                },

                _ => ()
            }
        }
    }
}
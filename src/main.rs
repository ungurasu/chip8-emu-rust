mod emulib;

use emulib::emulib::*;

use std::env;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn draw_screen(emu: &Emu, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = emu.get_display();

    canvas.set_draw_color(Color::RGB(255,255,255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel {
            let x = (i % SCREEN_WIDTH) as u32;
            let y = (i / SCREEN_WIDTH) as u32;

            //println!("i {} x {} y {} w", i, x, y);

            let rect = Rect::new((x * SCALE) as i32,
                                 (y * SCALE) as i32,
                                 SCALE,
                                 SCALE,
            );
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut chip8 = Emu::new();

    println!("File is {}", &args[1]);

    match env::current_exe() {
        Ok(exe_path) => println!("Path of this executable is: {}",
                                 exe_path.display()),
        Err(e) => println!("failed to get current exe path: {e}"),
    };

    let mut rom = File::open(&args[1]).expect("Unable to open file!");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8.load(&buffer);

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

        for _ in 0..TICKS_PER_FRAME {
            chip8.tick();
        }
        chip8.tick_timers();
        draw_screen(&chip8, &mut canvas);
    }
}
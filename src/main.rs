extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::thread::sleep;


pub fn main() {
    let sdl_context = sdl2::init().expect("Couldn't initialize sdl2");
    let video_subsystem = sdl_context.video().expect("Couldn't get video");
    let window = video_subsystem.window("Tetris", 800, 600).position_centered().opengl().build().expect("Failed to create window");
    let mut canvas = window.into_canvas().build().expect("Failed to create canvas from window");
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get sdl event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
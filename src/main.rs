use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("Could not init SDL");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem
        .window("A Rust Game", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to build main window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to get SDL canvas");
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion {
                    x, y, xrel, yrel, ..
                } => {
                    println!("x: {}, y: {}; xrel: {}, yrel: {}", x, y, xrel, yrel);
                }
                _ => {}
            }
        }

        let (w, h) = {
            let (w, h) = canvas.window().size();
            (w as i32, h as i32)
        };
        let cell_width = w / 5;
        let cell_height = h / 5;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        for i in 0..5 {
            canvas
                .draw_line(
                    Point::new(cell_width / 2, cell_height / 2 + i * cell_height),
                    Point::new(w - cell_width / 2, cell_height / 2 + i * cell_height),
                )
                .expect("Fail do draw lines");

            canvas
                .draw_line(
                    Point::new(cell_width / 2 + i * cell_width, cell_height / 2),
                    Point::new(cell_width / 2 + i * cell_width, h - cell_height / 2),
                )
                .expect("Fail do draw columns");
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

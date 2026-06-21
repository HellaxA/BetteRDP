use std::time::Instant;

use minifb::{Key, Window, WindowOptions};
use screenshots::Screen;

pub fn test_capture() {
    let screen = Screen::all().unwrap().remove(0);
    let width = screen.display_info.width as usize;
    let height = screen.display_info.height as usize;

    println!("Current display sizes are {width}*{height}");

    let mut window = Window::new(
        "Screen Capture",
        width / 2,
        height / 2,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let capture_start = Instant::now();
        let image = screen.capture().unwrap();
        let capture_time = capture_start.elapsed();

        let convert_start = Instant::now();
        let buffer: Vec<u32> = image
            .as_raw()
            .chunks(4)
            .map(|p| {
                let r = p[0] as u32;
                let g = p[1] as u32;
                let b = p[2] as u32;
                (r << 16) | (g << 8) | b
            })
            .collect();
        let convert_time = convert_start.elapsed();

        let draw_start = Instant::now();
        window.update_with_buffer(&buffer, width, height).unwrap();
        let draw_time = draw_start.elapsed();
        println!(
            "capture: {:?} | convert: {:?} | draw: {:?}",
            capture_time, convert_time, draw_time
        );
    }
}

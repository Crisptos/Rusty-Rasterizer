use minifb::{Key, Window, WindowOptions};
use rusty_rasterizer::{color::Color, screen::{fill, draw}, global::{WIDTH, HEIGHT}};

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Software Rasterizer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        fill(&mut buffer, 0); // Black Screen
        draw(&mut buffer, WIDTH/2, HEIGHT/2, Color::new(255,0,0).value()); // Draw red dot in the middle of the screen

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

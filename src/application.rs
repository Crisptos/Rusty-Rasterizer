use std::time::{Duration, Instant};

use crate::{
    color::Color,
    mesh::Mesh,
    projection::{world_to_screen, rotate},
    screen::{draw, fill}, global::{WIDTH, HEIGHT},
};
use minifb::{Key, Window, WindowOptions};
use raqote::*;

struct DeltaTime{
    start: Instant,
    now: Duration,
    last_time: f32,
}

pub struct Application {
    window: Window,
    buffer: DrawTarget,
    elapsed_time: DeltaTime,
    mesh: Mesh,
}

impl Application {
    // Create a new Window and Framebuffer handle
    pub fn new(width: usize, height: usize) -> Self {
        Application {
            window: Window::new(
                "Software Rasterizer",
                width,
                height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            }),

            buffer: DrawTarget::new(width as i32, height as i32),

            mesh: Mesh::new_cube(),

            elapsed_time: DeltaTime{start: Instant::now(), now: Duration::default(), last_time: 0.0}
        }
    }

    pub fn init(&mut self) {
        self.window
            .limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    }

    pub fn update(&mut self) {
        self.buffer.clear(SolidSource::from_unpremultiplied_argb(0xff, 0, 0, 0));
        let delta_time = self.delta_time();
        for tri in &mut self.mesh.tris {
            // Clear render buffer with solid black

            let mut pb = PathBuilder::new();

            // Rotate Triangle
            rotate(tri, delta_time);

            // Transform the world space triangle into a projected screen space triangle
            let mut trans_tri = tri.clone();
            trans_tri.points[0].z += 0.0;
            trans_tri.points[1].z += 0.0;
            trans_tri.points[2].z += 0.0;


            let mut proj_tri = world_to_screen(&trans_tri);

            // Scale the screen space triangle into our viewport coordinates
            proj_tri.points[0].x += 1.0;
            proj_tri.points[0].y += 1.0;
            proj_tri.points[1].x += 1.0;
            proj_tri.points[1].y += 1.0;
            proj_tri.points[2].x += 1.0;
            proj_tri.points[2].y += 1.0;

            proj_tri.points[0].x *= 0.5 * WIDTH as f32;
            proj_tri.points[0].y *= 0.5 * HEIGHT as f32;
            proj_tri.points[1].x *= 0.5 * WIDTH as f32;
            proj_tri.points[1].y *= 0.5 * HEIGHT as f32;
            proj_tri.points[2].x *= 0.5 * WIDTH as f32;
            proj_tri.points[2].y *= 0.5 * HEIGHT as f32;

            // Draw the triangle
            pb.move_to(proj_tri.points[0].x, proj_tri.points[0].y);
            pb.line_to(proj_tri.points[1].x, proj_tri.points[1].y);
            pb.line_to(proj_tri.points[2].x, proj_tri.points[2].y);
            pb.line_to(proj_tri.points[0].x, proj_tri.points[0].y);
            let path = pb.finish();
            // Draw Triangle Wireframe
            self.buffer.stroke(
                &path,
                &Source::Solid(SolidSource::from_unpremultiplied_argb(0xFF, 0xFF, 0, 0)),
                &StrokeStyle::default(),
                &DrawOptions::new(),
            );
        }
    }

    pub fn draw_frame(&mut self) {
        let (width, height) = self.window.get_size();
        self.window
            .update_with_buffer(self.buffer.get_data(), width, height)
            .unwrap();
    }

    pub fn is_running(&mut self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn delta_time(&mut self) -> f32{
        self.elapsed_time.now = Instant::elapsed(&self.elapsed_time.start);
        let delta_time = self.elapsed_time.now.as_secs_f32() - self.elapsed_time.last_time;
        self.elapsed_time.last_time = self.elapsed_time.now.as_secs_f32();
        delta_time
    }
}

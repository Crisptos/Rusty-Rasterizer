extern crate nalgebra_glm as glm;
use crate::global::{HEIGHT, WIDTH, TO_RADIANS};
use crate::mesh::Triangle;

pub fn rotate(tri: &mut Triangle, delta_time: f32) {
    tri.points[0] = glm::rotate_vec4(&tri.points[0], (15.0*TO_RADIANS)*delta_time, &glm::Vec3::new(1.0, 0.0, 0.0));
    tri.points[1] = glm::rotate_vec4(&tri.points[1], (15.0*TO_RADIANS)*delta_time, &glm::Vec3::new(1.0, 0.0, 0.0));
    tri.points[2] = glm::rotate_vec4(&tri.points[2], (15.0*TO_RADIANS)*delta_time, &glm::Vec3::new(1.0, 0.0, 0.0));

    tri.points[0] = glm::rotate_vec4(&tri.points[0], (5.0*TO_RADIANS)*delta_time, &glm::Vec3::new(0.0, 0.0, 1.0));
    tri.points[1] = glm::rotate_vec4(&tri.points[1], (5.0*TO_RADIANS)*delta_time, &glm::Vec3::new(0.0, 0.0, 1.0));
    tri.points[2] = glm::rotate_vec4(&tri.points[2], (5.0*TO_RADIANS)*delta_time, &glm::Vec3::new(0.0, 0.0, 1.0));
}

pub fn world_to_screen(tri: &Triangle) -> Triangle {
    let proj = glm::perspective_fov(90.0, WIDTH as f32, HEIGHT as f32, 0.1, 1000.0);
    let v0: glm::Vec4 = proj * tri.points[0];
    let v1: glm::Vec4 = proj * tri.points[1];
    let v2: glm::Vec4 = proj * tri.points[2];
    Triangle {
        points: ([v0, v1, v2]),
    }
}

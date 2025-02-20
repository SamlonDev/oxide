use std::f32::consts::PI;

use self::{angles::RotationVectors, vector3::Vector3};

pub mod angles;
pub mod vector2;
pub mod vector3;
pub mod vector4;
pub mod view_matrix;

pub fn dtr(deg: f32) -> f32 {
    (deg / 180f32) * PI
}
pub fn rtd(rad: f32) -> f32 {
    (rad / PI) * 180f32
}

pub fn get_corners(pos: &Vector3, rotation: &RotationVectors, min: &Vector3, max: &Vector3) -> [Vector3; 8] {
    let mut corners = [
        Vector3::new(max.x, max.y, max.z),
        Vector3::new(min.x, max.y, max.z),
        Vector3::new(max.x, min.y, max.z),
        Vector3::new(min.x, min.y, max.z),
        Vector3::new(max.x, max.y, min.z),
        Vector3::new(min.x, max.y, min.z),
        Vector3::new(max.x, min.y, min.z),
        Vector3::new(min.x, min.y, min.z),
    ];
    for corner in &mut corners {
        *corner = corner.rotate(&rotation) + *pos
    }
    corners
}

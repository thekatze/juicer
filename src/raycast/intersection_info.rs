use crate::raycast::ray::Ray;
use crate::vector::Vector;

pub struct IntersectionInfo {
    pub distance: f32,
    pub point: Vector<3, f32>,
    pub normal: Vector<3, f32>,
    pub front_face: bool,
}

impl IntersectionInfo {
    pub fn from_ray(ray: &Ray, distance: f32, normal: Vector<3, f32>) -> Self {
        let front_face = ray.direction.dot(&normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Self {
            distance,
            point: ray.at(distance),
            normal,
            front_face,
        }
    }
}


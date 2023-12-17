use std::ops::Range;

use crate::{
    raycast::{intersection_info::IntersectionInfo, ray::Ray, raycast_target::RaycastTarget},
    vector::Vector,
};

pub struct Sphere {
    pub origin: Vector<3, f32>,
    pub radius: f32,
}

impl RaycastTarget for Sphere {
    fn get_intersection(&self, ray: &Ray, bounds: Range<f32>) -> Option<IntersectionInfo> {
        let sphere_direction = ray.start.clone() - self.origin.clone();

        // solve using quadratic formula
        let a = ray.direction.len_squared();
        let half_b = sphere_direction.dot(&ray.direction);
        let c = sphere_direction.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();

        let mut distance = (-half_b - discriminant) / a;
        if !bounds.contains(&distance) {
            distance = (-half_b + discriminant) / a;
            if !bounds.contains(&distance) {
                return None;
            }
        }

        // fast normal by dividing by radius instead of sqrt
        let normal = (ray.at(distance) - self.origin.clone()) / self.radius;

        Some(IntersectionInfo::from_ray(&ray, distance, normal))
    }
}

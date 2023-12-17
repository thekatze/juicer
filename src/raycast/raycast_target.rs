use std::ops::Range;

use crate::vector::Vector;

use crate::raycast::intersection_info::IntersectionInfo;
use crate::raycast::ray::Ray;

pub enum RaycastTarget {
    Sphere { origin: Vector<3, f32>, radius: f32 },
}

impl RaycastTarget {
    pub fn get_intersection(&self, ray: &Ray, bounds: &Range<f32>) -> Option<IntersectionInfo> {
        match self {
            RaycastTarget::Sphere { origin, radius } => {
                let sphere_direction = ray.start.clone() - origin.clone();

                // solve using quadratic formula
                let a = ray.direction.len_squared();
                let half_b = sphere_direction.dot(&ray.direction);
                let c = sphere_direction.len_squared() - radius * radius;

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
                let normal = (ray.at(distance) - origin.clone()) / *radius;

                Some(IntersectionInfo::from_ray(&ray, distance, normal))
            }
        }
    }
}

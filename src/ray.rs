use std::ops::Range;

use crate::vector::Vector;

pub struct Ray {
    pub start: Vector<3, f32>,
    pub direction: Vector<3, f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector<3, f32> {
        self.start.clone() + self.direction.clone() * t
    }

    pub fn color(&self) -> Vector<3, f32> {
        let sphere = Sphere {
            origin: Vector([0.0, 0.0, -1.0]),
            radius: 0.5,
        };

        if let Some(intersection) = sphere.get_intersection(self, 0.0..50.0) {
            return Vector([
                intersection.normal.x() + 1.0,
                intersection.normal.y() + 1.0,
                intersection.normal.z() + 1.0,
            ]) * 0.5;
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vector([1.0, 1.0, 1.0]) * (1.0 - a) + Vector([0.5, 0.7, 1.0]) * a
    }
}

struct Sphere {
    origin: Vector<3, f32>,
    radius: f32,
}

struct IntersectionInfo {
    distance: f32,
    point: Vector<3, f32>,
    normal: Vector<3, f32>,
    front_face: bool,
}

impl IntersectionInfo {
    fn from_ray(ray: &Ray, distance: f32, normal: Vector<3, f32>) -> Self {
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

trait RaycastTarget {
    fn get_intersection(&self, ray: &Ray, bounds: Range<f32>) -> Option<IntersectionInfo>;
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

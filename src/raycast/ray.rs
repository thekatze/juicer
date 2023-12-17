use crate::{vector::Vector, shape::sphere::Sphere};

use super::raycast_target::RaycastTarget;

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


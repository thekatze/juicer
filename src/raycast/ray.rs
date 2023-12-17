use std::ops::Range;

use crate::{camera::World, vector::Vector};

pub struct Ray {
    pub start: Vector<3, f32>,
    pub direction: Vector<3, f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector<3, f32> {
        self.start.clone() + self.direction.clone() * t
    }

    pub fn color(&self, world: &World, bounds: &Range<f32>) -> Vector<3, f32> {
        if let Some(intersection) = world.get_intersection(self, bounds) {
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

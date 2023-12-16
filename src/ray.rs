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
        if let Some(sphere_intersection) = self.get_sphere_hit_distance(Vector([0.0, 0.0, -1.0]), 0.5) {
            let normal = (self.at(sphere_intersection) - Vector([0.0, 0.0, -1.0])).normalize();
            return Vector([normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0]) * 0.5;
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vector([1.0, 1.0, 1.0]) * (1.0 - a) + Vector([0.5, 0.7, 1.0]) * a
    }

    fn get_sphere_hit_distance(&self, center: Vector<3, f32>, radius: f32) -> Option<f32> {
        let sphere_direction = self.start.clone() - center;

        // solve using quadratic formula
        let a = self.direction.len_squared();
        let half_b = sphere_direction.dot(&self.direction);
        let c = sphere_direction.len_squared() - radius * radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-half_b - discriminant.sqrt()) / a)
        }
    }
}

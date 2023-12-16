use crate::vector::Vector;

pub struct Ray {
    pub start: Vector<3, f32>,
    pub direction: Vector<3, f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector<3, f32> {
        self.start.clone() + self.direction.clone() * t
    }

    /*
    vec3 unit_direction = unit_vector(r.direction());
    auto a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
    */
    pub fn color(&self) -> Vector<3, f32> {
        let sphere_intersection = self.get_sphere_hit_distance(Vector([0.0, 0.0, -1.0]), 0.5);

        if sphere_intersection > 0.0 {
            let normal = (self.at(sphere_intersection) - Vector([0.0, 0.0, -1.0])).normalize();
            return Vector([normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0]) * 0.5;
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y() + 1.0);

        Vector([1.0, 1.0, 1.0]) * (1.0 - a) + Vector([0.5, 0.7, 1.0]) * a
    }

    fn get_sphere_hit_distance(&self, center: Vector<3, f32>, radius: f32) -> f32 {
        let sphere_direction = self.start.clone() - center;

        // solve using quadratic formula
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * sphere_direction.dot(&self.direction);
        let c = sphere_direction.dot(&sphere_direction) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}

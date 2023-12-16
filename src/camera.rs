use crate::{vector::Vector, ray::Ray};

pub struct Camera {
    origin: Vector<3, f32>,
    pub image_size: Vector<2, usize>,
    viewport_size: Vector<2, f32>,
    focal_length: f32,
}

impl Camera {
    pub fn new(origin: Vector<3, f32>, image_width: usize, aspect_ratio: f32) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio).max(1.0) as usize;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        Camera {
            origin,
            image_size: Vector([image_width, image_height]),
            viewport_size: Vector([viewport_width, viewport_height]),
            focal_length: 1.0,
        }
    }

    pub fn rays<'a>(&'a self) -> impl Iterator<Item = Ray> + 'a {
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector([self.viewport_size.x(), 0.0, 0.0]);
        let viewport_v = Vector([0.0, -self.viewport_size.y(), 0.0]);

        let pixel_delta_u = viewport_u.clone() / self.image_size.x() as f32;
        let pixel_delta_v = viewport_v.clone() / self.image_size.y() as f32;

        let viewport_upper_left = self.origin.clone()
            - Vector([0.0, 0.0, self.focal_length])
            - viewport_u / 2.0
            - viewport_v / 2.0;

        let first_pixel_center =
            viewport_upper_left + pixel_delta_u.clone() / 2.0 + pixel_delta_v.clone() / 2.0;

        (0..self.image_size.y()).flat_map(move |y| {
            let pixel_delta_u = pixel_delta_u.clone();
            let pixel_delta_v = pixel_delta_v.clone();
            let first_pixel_center = first_pixel_center.clone();
            (0..self.image_size.x()).map(move |x| Ray {
                start: self.origin.clone(),
                direction: (first_pixel_center.clone()
                    + pixel_delta_u.clone() * x as f32
                    + pixel_delta_v.clone() * y as f32)
                    - self.origin.clone(),
            })
        })
    }
}


use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::ops::Range;

use crate::{
    image::Image,
    raycast::{intersection_info::IntersectionInfo, ray::Ray, raycast_target::RaycastTarget},
    vector::Vector,
};

pub struct Camera {
    origin: Vector<3, f32>,
    pub image_size: Vector<2, usize>,
    viewport_size: Vector<2, f32>,
    focal_length: f32,
    bounds: Range<f32>,
    samples: usize,
    max_bounces: usize,
}

impl Camera {
    pub fn new(
        origin: Vector<3, f32>,
        image_width: usize,
        aspect_ratio: f32,
        bounds: Range<f32>,
        samples: usize,
        max_bounces: usize,
    ) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio).max(1.0) as usize;

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

        Camera {
            origin,
            bounds,
            samples,
            image_size: Vector([image_width, image_height]),
            viewport_size: Vector([viewport_width, viewport_height]),
            focal_length: 1.0,
            max_bounces,
        }
    }

    pub fn rays(
        &'_ self,
    ) -> impl IndexedParallelIterator<Item = impl Iterator<Item = Ray> + '_> + '_ {
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

        (0..self.image_size.x() * self.image_size.y())
            .into_par_iter()
            .map(move |index| {
                let pixel_delta_u = pixel_delta_u.clone();
                let pixel_delta_v = pixel_delta_v.clone();
                let first_pixel_center = first_pixel_center.clone();

                let x = index % self.image_size.x();
                let y = index / self.image_size.x();

                let mut rng = fastrand::Rng::new();

                (0..self.samples).map(move |_| {
                    let offset_x = rng.f32() - 0.5;
                    let offset_y = rng.f32() - 0.5;

                    Ray {
                        start: self.origin.clone(),
                        direction: (first_pixel_center.clone()
                            + pixel_delta_u.clone() * (offset_x + x as f32)
                            + pixel_delta_v.clone() * (offset_y + y as f32))
                            - self.origin.clone(),
                    }
                })
            })
    }

    pub fn render(&self, world: &World) -> Image {
        let len = self.image_size.x() * self.image_size.y();

        let pixels = self
            .rays()
            .map(|pixel_rays| {
                pixel_rays
                    .map(|ray| ray.color(world, &self.bounds, self.max_bounces) * 255.9)
                    .sum::<Vector<3, f32>>()
                    / self.samples as f32
            })
            .map(|color| {
                Vector([
                    color.x().clamp(0.0, 255.0) as u8,
                    color.y().clamp(0.0, 255.0) as u8,
                    color.z().clamp(0.0, 255.0) as u8,
                ])
            })
            .progress_count(len as u64)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Image {
            width: self.image_size.x(),
            height: self.image_size.y(),
            pixels,
        }
    }
}

pub struct World {
    pub targets: Box<[RaycastTarget]>,
}

impl World {
    pub fn get_intersection(&self, ray: &Ray, bounds: &Range<f32>) -> Option<IntersectionInfo> {
        self.targets
            .iter()
            .filter_map(|target| target.get_intersection(ray, bounds))
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }
}

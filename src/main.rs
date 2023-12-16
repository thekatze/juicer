#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(iter_collect_into)]

use std::io::Write;

use ray::Ray;

use crate::{
    image::{formats::ppm::PPM, Image},
    vector::Vector,
};

mod image;
mod ray;
mod vector;

struct Camera {
    origin: Vector<3, f32>,
    image_size: Vector<2, usize>,
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

fn main() {
    let camera = Camera::new(Vector([0.0, 0.0, 0.0]), 1920, 16.0 / 9.0);
    let pixel_count = camera.image_size.x() * camera.image_size.y();
    let pixel_count_percentages = pixel_count / 100;

    let pixels = camera
        .rays()
        .enumerate()
        .inspect(|(index, _)| {
            if (index + 1) % pixel_count_percentages == 0 {
                println!("{}%", index / pixel_count_percentages);
            }
        })
        .map(|(_, ray)| ray.color() * 255.9)
        .map(|color| Vector([color.x() as u8, color.y() as u8, color.z() as u8]))
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let image = Image {
        width: camera.image_size.x(),
        height: camera.image_size.y(),
        pixels,
    };

    let bytes = image.serialize_to_bytes::<PPM>();

    let mut writer = std::io::BufWriter::new(
        std::fs::File::create("test.ppm").expect("file could not be created"),
    );
    writer.write_all(&bytes).expect("could not write to file");

    println!("Done");
}

#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(iter_collect_into)]

use std::io::Write;

use crate::{
    camera::Camera,
    image::{formats::ppm::PPM, Image},
    vector::Vector,
};

mod camera;
mod image;
mod raycast;
mod shape;
mod vector;

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

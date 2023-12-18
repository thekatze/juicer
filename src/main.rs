#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]

use std::io::Write;

use crate::{
    camera::{Camera, World},
    image::formats::ppm::Ppm,
    raycast::raycast_target::RaycastTarget,
    vector::Vector,
};

mod camera;
mod image;
mod raycast;
mod vector;

fn main() {
    let camera = Camera::new(
        Vector([0.0, 0.0, 0.0]),
        1920,
        16.0 / 9.0,
        0.0..f32::INFINITY,
        1,
        50,
    );

    let world = World {
        targets: vec![
            RaycastTarget::Sphere {
                origin: Vector([0.0, 0.0, -1.0]),
                radius: 0.5,
            },
            RaycastTarget::Sphere {
                origin: Vector([0.0, -100.5, -1.0]),
                radius: 100.0,
            },
        ]
        .into_boxed_slice(),
    };

    let image = camera.render(&world);

    let bytes = image.serialize_to_bytes::<Ppm>();

    let mut writer = std::io::BufWriter::new(
        std::fs::File::create("test.ppm").expect("file could not be created"),
    );
    writer.write_all(&bytes).expect("could not write to file");

    println!("Done");
}

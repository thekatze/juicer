use crate::{image::formats::ImageFormat, vector::Vector};

pub struct Ppm;
impl ImageFormat for Ppm {
    fn serialize_to_bytes(data: &[Vector<3, u8>], width: usize, height: usize) -> Box<[u8]> {
        let mut serialized = String::with_capacity(
            3 + // header P3
            10 + // header width and height
            4 + // header max color value
            height + // newlines after each row
            12 * width * height, // color values
        );
        let header = format!("P3\n{width} {height}\n255\n");
        serialized.push_str(&header);

        assert!(data.len() == width * height);

        for row in data.chunks(width) {
            for pixel in row {
                serialized.push_str(&format!(
                    "{r} {g} {b} ",
                    r = pixel.0[0],
                    g = pixel.0[1],
                    b = pixel.0[2]
                ));
            }
            serialized.push('\n');
        }

        serialized.as_bytes().into()
    }
}

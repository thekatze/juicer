use crate::Vector;

pub mod ppm;

pub trait ImageFormat {
    fn serialize_to_bytes(data: &[Vector<3, u8>], width: usize, height: usize) -> Box<[u8]>;
}


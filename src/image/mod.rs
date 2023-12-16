use crate::Vector;

use self::formats::ImageFormat;
pub mod formats;

impl Image {
    pub fn serialize_to_bytes<Format>(&self) -> Box<[u8]>
    where
        Format: ImageFormat,
    {
        Format::serialize_to_bytes(&self.pixels, self.width, self.height)
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Vector<3, u8>]>,
}

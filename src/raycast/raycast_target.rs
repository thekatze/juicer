use std::ops::Range;

use super::{ray::Ray, intersection_info::IntersectionInfo};

pub trait RaycastTarget {
    fn get_intersection(&self, ray: &Ray, bounds: Range<f32>) -> Option<IntersectionInfo>;
}

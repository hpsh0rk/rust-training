use image::ImageOutputFormat;

use crate::pb::Spec;

mod photon;
pub use photon::Photon;

pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}

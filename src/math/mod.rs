mod scalar;
mod vector2;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/length.md"))]
pub mod length;

pub use scalar::Scalar;
pub use vector2::Vector2;
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/line.md"))]
pub mod line;
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/circle.md"))]
pub mod circle;

mod shape;
mod shape_result;
mod shape_error;

pub use shape::Shape;
pub use shape_error::ShapeError;
pub use shape_result::ShapeResult;
pub use shape_result::ToShapeResult;

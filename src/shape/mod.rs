/// A geometric [circle](https://en.wikipedia.org/wiki/Circle).
pub mod circle;
mod shape;
mod shape_result;
mod shape_error;

pub use shape::Shape;
pub use shape_error::ShapeError;
pub use shape_result::ShapeResult;
pub use shape_result::ToShapeResult;

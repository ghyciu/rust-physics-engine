/// A geometric [circle](https://en.wikipedia.org/wiki/Circle).
pub mod circle;
mod shape;
mod shape_result;

pub use shape::Shape;
pub use shape_result::ShapeResult;
pub use shape_result::ShapeError;
pub use shape_result::ToShapeResult;

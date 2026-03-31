mod body;
mod world;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/rigidbody.md"))]
pub mod rigidbody;

pub use body::Body;
pub use world::World;
use crate::math::length::Length;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/line.md"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
  length: Length
}
use dpi::PhysicalPosition;

#[deprecated]
pub type Point = PhysicalPosition<i32>;

pub const ORIGIN_POINT: PhysicalPosition<i32> = PhysicalPosition::new(0, 0);

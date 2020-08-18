use na::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Screen {
    pub w: u32,
    pub h: u32,
    pub lower_left_corner: Point3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>,
}

use na::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub pos: Point3<f64>,
    pub lens_radius: f64,
}

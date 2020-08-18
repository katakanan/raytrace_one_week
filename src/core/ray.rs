use na::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vector3<f64>) -> Ray {
        Ray {
            origin: orig,
            dir: dir.normalize(),
        }
    }

    pub fn point_at_paramete(&self, t: f64) -> Point3<f64> {
        self.origin + self.dir * t
    }
}

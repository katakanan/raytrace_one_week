use na::{Point3, Vector3};

use super::camera::Camera;
use super::color::Color;
use super::ray::Ray;
use super::screen::Screen;
use super::shape_trait::HIT;
use super::sphere::Sphere;

#[derive(Debug, Clone)]
pub struct Scene {
    pub screen: Screen,
    pub camera: Camera,
    pub shapes: Sphere,
}

impl Scene {
    pub fn new(w: u32, h: u32) -> Scene {
        let lower_left_corner = Point3::new(-2.0, -1.0, -1.0);
        let horizontal = Vector3::new(4.0, 0.0, 0.0);
        let vertical = Vector3::new(0.0, 2.0, 0.0);

        let screen = Screen {
            w,
            h,
            lower_left_corner,
            horizontal,
            vertical,
        };

        let camera = Camera {
            pos: Point3::origin(),
        };

        let sphere = Sphere {
            center: Point3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        };

        Scene {
            screen,
            camera,
            shapes: sphere,
        }
    }

    pub fn color(&self, r: &Ray) -> Color {
        match self.shapes.hit(&r, 0.0, 0.0) {
            Some(hr) => {
                let c = (hr.n + Vector3::new(1.0, 1.0, 1.0)) * 0.5;
                Color::new(c.x, c.y, c.z)
            }
            None => {
                let unit_direction = r.dir / r.dir.norm();
                let t = 0.5 * (unit_direction.y + 1.0);
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            }
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rdir = self.screen.lower_left_corner - self.camera.pos
            + (self.screen.horizontal * u + self.screen.vertical * v);

        Ray::new(self.camera.pos, rdir)
    }
}

extern crate image;

use image::{Rgba, RgbaImage};

pub fn render(w: u32, h: u32) -> RgbaImage {
    let mut img = RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let ny = h - y - 1;
            let nx = x;

            let r = (nx as f64) / (w as f64);
            let g = (ny as f64) / (h as f64);
            let b = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            img.put_pixel(x, y, Rgba([ir, ig, ib, 255]))
        }
    }
    img
}

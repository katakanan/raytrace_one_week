extern crate image;
extern crate renderer;

use image::ImageFormat;

fn main() {
    println!("Hello World");
    let w = 200;
    let h = 100;
    let thread = 1;

    let _ = renderer::render(w, h, thread)
        .save_with_format(std::path::Path::new("test.png"), ImageFormat::Png)
        .unwrap();
}

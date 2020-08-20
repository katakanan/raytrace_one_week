extern crate image;
extern crate renderer;

use image::ImageFormat;

use std::env;

fn get_arg_if_exist(args: &Vec<String>, argnum: usize) -> u32 {
    let mut tmp = if args.len() > argnum {
        let num = &args[argnum];
        let number: u32 = match num.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error : {} argument is not an integer", argnum);
                1
            }
        };
        number
    } else {
        1
    };

    tmp = if tmp == 0 { 1 } else { tmp };

    tmp
}

fn main() {
    println!("Hello World");

    let args: Vec<String> = env::args().collect();
    let w = get_arg_if_exist(&args, 1);
    let h = get_arg_if_exist(&args, 2);
    let thread = get_arg_if_exist(&args, 3);
    let sample = get_arg_if_exist(&args, 4);

    println!("{}x{}", w, h);
    println!("thread\t: {}", thread);
    println!("ss\t: {}", sample);
    let _ = renderer::render(w, h, thread, sample)
        .save_with_format(std::path::Path::new("test.png"), ImageFormat::Png)
        .unwrap();
}

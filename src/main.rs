use image::{ImageBuffer, RgbImage};
use std::env;
use std::fs::create_dir_all;

fn parse_args(args: Vec<String>, width: &mut i32, height: &mut i32, max_iteration: &mut i32) {
    match args.len() {
        3 => {
            *width = match args[1].parse() {
                Ok(val) => val,
                Err(_) => {
                    panic!("error: width argument not an integer");
                }
            };
            *height = match args[2].parse() {
                Ok(val) => val,
                Err(_) => {
                    panic!("error: height argument not an integer");
                }
            };
            *max_iteration = 1000;
        }
        4 => {
            *width = match args[1].parse() {
                Ok(val) => val,
                Err(_) => {
                    panic!("error: width argument not an integer");
                }
            };
            *height = match args[2].parse() {
                Ok(val) => val,
                Err(_) => {
                    panic!("error: height argument not an integer");
                }
            };
            *max_iteration = match args[3].parse() {
                Ok(val) => val,
                Err(_) => {
                    panic!("error: max_iteration argument not an integer");
                }
            };
        }
        _ => {}
    }
}

fn main() {
    let mut width: i32 = 1920;
    let mut height: i32 = 1080;
    let mut max_iteration: i32 = 1000;

    let args: Vec<String> = env::args().collect();

    parse_args(args, &mut width, &mut height, &mut max_iteration);

    let mut image_buffer: RgbImage = ImageBuffer::new(width as u32, height as u32);
    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let r = (0.017 * x as f32) as u8;
        let b = (0.017 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    for row in 0i32..height {
        for col in 0i32..width {
            let re = (col - width / 2) as f64 * 4. / width as f64;
            let im = (row - height / 2) as f64 * 4. / width as f64;
            let mut x = 0.;
            let mut y = 0.;
            let mut w = 0.;
            let mut iteration = 0;

            while x + y <= 4. && iteration < max_iteration {
                let x_new = x - y + re;
                let y_new = w - x - y + im;
                y = y_new * y_new;
                x = x_new * x_new;
                w = (x_new + y_new) * (x_new + y_new);
                iteration += 1;
            }

            let pixel = image_buffer.get_pixel_mut(col as u32, row as u32);
            if iteration < max_iteration {
                let image::Rgb(data) = *pixel;
                *pixel = image::Rgb([data[0], iteration as u8, data[2]]);
            } else {
                *pixel = image::Rgb([0, 0, 0]);
            }
        }
    }

    create_dir_all("./out").expect("can not create output directory");
    image_buffer.save("./out/mandelbrot-set.png").unwrap();
}

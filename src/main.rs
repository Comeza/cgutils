extern crate image;

mod dimension;
mod img;
mod write_direction;

use clap::{Clap, ValueHint};
use dimension::Dimensions;
use image::GenericImageView;
use image::{ImageBuffer, ImageFormat};
use img::{process_images, save_image_buffer};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use write_direction::WriteDirection;

#[derive(Clap, Debug)]
#[clap(name = "image-stitch", version = "0.2.0")]
pub struct Opt {
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = "output/output.png")]
    pub output: PathBuf,

    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::FilePath, default_value = ".")]
    pub input: PathBuf,

    #[clap(short, long)]
    pub max: Option<u32>,

    #[clap(short, long, default_value = "X")]
    pub direction: WriteDirection,
}

fn main() {
    let opt = Opt::parse();
    let global_instant = Instant::now();
    let files = &mut index_images(fs::read_dir(opt.input).unwrap());

    alphanumeric_sort::sort_path_slice(files);

    let imgs_dims =
        Dimensions::from_tuple(image::open(files.first().unwrap()).unwrap().dimensions());
    let max_length = opt.max.unwrap_or(1);

    let (x, y) = match opt.direction {
        WriteDirection::X => {
            let x_count = max_length / imgs_dims.x;
            let y_count = (files.len() as f32 / x_count as f32).ceil() as u32;

            (x_count * imgs_dims.x, y_count * imgs_dims.y)
        }
        WriteDirection::Y => {
            let y_count = max_length / imgs_dims.y;
            let x_count = (files.len() as f32 / y_count as f32).ceil() as u32;

            (x_count * imgs_dims.x, y_count * imgs_dims.y)
        }
    };

    println!("Images: {}", files.len());
    println!("Row Length: {}", opt.max.unwrap_or(1));
    println!("Buffer Dimensions: {}x{}", x, y);

    let mut image_buffer = ImageBuffer::new(x, y);
    process_images(files, imgs_dims, &mut image_buffer, opt.direction);
    save_image_buffer(&opt.output, image_buffer, ImageFormat::Png);

    println!("finished process in {:?}", global_instant.elapsed())
}

fn index_images(dir: std::fs::ReadDir) -> Vec<PathBuf> {
    let mut file_list: Vec<PathBuf> = vec![];
    for file in dir {
        match file {
            Ok(file) => {
                let path = file.path();
                match ImageFormat::from_path(path.as_path()) {
                    Ok(p) => {
                        if p.can_read() {
                            file_list.push(path);
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }

    return file_list;
}

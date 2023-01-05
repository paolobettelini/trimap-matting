use image::{DynamicImage, Rgba};
use opencv::{core::{Vector, ToInputArray}, imgcodecs::*, prelude::*};
use std::{error::Error, fs};

mod args;
use args::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let target_path = path_str!(args.target);
    let trimap_path = path_str!(args.trimap);
    let output_path = path_str!(args.output);

    let target = imread(&target_path, IMREAD_COLOR)?;
    let trimap = imread(&trimap_path, IMREAD_GRAYSCALE)?;
    let mut output = trimap.clone();

    let res = opencv::alphamat::info_flow(&target, &trimap, &mut output);

    let mut imwrite_flags = Vector::new();
    let res = imwrite(&output_path, &output, &imwrite_flags);

    if let Ok(success) = res {
        println!(
            "Status: {}",
            if !success || res.is_err() {
                false
            } else {
                success
            }
        );
    }

    let mask: DynamicImage = opencv_to_dynamic_image_gray(output);
    let target: DynamicImage = opencv_to_dynamic_image(target);

    let result = remove_background(target, mask);

    result.save("/home/paolo/Scrivania/trans.png")?;

    Ok(())
}

use std::io::Cursor;
use image::RgbImage;
use std::io::Read;

fn opencv_to_dynamic_image_gray(mat: Mat) -> DynamicImage {
    let w = mat.cols();
    let h = mat.rows();
    let mut rgbim = RgbImage::new(w as u32, h as u32);

    let data = mat.data_bytes().unwrap();
        let w = rgbim.width();
        for (pixi, i) in (0..data.len()).enumerate() {
            let b = data[i];
            let g = data[i];
            let r = data[i];
            let impix = image::Rgb([r, g, b]);
            let x = pixi as u32 % w;
            let y = pixi as u32 / w;
            rgbim.put_pixel(x, y, impix);
        }
    let im = DynamicImage::ImageRgb8(rgbim);
    im
}

fn opencv_to_dynamic_image(mat: Mat) -> DynamicImage {
    let w = mat.cols();
    let h = mat.rows();
    let mut rgbim = RgbImage::new(w as u32, h as u32);

    let data = mat.data_bytes().unwrap();
        let w = rgbim.width();
        for (pixi, i) in (0..data.len()).step_by(3).enumerate() {
            let b = data[i];
            let g = data[i + 1];
            let r = data[i + 2];
            let impix = image::Rgb([r, g, b]);
            let x = pixi as u32 % w;
            let y = pixi as u32 / w;
            rgbim.put_pixel(x, y, impix);
        }
    let im = DynamicImage::ImageRgb8(rgbim);
    im
}

use image::GenericImage;

fn remove_background(image: DynamicImage, mask: DynamicImage) -> DynamicImage {
    let mask = mask.as_rgb8().unwrap();
    let image = image.as_rgb8().unwrap();

    let (width, height) = image.dimensions();
    let mut out = DynamicImage::new_rgba8(width, height);

    for x in 0..width {
        for y in 0..height {
            let mask_pixel = mask.get_pixel(x, y);
            let image_pixel = image.get_pixel(x, y);

            let mut new_pixel = Rgba([0, 0, 0, 0]);
            for i in 0..3 {
                new_pixel[i] = ((mask_pixel[0] as u16 * image_pixel[i] as u16) / 255u16) as u8;
            }
            new_pixel[3] = mask_pixel[0];
            //new_pixel[3] = 0;

            out.put_pixel(x, y, new_pixel);
        }
    }

    out
}


// vtk opencv clang qt5-base cmake

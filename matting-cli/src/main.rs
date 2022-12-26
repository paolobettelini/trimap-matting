use image::DynamicImage;
use opencv::{core::Vector, imgcodecs::*, prelude::*};
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

    

    Ok(())
}

use image::{GrayImage, Rgb, RgbImage};
use std::collections::HashMap;

fn trimap_matting(image: &RgbImage, trimap: &GrayImage) -> RgbImage {
    todo!()
}

// opencv clang qt5-base cmake

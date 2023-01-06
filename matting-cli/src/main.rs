use image::{DynamicImage, GenericImage, RgbImage, Rgba};
use opencv::{core::Vector, imgcodecs::*, prelude::*};
use std::error::Error;

mod args;
use args::*;

// pacman -S vtk opencv clang qt5-base cmake

fn main() {
    let args = Args::parse();

    let target_path = path_str!(args.target);

    let mask = if let Some(path) = args.trimap {
        // Trimap is given
        let target = imread(&target_path, IMREAD_COLOR).unwrap();
        let trimap_path = path_str!(path);
        let trimap = imread(&trimap_path, IMREAD_GRAYSCALE).unwrap();

        let output = generate_mask(&target, &trimap);

        mat_to_dynamic_image_gray(&output)
    } else {
        // Mask is given
        let mask_path = path_str!(args.mask.unwrap());
        image::open(mask_path).unwrap()
    };

    let target = image::open(target_path).unwrap();

    // Save mask option
    if let Some(path) = args.save_mask {
        mask.save(path_str!(path)).unwrap();
    }

    // Action on image option
    if let Some(path) = args.output {
        let result: DynamicImage = if args.transparent {
            // Transparent background option

            remove_background(&target, &mask)
        } else if let Some(color) = args.fill {
            // Fill color option

            let color = csscolorparser::parse(&color).unwrap();
            fill_background(&target, &mask, color.to_rgba8())
        } else if let Some(path) = args.replace {
            // Replace background option

            let replacement_path = path_str!(path);
            let replacement = image::open(replacement_path).unwrap();

            replace_background(&target, &mask, &replacement)
        } else {
            panic!("Argument parsing is corrupt and you should've never seen this message");
        };

        let output_path = path_str!(path);
        result.save(output_path).unwrap();
    }
}

fn generate_mask(target: &Mat, trimap: &Mat) -> Mat {
    let mut output = trimap.clone();

    opencv::alphamat::info_flow(&target, &trimap, &mut output).unwrap();

    output
}

fn mat_to_dynamic_image_gray(mat: &Mat) -> DynamicImage {
    let w = mat.cols();
    let h = mat.rows();
    let mut rgbim = RgbImage::new(w as u32, h as u32);

    let data = mat.data_bytes().unwrap();
    let w = rgbim.width();
    for (pixel, i) in (0..data.len()).enumerate() {
        let r @ g @ b = data[i];
        let impix = image::Rgb([r, g, b]);
        let x = pixel as u32 % w;
        let y = pixel as u32 / w;
        rgbim.put_pixel(x, y, impix);
    }

    DynamicImage::ImageRgb8(rgbim)
}

fn remove_background(image: &DynamicImage, mask: &DynamicImage) -> DynamicImage {
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

            out.put_pixel(x, y, new_pixel);
        }
    }

    out
}

fn replace_background(
    image: &DynamicImage,
    mask: &DynamicImage,
    replacement: &DynamicImage,
) -> DynamicImage {
    let mask = mask.as_rgb8().unwrap();
    let image = image.as_rgb8().unwrap();
    let replacement = replacement.as_rgb8().unwrap();

    let (width, height) = image.dimensions();
    let mut out = DynamicImage::new_rgba8(width, height);

    for x in 0..width {
        for y in 0..height {
            let mask_pixel = mask.get_pixel(x, y);
            let image_pixel = image.get_pixel(x, y);
            let replacement_pixel = replacement.get_pixel(x, y);

            let mut new_pixel = Rgba([0, 0, 0, 0]);
            for i in 0..3 {
                new_pixel[i] = ((mask_pixel[0] as u16 * image_pixel[i] as u16) / 255u16
                    + ((255u16 - mask_pixel[0] as u16) * replacement_pixel[i] as u16) / 255u16)
                    as u8;
            }
            new_pixel[3] = 255u8;

            out.put_pixel(x, y, new_pixel);
        }
    }

    out
}

fn fill_background(image: &DynamicImage, mask: &DynamicImage, color: [u8; 4]) -> DynamicImage {
    let mask = mask.as_rgb8().unwrap();
    let image = image.as_rgb8().unwrap();

    let (width, height) = image.dimensions();
    let mut out = DynamicImage::new_rgba8(width, height);

    let replacement_pixel = Rgba(color);

    for x in 0..width {
        for y in 0..height {
            let mask_pixel = mask.get_pixel(x, y);
            let image_pixel = image.get_pixel(x, y);

            let mut new_pixel = Rgba([0, 0, 0, 0]);
            for i in 0..3 {
                new_pixel[i] = ((mask_pixel[0] as u16 * image_pixel[i] as u16) / 255u16
                    + ((255u16 - mask_pixel[0] as u16) * replacement_pixel[i] as u16) / 255u16)
                    as u8;
            }
            // Add opacities
            let v = mask_pixel[0] as u16 + replacement_pixel[3] as u16;
            new_pixel[3] = if v > 255 { 255 } else { v as u8 };

            out.put_pixel(x, y, new_pixel);
        }
    }

    out
}


/*
fn opencv_to_dynamic_image(mat: &Mat) -> DynamicImage {
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
}*/
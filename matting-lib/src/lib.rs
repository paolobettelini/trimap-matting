use image::{GenericImage, RgbImage, Rgba};
use opencv::{core::Vector, prelude::*};

// Export imports
pub use image::{ImageFormat, DynamicImage};
pub use opencv::imgcodecs::*;

pub mod error;

use error::*;

pub fn generate_mask(target: &Mat, trimap: &Mat) -> MessageResult<Mat> {
    let mut output = trimap.clone();

    opencv::alphamat::info_flow(&target, &trimap, &mut output)?;

    Ok(output)
}

pub fn bytes_to_mat(data: &[u8], flags: i32) -> MessageResult<Mat> {
    Ok(imdecode(&Vector::from_slice(data), flags)?)
}

pub fn read_as_mat(filename: &str, flags: i32) -> MessageResult<Mat> {
    Ok(imread(filename, flags)?)
}

pub fn read_as_image(filename: &str) -> MessageResult<DynamicImage> {
    Ok(image::open(filename)?)
}

pub fn bytes_to_image(data: &[u8]) -> MessageResult<DynamicImage> {
    Ok(image::load_from_memory(data)?)
}

pub fn mat_to_dynamic_image_gray(mat: &Mat) -> MessageResult<DynamicImage> {
    let w = mat.cols();
    let h = mat.rows();
    let mut rgbim = RgbImage::new(w as u32, h as u32);

    let data = mat.data_bytes()?;
    let w = rgbim.width();
    for (pixel, i) in (0..data.len()).enumerate() {
        let r @ g @ b = data[i];
        let impix = image::Rgb([r, g, b]);
        let x = pixel as u32 % w;
        let y = pixel as u32 / w;
        rgbim.put_pixel(x, y, impix);
    }

    Ok(DynamicImage::ImageRgb8(rgbim))
}

// .as_rgb8() -> pointer as rgb8
// .to_rgb8() -> copy of image
// .into_rgb8() -> pointer or copy is it not the correct format

pub fn remove_background(image: &RgbImage, mask: &RgbImage) -> DynamicImage {
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

pub fn replace_background(
    image: &RgbImage,
    mask: &RgbImage,
    replacement: &RgbImage,
) -> DynamicImage {
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

pub fn fill_background(
    image: &RgbImage,
    mask: &RgbImage,
    color: [u8; 4],
) -> DynamicImage {
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


pub fn image_to_format(image: DynamicImage, format: ImageFormat) -> Vec<u8> {
    use std::io::{Cursor, Read, Seek, SeekFrom};

    let color = image.color();
    let width = image.width();
    let height = image.height();

    // Implements `Seek` and `Write`
    let mut cursor = Cursor::new(Vec::new());

    let res = image::write_buffer_with_format(
        &mut cursor,
        &mut image.into_bytes(),
        width,
        height,
        color,
        format,
    );

    if res.is_err() {
        return vec![];
    }

    // Read result
    cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = Vec::new();
    cursor.read_to_end(&mut buffer).unwrap();

    buffer
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

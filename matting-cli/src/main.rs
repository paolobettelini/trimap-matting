use image::{DynamicImage, GenericImage, RgbImage, Rgba};
use opencv::{core::Vector, imgcodecs::*, prelude::*};

use matting_lib as matting;

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

        let output = matting::generate_mask(&target, &trimap);

        matting::mat_to_dynamic_image_gray(&output)
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

            matting::remove_background(&target, &mask)
        } else if let Some(color) = args.fill {
            // Fill color option

            let color = csscolorparser::parse(&color).unwrap();
            matting::fill_background(&target, &mask, color.to_rgba8())
        } else if let Some(path) = args.replace {
            // Replace background option

            let replacement_path = path_str!(path);
            let replacement = image::open(replacement_path).unwrap();

            matting::replace_background(&target, &mask, &replacement)
        } else {
            panic!("Argument parsing is corrupt and you should've never seen this message");
        };

        let output_path = path_str!(path);
        result.save(output_path).unwrap();
    }
}
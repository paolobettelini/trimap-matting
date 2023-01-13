use matting::{error::*, *};
use matting_lib as matting;

mod args;
mod helpers;

use args::*;
use helpers::*;

// pacman -S vtk opencv clang qt5-base cmake

fn main() {
    let args = Args::parse();

    let result = consume(args);

    if let Err(error) = result {
        println!("An error occured: {}", error.message);
    }
}

fn consume(args: Args) -> MessageResult<()> {
    let target_path = path_str!(args.target);

    let mask = if let Some(path) = args.trimap {
        // Trimap is given
        let target = log!(
            "Reading target image",
            args.verbose,
            matting::read_as_mat(&target_path, IMREAD_COLOR)?
        );

        let trimap_path = path_str!(path);

        let trimap = log!(
            "Reading trimap image",
            args.verbose,
            matting::read_as_mat(&trimap_path, IMREAD_GRAYSCALE)?
        );

        let output = log!(
            "Generating soft mask",
            args.verbose,
            matting::generate_mask(&target, &trimap)?
        );

        matting::mat_to_dynamic_image_gray(&output)?
    } else {
        // Mask is given
        if let Some(mask_path) = args.mask {
            let mask_path = path_str!(mask_path);

            log!(
                "Reading soft mask image",
                args.verbose,
                matting::read_as_image(&mask_path)?
            )
        } else {
            return error!("Argument parsing is corrupt and you should've never seen this message");
        }
    };

    let target = log!(
        "Reading target image",
        args.verbose,
        matting::read_as_image(&target_path)?
    );

    // Save mask option
    if let Some(path) = args.save_mask {
        log!(
            "Saving soft mask",
            args.verbose,
            mask.save(path_str!(path))?
        );
    }

    // Action on image option
    if let Some(path) = args.output {
        let result: DynamicImage = if args.transparent {
            // Transparent background option

            log!(
                "Removing background",
                args.verbose,
                matting::remove_background(&target, &mask)?
            )
        } else if let Some(color) = args.fill {
            // Fill color option

            log!("Filling background with color", args.verbose, {
                let color = csscolorparser::parse(&color).convert()?;
                matting::fill_background(&target, &mask, color.to_rgba8())?
            })
        } else if let Some(path) = args.replace {
            // Replace background option

            let replacement_path = path_str!(path);
            let replacement = log!(
                "Reading replacement image",
                args.verbose,
                matting::read_as_image(&replacement_path)?
            );

            log!(
                "Replacing background",
                args.verbose,
                matting::replace_background(&target, &mask, &replacement)?
            )
        } else {
            return error!("Argument parsing is corrupt and you should've never seen this message");
        };

        let output_path = path_str!(path);
        log!("Saving output", args.verbose, result.save(output_path)?);
    }

    Ok(())
}

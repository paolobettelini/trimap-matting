#![feature(path_file_prefix)]

use matting::{error::*, *};
use matting_lib as matting;

use std::{borrow::Cow, path::Path, time::SystemTime};
use chrono::{DateTime, offset::Local};

mod args;
mod helpers;

use args::*;
use helpers::*;

// pacman -S vtk opencv clang qt5-base cmake

const TIMESTAMP: &str = "%Y%m%d-%H%M%S";

const LOG_ENV: &str = "MATTING_LOG";
const LOG_ENV_STYLE: &str = "MATTING_LOG_STYLE";

fn main() {
    let args = Args::parse();

    if args.verbose {
        std::env::set_var(LOG_ENV, "info");
    } else if std::env::var(LOG_ENV).is_err() {
        std::env::set_var(LOG_ENV, "warn");
    }

    let env = env_logger::Env::new()
        .filter(LOG_ENV)
        .write_style(LOG_ENV_STYLE);
    env_logger::init_from_env(env);

    let result = consume(args);

    if let Err(error) = result {
        error!("An error occured: {}", error.message);
    }
}

fn consume(args: Args) -> MessageResult<()> {
    let target_path = path_str!(args.target);

    let mask = if let Some(path) = args.trimap {
        // Trimap is given
        let target = log!(
            "Reading target image",
            matting::read_as_mat(&target_path, IMREAD_COLOR)?
        );

        let trimap_path = path_str!(path);

        let trimap = log!(
            "Reading trimap image",
            matting::read_as_mat(&trimap_path, IMREAD_GRAYSCALE)?
        );

        // Check trimap and target sizes
        if !same_size(&trimap_path, &target_path)? {
            return msgerror!("The trimap and target image must be of the same size.");
        }

        let output = log!(
            "Generating soft mask",
            matting::generate_mask(&target, &trimap)?
        );

        matting::mat_to_dynamic_image_gray(&output)?
    } else {
        // Mask is given
        if let Some(mask_path) = args.mask {
            let mask_path = path_str!(mask_path);

            if !same_size(&mask_path, &target_path)? {
                return msgerror!("The mask and target image must be of the same size.");
            }

            log!(
                "Reading soft mask image",
                matting::read_as_image(&mask_path)?
            )
        } else {
            return msgerror!("Argument parsing is corrupt and you should've never seen this message");
        }
    };

    let target = log!(
        "Reading target image",
        matting::read_as_image(&target_path)?
    );

    // Save mask option
    if let Some(parameter) = args.save_mask {
        let output_path = if let Some(path) = parameter {
            path_str!(path)
        } else {
            let timestamp = get_timestamp();

            format!("mask_{timestamp}.png")
        };
        
        log!(
            format!("Saving soft mask to {output_path}"),
            mask.save(output_path)?
        );
    }

    // Apply action

    let result: DynamicImage = if args.transparent {
        // Transparent background option

        log!(
            "Removing background",
            matting::remove_background(&target.into_rgb8(), &mask.into_rgb8())
        )
    } else if let Some(color) = args.fill {
        // Fill color option

        let color = csscolorparser::parse(&color).convert()?;

        log!("Filling background with color",
            matting::fill_background(&target.into_rgb8(), &mask.into_rgb8(), color.to_rgba8())
        )
    } else if let Some(path) = args.replace {
        // Replace background option

        let replacement_path = path_str!(path);

        // Check target and replacement sizes
        if !same_size(&replacement_path, &target_path)? {
            return msgerror!("The replacement and target image must be of the same size.");
        }

        let replacement = log!(
            "Reading replacement image",
            matting::read_as_image(&replacement_path)?
        );

        log!(
            "Replacing background",
            matting::replace_background(&target.into_rgb8(), &mask.into_rgb8(), &replacement.into_rgb8())
        )
    } else {
        return Ok(()); // No action
    };

    let output_path = if let Some(path) = args.output {
        path_str!(path)
    } else {
        // Construct [input]_[timestamp_iso][.ext]
        let path = Path::new(&target_path);
        
        let filename = path.file_prefix()
            .unwrap() // at this point we know there is a filename
            .to_string_lossy();

        let ext = if args.transparent {
            Cow::Owned("png".to_owned()) // Default format for transparent images  
        } else {
            path.extension()
                .unwrap() // at this point we know there is an extension
                .to_string_lossy()
        };
        
        let timestamp = get_timestamp();

        format!("{filename}_{timestamp}.{ext}")
    };

    // Notify if there is transparency and it's not a png
    if args.transparent && !output_path.ends_with(".png") {
        warn!("The image has an alpha channel but the format is not PNG.");
        warn!("It is advised to change the output format to \".png\"");
    }

    log!(format!("Saving output to {output_path}"), result.save(output_path)?);

    Ok(())
}

fn get_timestamp() -> String {
    let sys_time = SystemTime::now();
    let datetime: DateTime<Local> = sys_time.into();
    datetime.format(TIMESTAMP).to_string()
}
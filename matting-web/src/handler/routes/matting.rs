use futures::stream::TryStreamExt;
use log::{error, info, warn};
use matting::*;
use matting_lib as matting;
use std::{
    io::{Read, Write},
    process::Command,
};
use tempfile::Builder;
use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    Buf,
};

use crate::handler::routes::macros::*;

pub async fn matting(form: FormData) -> Response<Vec<u8>> {
    let parts = read_parts!(form);

    let mut target: Option<Part> = None;
    let mut trimap: Option<Part> = None;

    for part in parts {
        match part.name() {
            "target" => target = Some(part),
            "trimap" => trimap = Some(part),
            _ => {}
        }
    }

    let mut target_part = read_part!(target);
    let mut trimap_part = read_part!(trimap);

    let bytes = if cfg!(feature = "no_matting_cli") {
        // Conditional compilation - if no_matting_cli is set
        // directly call the library (core dump risk)

        let target = part_to_mat!(target_part, IMREAD_COLOR);
        let trimap = part_to_mat!(trimap_part, IMREAD_GRAYSCALE);

        let mask = {
            info!("Calling matting library");

            let output = if let Ok(out) = matting::generate_mask(&target, &trimap) {
                out
            } else {
                error!("Could not generate mask");
                return internal_error!();
            };

            if let Ok(v) = matting::mat_to_dynamic_image_gray(&output) {
                v
            } else {
                error!("Could not convert mat to gray image");
                return internal_error!();
            }
        };

        matting::image_to_format(mask, matting::ImageFormat::Png)
    } else {
        // Conditional compilation - if no_matting_cli is not set
        // call the 'matting' CLI.

        let target = part_to_file!(target_part);
        let trimap = part_to_file!(trimap_part);
        let result = png_tempfile!();

        let mut result_file = if let Ok(file) = result.reopen() {
            file
        } else {
            error!("Could not reopen temp file");
            return internal_error!();
        };

        let target_path = &target.into_temp_path();
        let trimap_path = &trimap.into_temp_path();
        let result_path = &result.into_temp_path();

        let target_path_abs = get_owned_abs!(target_path);
        let trimap_path_abs = get_owned_abs!(trimap_path);
        let result_path_abs = get_owned_abs!(result_path);

        info!("Calling matting CLI");

        let res = Command::new("matting-cli")
            .args([
                "--target",
                &target_path_abs,
                "--trimap",
                &trimap_path_abs,
                "--save-mask",
                &result_path_abs,
            ])
            .output();

        if res.is_err() {
            error!("Could not call matting-cli");
            return internal_error!();
        }

        let metadata = if let Ok(data) = std::fs::metadata(&result_path_abs) {
            data
        } else {
            error!("Could not read metadata of file");
            return internal_error!();
        };
        let mut buffer = vec![0; metadata.len() as usize];
        if result_file.read(&mut buffer).is_err() {
            error!("Could not read data from file");
            return internal_error!();
        }

        buffer
    };

    image_response!(bytes)
}

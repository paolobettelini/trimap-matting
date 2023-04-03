use futures::stream::TryStreamExt;
use log::{error, warn};

use matting_lib as matting;

use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    Buf,
};

use crate::handler::routes::macros::*;

pub async fn transparent(form: FormData) -> Response<Vec<u8>> {
    let parts = read_parts!(form);

    let mut target: Option<Part> = None;
    let mut mask: Option<Part> = None;

    for part in parts {
        match part.name() {
            "target" => target = Some(part),
            "mask" => mask = Some(part),
            _ => {}
        }
    }

    let mut target_part = read_part!(target);
    let mut mask_part = read_part!(mask);

    let mask = part_to_image!(mask_part);
    let target = part_to_image!(target_part);

    assert_same_size!(mask, target);

    // Remove background action
    let output = matting::remove_background(&target, &mask);

    let bytes = matting::image_to_format(output, matting::ImageFormat::Png);

    image_response!(bytes)
}

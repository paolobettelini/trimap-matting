use futures::stream::TryStreamExt;
use matting::*;
use matting_lib as matting;
use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    Buf, Filter, Rejection, Reply,
};

#[allow(opaque_hidden_inferred_bound)]
pub fn get_routes(
    www: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let static_files = warp::fs::dir(www);

    // Useful macros

    macro_rules! bad_request {
        () => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(vec![])
                .unwrap()
        };
    }

    macro_rules! image_response {
        ($bytes:tt) => {
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/*")
                .body($bytes)
                .unwrap()
        };
    }

    macro_rules! read_parts {
        ($form:tt) => {{
            let parts: Result<Vec<Part>, _> = $form.try_collect().await;

            if let Ok(data) = parts {
                data
            } else {
                return bad_request!();
            }
        }};
    }

    macro_rules! read_part {
        ($option:tt) => {
            if let Some(part) = $option {
                part
            } else {
                return bad_request!();
            }
        };
    }

    macro_rules! part_to_image {
        ($part:tt) => {{
            let buf = $part.data().await.unwrap().unwrap();
            let data = buf.chunk();
            matting::bytes_to_image(&data).unwrap().into_rgb8()
        }};
    }

    macro_rules! part_to_mat {
        ($part:tt, $prop:tt) => {{
            let buf = $part.data().await.unwrap().unwrap();
            let data = buf.chunk();
            //std::fs::write(format!("/home/paolo/Desktop/{}.png", stringify!($part)), data);
            matting::bytes_to_mat(&data, $prop).unwrap()
        }};
    }

    macro_rules! assert_same_size {
        ($img1:tt, $img2:tt) => {
            if $img1.dimensions() != $img2.dimensions() {
                return bad_request!();
            }
        };
    }

    // target and mask need to be RgbImage
    // for image_to_format

    let matting_api = warp::path!("api" / "matting")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            println!("Received matting request");

            let parts = read_parts!(form);

            let mut target: Option<Part> = None;
            let mut trimap: Option<Part> = None;

            for part in parts {
                if part.name() == "target" {
                    target = Some(part);
                } else if part.name() == "trimap" {
                    trimap = Some(part);
                }
            }

            let mut target_part = read_part!(target);
            let mut trimap_part = read_part!(trimap);

            let target = part_to_mat!(target_part, IMREAD_COLOR);
            let trimap = part_to_mat!(trimap_part, IMREAD_GRAYSCALE);

            let mask = {
                let output = matting::generate_mask(&target, &trimap).unwrap();

                matting::mat_to_dynamic_image_gray(&output).unwrap()
            };

            let bytes = matting::image_to_format(mask, matting::ImageFormat::Png);

            image_response!(bytes)
        });

    let transparent_api = warp::path!("api" / "transparent")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            println!("Received transparent request");

            let parts = read_parts!(form);

            let mut target: Option<Part> = None;
            let mut mask: Option<Part> = None;

            for part in parts {
                if part.name() == "target" {
                    target = Some(part);
                } else if part.name() == "mask" {
                    mask = Some(part);
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
        });

    let fill_api = warp::path!("api" / "fill" / String)
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|color: String, form: FormData| async move {
            println!("Received fill request");

            let color = format!("#{color}");

            let parts = read_parts!(form);

            let mut target: Option<Part> = None;
            let mut mask: Option<Part> = None;

            for part in parts {
                if part.name() == "target" {
                    target = Some(part);
                } else if part.name() == "mask" {
                    mask = Some(part);
                }
            }

            let mut target_part = read_part!(target);
            let mut mask_part = read_part!(mask);

            let target = part_to_image!(target_part);
            let mask = part_to_image!(mask_part);

            assert_same_size!(mask, target);

            let color = csscolorparser::parse(&color).unwrap();

            // Fill background action
            let output =
                matting::fill_background(&target, &mask, color.to_rgba8());

            let bytes = matting::image_to_format(output, matting::ImageFormat::Png);

            image_response!(bytes)
        });

    let replace_api = warp::path!("api" / "replace")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            println!("Received replace request");

            let parts = read_parts!(form);

            let mut target: Option<Part> = None;
            let mut mask: Option<Part> = None;
            let mut replacement: Option<Part> = None;

            for part in parts {
                if part.name() == "target" {
                    target = Some(part);
                } else if part.name() == "mask" {
                    mask = Some(part);
                } else if part.name() == "replacement" {
                    replacement = Some(part);
                }
            }

            let mut target_part = read_part!(target);
            let mut mask_part = read_part!(mask);
            let mut replacement_part = read_part!(replacement);

            let mask = part_to_image!(mask_part);
            let target = part_to_image!(target_part);
            let replacement = part_to_image!(replacement_part);

            assert_same_size!(mask, target);
            assert_same_size!(target, replacement);

            // Replace background action
            let output = matting::replace_background(
                &target,
                &mask,
                &replacement,
            );

            let bytes = matting::image_to_format(output, matting::ImageFormat::Png);

            image_response!(bytes)
        });

    let api = matting_api.or(transparent_api).or(fill_api).or(replace_api);

    let routes = api.or(static_files);

    routes
}

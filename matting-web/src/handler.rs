use futures::stream::TryStreamExt;
use matting::*;
use matting_lib as matting;
use warp::{
    http::{Response, StatusCode},
    multipart::{FormData, Part},
    reply, Buf, Filter, Rejection, Reply,
};

#[warn(opaque_hidden_inferred_bound)]
pub fn get_routes(
    www: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let static_files = warp::fs::dir(www);

    let upload_api = warp::path!("api" / "upload")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            macro_rules! bad_request {
                () => {
                    Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .header("Content-Type", "text/html; charset=utf-8")
                        .body(vec![])
                        .unwrap()
                };
            }
            
            let mut parts = {
                let parts: Result<Vec<Part>, _> = form.try_collect().await;

                if let Ok(data) = parts {
                    data
                } else {
                    panic!("Panico :O");
                }
            };

            let mut target: Option<Part> = None;
            let mut trimap: Option<Part> = None;
            let mut mask: Option<Part> = None;

            for mut part in parts {
                if part.name() == "target" {
                    target = Some(part);
                } else if part.name() == "trimap" {
                    trimap = Some(part);
                } else if part.name() == "mask" {
                    mask = Some(part);
                }
            }

            // This field is mandatory and should always be in the request
            let mut target_part = if let Some(mut target_part) = target {
                target_part
            } else {
                return bad_request!();
            };

            let target_buf = target_part.data().await.unwrap().unwrap();
            let target_data = target_buf.chunk();

            let mask = if let Some(mut trimap_part) = trimap {
                // Trimap is given
                let trimap_buf = trimap_part.data().await.unwrap().unwrap();
                let trimap_data = trimap_buf.chunk();

                // pre-process trimap
                // is it read correctly?
                let trimap = matting::bytes_to_mat(&trimap_data, IMREAD_GRAYSCALE).unwrap();

                let target = matting::bytes_to_mat(&target_data, IMREAD_COLOR).unwrap();

                // CORE DUMP
                let output = matting::generate_mask(&target, &trimap).unwrap();

                matting::mat_to_dynamic_image_gray(&output).unwrap()
            } else if let Some(mut mask_part) = mask {
                // Mask is given
                let buf = mask_part.data().await.unwrap().unwrap();
                let data = buf.chunk();
                matting::bytes_to_image(&data).unwrap()
            } else {
                return bad_request!();
            };

            let target = matting::bytes_to_image(target_data).unwrap();

            // target and mask need to be RgbImage

            // Implement actions, for now remove background
            let output = matting::remove_background(&target.into_rgb8(), &mask.into_rgb8());

            let bytes = matting::image_to_format(output, matting::ImageFormat::Png);

            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "image/*")
                .body(bytes)
                .unwrap();
        });

    let routes = upload_api.or(static_files);

    routes
}

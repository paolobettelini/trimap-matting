use log::info;

use warp::{multipart::FormData, Filter, Rejection, Reply};
mod routes;

#[allow(opaque_hidden_inferred_bound)]
pub fn get_routes(
    www: &'static str,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    info!("Server starting");

    let matting_api = warp::path!("api" / "matting")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            info!("Received matting request");

            routes::matting(form).await
        });

    let transparent_api = warp::path!("api" / "transparent")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            info!("Received transparent request");

            routes::transparent(form).await
        });

    let fill_api = warp::path!("api" / "fill" / String)
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|color: String, form: FormData| async move {
            info!("Received fill request");

            routes::fill(color, form).await
        });

    let replace_api = warp::path!("api" / "replace")
        .and(warp::post())
        .and(warp::multipart::form().max_length(2500000))
        .then(|form: FormData| async move {
            info!("Received replace request");

            routes::replace(form).await
        });

    let static_files = warp::fs::dir(www);

    let api = matting_api.or(transparent_api).or(fill_api).or(replace_api);

    api.or(static_files)
}

use std::net::IpAddr;

mod handler;

#[tokio::main]
async fn main() {
    start_service(
        "/home/paolo/Desktop/trimap-matting-interattivo/matting-web/static",
        "0.0.0.0",
        8080u16,
    )
    .await;
}

async fn start_service(www: &'static str, ip: &'static str, port: u16) {
    let routes = handler::get_routes(www);

    let ip = if let Ok(address) = ip.parse::<IpAddr>() {
        address
    } else {
        panic!("Invalid IP");
    };

    warp::serve(routes).run((ip, port)).await;
}

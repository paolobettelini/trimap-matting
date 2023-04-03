macro_rules! bad_request {
    () => {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(vec![])
            .unwrap()
    };
}

macro_rules! internal_error {
    () => {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
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
            warn!("Could not read parts");
            return bad_request!();
        }
    }};
}

macro_rules! read_part {
    ($option:tt) => {
        if let Some(part) = $option {
            part
        } else {
            warn!("Could not read part");
            return bad_request!();
        }
    };
}

macro_rules! read_part_buf {
    ($part:tt) => {
        if let Some(res) = $part.data().await {
            if let Ok(buf) = res {
                buf
            } else {
                warn!("Could not read part buffer");
                return bad_request!();
            }
        } else {
            error!("Could not read part buffer");
            return bad_request!();
        }
    };
}

macro_rules! part_to_image {
    ($part:tt) => {{
        let buf = read_part_buf!($part);
        let data = buf.chunk();
        let res = matting::bytes_to_image(&data);
        if let Ok(v) = res {
            v.into_rgb8()
        } else {
            return internal_error!();
        }
    }};
}

macro_rules! part_to_mat {
    ($part:tt, $prop:tt) => {{
        let buf = read_part_buf!($part);
        let data = buf.chunk();
        let res = matting::bytes_to_mat(&data, $prop);
        if let Ok(v) = res {
            v
        } else {
            return internal_error!();
        }
    }};
}

macro_rules! part_to_file {
    ($part:tt) => {{
        let buf = read_part_buf!($part);
        let data = buf.chunk();

        let mut file = png_tempfile!();
        if file.write(&data).is_err() {
            error!("Could not write to file");
            return internal_error!();
        }

        file
    }};
}

macro_rules! png_tempfile {
    () => {
        if let Ok(file) = Builder::new().suffix(".png").tempfile() {
            file
        } else {
            error!("Could not create tempfile");
            return internal_error!();
        }
    };
}

macro_rules! assert_same_size {
    ($img1:tt, $img2:tt) => {
        if $img1.dimensions() != $img2.dimensions() {
            warn!("Images are not of the same size");
            return bad_request!();
        }
    };
}

macro_rules! get_owned_abs {
    ($file:expr) => {{
        if let Ok(v) = $file.canonicalize() {
            v.to_string_lossy().into_owned()
        } else {
            error!("Could not extract absolute path");
            return internal_error!();
        }
    }};
}

pub(crate) use assert_same_size;
pub(crate) use bad_request;
pub(crate) use get_owned_abs;
pub(crate) use image_response;
pub(crate) use internal_error;
pub(crate) use part_to_file;
pub(crate) use part_to_image;
pub(crate) use part_to_mat;
pub(crate) use png_tempfile;
pub(crate) use read_part;
pub(crate) use read_part_buf;
pub(crate) use read_parts;

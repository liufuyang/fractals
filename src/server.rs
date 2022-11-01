use std::io::Cursor;

use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use image::RgbImage;
use std::collections::HashMap;
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

use crate::math::complex::Complex;
use crate::math::polynomial::Polynomial;
use crate::rendering::render_image;
use crate::{newton_method_field, Field};
use crate::server::ServerError::ParsingError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("passing error on field: {field:?}, message: {message:?}")]
    ParsingError {
        field: String,
        message: String,
    },
    #[error("unknown error")]
    UnknownError,
}

pub async fn api(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match api_inner(req).await {
        Ok(r) => Ok(r),
        Err(e) => Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(e.to_string()))
            .unwrap())
    }
}

pub async fn api_inner(req: Request<Body>) -> Result<Response<Body>, ServerError> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let params = read_query(req.uri());
            let field = parse_field_params(&params)?;
            let pol = parse_pol_param(&params)?;
            let d = handle_image_request(pol, field).await;
            *response.body_mut() = d.into();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Ok(response)
}

fn read_query(uri: &Uri) -> HashMap<String, String> {
    uri.query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new)
}

fn parse_field_params(params: &HashMap<String, String>) -> Result<Field, ServerError> {
    let tsize: f64 = parse_param_f64(params, "tw")?;
    let tx: f64 = parse_param_f64(params, "tx")?;
    let ty: f64 = parse_param_f64(params, "ty")?;

    Ok(Field {
        source: Complex { re: tx, im: ty },
        size: tsize,
        grid: 512,
    })
}

fn parse_param_f64(params: &HashMap<String, String>, name: &str) -> Result<f64, ServerError> {
    if let Some(param) = params.get(name) {
        return param.parse().map_err(|e: ParseFloatError| ParsingError { message: e.to_string(), field: name.to_string() });
    }
    Err(ServerError::InvalidArgument(format!("missing {}", name)))
}

fn parse_pol_param(params: &HashMap<String, String>) -> Result<Polynomial, ServerError> {
    let pol = params
        .get("pol").ok_or(ServerError::InvalidArgument("missing pol".to_string()))?;
    let coef_r: Result<Vec<i32>, _> = pol.split(",").map(|s| s.parse::<i32>()).collect();
    let coef: Vec<i32> = coef_r.map_err(|e: ParseIntError| ParsingError { message: e.to_string(), field: "pol".to_string() })?;
    Ok(Polynomial::new(coef))
}

async fn handle_image_request(pol: Polynomial, field: Field) -> Vec<u8> {
    let max_iter = 100;

    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let solutions = newton_method_field(&pol, &field, max_iter);
        let image = render_image(&solutions, &field, max_iter);
        let _ = send.send(serialize_image(image));
    });

    recv.await.expect("Panic in rayon:spawn")
}

fn serialize_image(image: RgbImage) -> Vec<u8> {
    let mut data = Cursor::new(Vec::new());
    image
        .write_to(&mut data, image::ImageOutputFormat::Jpeg(255))
        .expect("Unable to write");
    data.get_ref().clone()
}

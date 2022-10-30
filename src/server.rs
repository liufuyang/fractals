use std::io::Cursor;

use hyper::{Body, Method, Request, Response, StatusCode, Uri};
use image::RgbImage;
use std::collections::HashMap;
use std::convert::Infallible;

use crate::math::complex::Complex;
use crate::math::polynomial::Polynomial;
use crate::rendering::render_image;
use crate::{newton_method_field, Field};

pub async fn api(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let params = read_query(req.uri());
            let field = parse_field_params(&params);
            let pol = parse_pol_param(&params);
            let d = handle_image_request(pol, field);
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

fn parse_field_params(params: &HashMap<String, String>) -> Field {
    let tsize: f64 = parse_param_f64(params, "tw", 10.);
    let tx: f64 = parse_param_f64(params, "tx", -5.);
    let ty: f64 = parse_param_f64(params, "ty", -5.);

    Field {
        source: Complex { re: tx, im: ty },
        size: tsize,
        grid: 512,
    }
}

fn parse_param_f64(params: &HashMap<String, String>, name: &str, default: f64) -> f64 {
    if let Some(param) = params.get(name) {
        if let Ok(val) = param.parse() {
            return val;
        }
    }

    default
}

fn parse_pol_param(params: &HashMap<String, String>) -> Polynomial {
    let coef: Vec<i32> = params
        .get("pol")
        .map(|s| s.split(",").map(|s| s.parse().unwrap()).collect())
        .unwrap_or(vec![-1, 0, 0, 1]);
    Polynomial::new(coef)
}

fn handle_image_request(pol: Polynomial, field: Field) -> Vec<u8> {
    let max_iter = 100;
    let solutins = newton_method_field(&pol, &field, max_iter);
    let image = render_image(&solutins, &field, max_iter);
    serialize_image(image)
}

fn serialize_image(image: RgbImage) -> Vec<u8> {
    let mut data = Cursor::new(Vec::new());
    image
        .write_to(&mut data, image::ImageOutputFormat::Jpeg(255))
        .expect("Unable to write");
    data.get_ref().clone()
}

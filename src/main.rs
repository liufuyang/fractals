use newton_factal::math::polynomial::Polynomial;
use newton_factal::{render_image, Field};
use std::io::Cursor;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;


async fn api(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let params = parse_params(req.uri());
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

fn parse_params(uri: &Uri) -> HashMap<String, String> {
    uri
    .query()
    .map(|v| {
        url::form_urlencoded::parse(v.as_bytes())
        .into_owned()
        .collect()
    }).unwrap_or_else(HashMap::new)

}

fn parse_field_params(params: &HashMap<String, String>) -> Field {
    let tsize: f64 = parse_param_f64(params, "tw", 10.);
    let tx: f64 = parse_param_f64(params, "tx", -5.);
    let ty: f64 = parse_param_f64(params, "ty", -5.);

    Field {
        source: (0, 0),
        ssize: 512,

        target: (tx, ty),
        tsize: tsize,
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
        .unwrap();
    Polynomial::new(coef)
}

fn handle_image_request(pol: Polynomial, field: Field) -> Vec<u8> {
    let image = render_image(pol, field);
    let mut data = Cursor::new(Vec::new());
    image
        .write_to(&mut data, image::ImageOutputFormat::Jpeg(255))
        .expect("Unable to write");
    let d = data.get_ref().clone();
    d
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(api))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

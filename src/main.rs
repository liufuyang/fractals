use newton_factal::math::polynomial::Polynomial;
use newton_factal::{render_image, Field};
use std::io::Cursor;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let pol = Polynomial::new(vec![-1, 0, 0, 1]);
    let field = Field {
        source: (0, 0),
        ssize: 512,

        target: (-5., -5.),
        tsize: 10.0,
    };
    let image = render_image(pol, field);
    let mut data = Cursor::new(Vec::new());
    image
        .write_to(&mut data, image::ImageOutputFormat::Jpeg(255))
        .expect("Unable to write");
    let d: Vec<u8> = data.get_ref().clone();
    let mut response = Response::new(Body::empty());
    *response.body_mut() = d.into();
    Ok(response)
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

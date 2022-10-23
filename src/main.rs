use newton_factal::math::polynomial::Polynomial;
use newton_factal::{render_image, Field};
use std::net::TcpListener;

use std::io::prelude::*;
use std::io::Cursor;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let pol = Polynomial::new(vec![-1, 0, 0, 1]);
    let field = Field {
        source: (0, 0),
        ssize: 512,

        target: (-5., -5.),
        tsize: 10.0,
    };
    let image = render_image(pol, field);
    let headers = ["HTTP/1.1 200 OK", "Content-type: image/jpeg", "\r\n"];
    let mut response: Vec<u8> = headers.join("\r\n").to_string().into_bytes();
    let mut data = Cursor::new(Vec::new());
    image
        .write_to(&mut data, image::ImageOutputFormat::Jpeg(255))
        .expect("Unable to write");
    response.extend(&data.get_ref()[..]);
    stream.write_all(&response).unwrap();
    stream.flush().unwrap();
}

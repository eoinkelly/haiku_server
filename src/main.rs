mod haiku;
use haiku::Haiku;

extern crate hyper;
extern crate rand;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentLength;
use std::io::Write; // required for write_all()


fn handler(req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            println!("Processing GET request");

            let body: &[u8] = Haiku::choose_random().content.as_bytes();

            // get a mutable reference to headers and then set the content length to be the length
            // of body we are about to send
            res.headers_mut().set(ContentLength(body.len() as u64));

            // "consume" this response by writing headers and status to the stream and creating a
            // mutable response that you can use to write the body
            let mut mut_res = res.start().unwrap();

            mut_res.write_all(body).unwrap();
        }
        // Set status to HTTP 405 (Method not allowed) for all other HTTP methods
        _ => *res.status_mut() = StatusCode::MethodNotAllowed,
    }
}

const IP: &'static str = "0.0.0.0";
const PORT: &'static str = "3000";

fn main() {
    let connection_str = format!("{}:{}", IP, PORT);
    println!("Starting haiku server on port {}", connection_str);
    Server::http(connection_str.as_str()).unwrap().handle(handler).unwrap();
}

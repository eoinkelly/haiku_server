mod haiku;
use haiku::Haiku;

extern crate hyper;
extern crate rand;

extern crate rustc_serialize;
use rustc_serialize::json;

use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::header::ContentLength;
use std::io::Write; // required for write_all()
use std::env;

const DEFAULT_IP: &'static str = "0.0.0.0";
const DEFAULT_PORT: &'static str = "3000";

fn main() {
    let connection_str = format!("{}:{}", DEFAULT_IP, get_server_port());
    println!("Starting haiku server on {}", connection_str);
    Server::http(connection_str.as_str()).unwrap().handle(handler).unwrap();
}

fn get_server_port() -> String {
    match env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from(DEFAULT_PORT),
    }
}

fn handler(req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            println!("Processing GET request");

            let haiku = Haiku::choose_random();
            // let fallback = String::from("failed to encode the haiku as JSON");

            let body_json = json::encode(&haiku).unwrap();
            let body_bytes = body_json.as_bytes();

            // get a mutable reference to headers and then set the content length to be the length
            // of body we are about to send
            res.headers_mut().set(ContentLength(body_bytes.len() as u64));

            // "consume" this response by writing headers and status to the stream and creating a
            // mutable response that you can use to write the body
            let mut mut_res = res.start().unwrap();

            // write the body bytes into the stream
            mut_res.write_all(body_bytes).unwrap();
        }
        // Set status to HTTP 405 (Method not allowed) for all other HTTP methods
        _ => *res.status_mut() = StatusCode::MethodNotAllowed,
    }
}

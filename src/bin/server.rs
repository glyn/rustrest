extern crate hyper;

use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
    // handle things here
    println!("Request received: {}", req.headers);
}

fn main() {
    Server::http("localhost:8080").unwrap().handle(hello).unwrap();
}

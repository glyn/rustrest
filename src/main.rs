extern crate hyper;
extern crate hyper_native_tls;

use std::env;
use std::io::Read;
use hyper::Client;
use hyper::client::response::Response;
use hyper_native_tls::NativeTlsClient;
use hyper::net::HttpsConnector;

fn main() {
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return;
        }
    };

    let url = url.parse::<hyper::Url>().unwrap();
    
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let result = client.get(url).send().and_then(|res| {
        println!("Response: {}", res.status);
        println!("Headers: \n{}", res.headers);
        read_to_string(res)
    }).map(|s| {
        println!("Response body: {}", s);
    });
    
    match result {
        Ok(_) => println!("\n\nDone."),
        Err(e) => println!("\n\nFailed {:?}", e),
    }
}

fn read_to_string(mut r: Response) -> hyper::Result<String> {
    let mut s = String::new();
    try!(r.read_to_string(&mut s));
    Ok(s)
}

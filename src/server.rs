use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }
    // fn new(address: String) -> Server {
    // Server { address: address }
    // }

    pub fn run(self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on: {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    Response::new(
                                        StatusCode::Ok,
                                        Option::Some("<html><body><div>You awesome Rust server is serving pages</div></body></html>".to_string()),
                                    )
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send a response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
            }
        }
    }
}

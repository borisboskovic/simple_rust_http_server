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
        println!("Listening on: {}", self.address);
        let listener = TcpListener::bind(&self.address);
    }
}

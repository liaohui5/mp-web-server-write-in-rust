use crate::request::Request;
use crate::router::Router;
use crate::thread_pool::ThreadPool;
use colored::Colorize;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        // bind listener
        let listener = TcpListener::bind(self.socket_addr).unwrap();
        let pool = ThreadPool::new(4);

        println!(
            "Server running on: {}{}",
            "http://".red(),
            self.socket_addr.red()
        );

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            // handle request by threads pool
            pool.execute(move || {
                handle_http_request(&mut stream);
            });
        }
    }
}

fn handle_http_request<T: Read + Write>(stream: &mut T) {
    // read stream to String
    let mut buffer: [u8; 1024] = [0; 1024];
    Read::read(stream, &mut buffer).unwrap();

    // parse String to Request
    let request: Request = String::from_utf8(buffer.to_vec()).unwrap().into();

    // dispatch to handlers
    Router::route(request, stream);
}

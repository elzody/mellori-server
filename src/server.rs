use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

use crate::http::HttpRequest;

pub struct Server {
    pub address: SocketAddr,
}

impl Server {
    pub fn run(&self) {
        let listener = TcpListener::bind(self.address).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(move || handle_stream(stream));
                }
                Err(_) => println!("An error has occurred with the server"),
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut data: Vec<u8> = Vec::new();

    while let Ok(i) = stream.read_to_end(&mut data) {
      match i {
        0 => {
          let request = HttpRequest::from(data);
          println!("{:?}", request);

          break;
        }
        _ => ()
      }
    }
}

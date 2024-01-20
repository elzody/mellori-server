use std::net::SocketAddr;

mod http;
mod server;

fn main() {
  let address = SocketAddr::from(([127, 0, 0, 1], 7654));

  let my_server = server::Server { address };

  my_server.run();
}

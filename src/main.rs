use http::Request;
use http::Method;
use server::Server;

mod server;
mod http;

fn main() {

    // string holds IP address and Port number
    // here, 'new' is an associated function
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
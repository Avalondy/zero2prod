use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Port 0 is special-cased at the OS level: trying to bind port 0 will trigger an OS scan
    // for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    zero2prod::run(listener)?.await
}

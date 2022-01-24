use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind to port 8080");
    run(listener)?.await
}
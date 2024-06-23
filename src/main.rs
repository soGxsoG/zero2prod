use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to load configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    println!("Working on {} port", configuration.application_port);
    run(listener)?.await
}

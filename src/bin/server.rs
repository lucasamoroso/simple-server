use clap::App;
use clap::Arg;
use log::info;
use service::greeter::greeter_server::GreeterServer;
use service::MyGreeter;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tonic::transport::Server;

fn get_port() -> u16 {
    let flags = App::new("simple-server")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("NUMBER")
                .help("Sets the port number to listen on")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let port: u16 = flags
        .value_of("port")
        .unwrap_or("8080")
        .parse()
        .unwrap_or_else(|e| panic!(r#"--port value invalid: {}"#, e));

    port
}

// // Runtime to run our server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let port = get_port();
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let greeter = MyGreeter::default();

    info!("Starting gRPC Server on 127.0.0.1:{} ...", port);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

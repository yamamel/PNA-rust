use clap::{App, Arg};
use slog::info;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use kvs::{KvsError, Result};
use std::net::{TcpListener, TcpStream};
use std::io::Read;  


fn main() -> Result<()> {
    let matches = App::new("kvs-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A kvs database server")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .help("Set the ip and port of the server")
                .value_name("IP-PORT")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("engine")
                .long("engine")
                .help("Choose which engine to use")
                .value_name("ENGINE-NAME")
                .takes_value(true),
        ).get_matches();

    let address = match matches.value_of("addr") {
        Some(a) => {
            a
        }
        None =>{
            "127.0.0.1:4000"
        }
    };

    let engine = match matches.value_of("engine") {
        Some(e) => {
            e
        }
        None => {
            "kvs"
        }
    };

    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();

    let listener = TcpListener::bind(address)?;
    info!(logger, "initiate the database server, version: {} engine: {} address: {}", env!("CARGO_PKG_VERSION"), engine, address);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let remote_addr = stream.peer_addr()?;
                info!(logger, "connection from remote address: {}", remote_addr);
                let mut buf = String::new();
                stream.read_to_string(&mut buf)?;
                println!("{}", buf);
            }
            Err(_) => {

            }
        }
    }



    

    Ok(())
}

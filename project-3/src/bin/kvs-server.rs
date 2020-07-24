use clap::{App, Arg};
use slog::info;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use kvs::{KvsError, Result, Command, KvStore};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};  
use std::env::current_dir;
use std::process::exit;

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

    let mut kvstore = KvStore::open(current_dir()?)?;
    info!(logger, "initiate the database server"); 
    info!(logger, "version: {} engine: {} address: {}", env!("CARGO_PKG_VERSION"), engine, address);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let remote_addr = stream.peer_addr()?;
                info!(logger, "connection from remote address: {}", remote_addr);
                let mut buf = [0; 512];
                let byte_num = stream.read(&mut buf)?;
                // println!("{}", byte_num);
                let request = String::from_utf8((&buf[0..byte_num]).to_vec()).expect("Found invalid utf-8");
                println!("{}", request);
                let cmd = serde_json::from_str(&request)?;
                match cmd {
                    Command::Set { key, value } => {
                        kvstore.set(key, value)?;
                    }
                    Command::Get { key } => {
                        let response = match kvstore.get(key.to_owned())? {
                            None => {
                                "Key not found".to_owned()
                            }
                            Some(value) => {
                                value
                            }
                        };
                        stream.write(response.as_bytes())?;
                        stream.flush()?;
                    }
                    Command::Rm { key } => {
                        match kvstore.remove(key.to_owned()) {
                            Ok(()) => (),
                            Err(KvsError::KeyNotFoundError) => {
                                stream.write("Key not found".as_bytes())?;
                                stream.flush()?;
                                exit(1);
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }

            }
            Err(_) => {

            }
        }
    }
    Ok(())
}

use clap::arg_enum;
use structopt::StructOpt;
use slog::info;
use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;
use kvs::{KvsError, Result, Command, KvStore, KvsEngine};
use std::net::TcpListener;
use std::io::{Read, Write, BufReader, BufWriter};  
use std::env::current_dir;
use kvs::SledStore;

arg_enum! {
    #[derive(Debug, Clone)]
    enum Engine {
        Kvs,
        Sled,
    }
}

#[derive(Debug, StructOpt, Clone)]
struct ServerOpt {
    #[structopt(long, possible_values = &Engine::variants(), case_insensitive = true)]
    engine: Option<Engine>,

    #[structopt(long, default_value = "127.0.0.1:4000")]
    addr: String,
}

fn main() -> Result<()> {
    let opt = ServerOpt::from_args();

    let engine = opt.clone().engine.unwrap_or(Engine::Kvs);

    let paths = std::fs::read_dir(current_dir()?).unwrap();

    let mut sled_exist: Option<String> = None;

    let mut kvs_exist: Option<String> = None;

    for entry in paths {
        let file_name = entry.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        if file_name.starts_with("kvs") {
            kvs_exist = Some(file_name);
            break;
        } else if file_name.starts_with("sled") {
            sled_exist = Some(file_name);
            break;
        }
    }

    match engine {
        Engine::Kvs => {
            if sled_exist.is_some() {
                return Err(KvsError::WrongEngineError);
            } else {
                run(KvStore::open(current_dir()?)?, opt)
            }
        }
        Engine::Sled => {
            if kvs_exist.is_some() {
                return Err(KvsError::WrongEngineError);
            } else {
                run(SledStore::open(current_dir()?)?, opt)
            }
        }
    }
}

fn run(mut store: impl KvsEngine, opt: ServerOpt) -> Result<()> {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();

    let listener = TcpListener::bind(&opt.addr)?;

    let engine = if opt.engine.is_none() {
        Engine::Kvs
    } else {
        opt.engine.unwrap()
    };

    info!(logger, "initiate the database server"); 
    info!(logger, "version: {} engine: {} address: {}", env!("CARGO_PKG_VERSION"), engine, opt.addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // let remote_addr = stream.peer_addr()?;
                // info!(logger, "connection from remote address: {}", remote_addr);
                let mut reader = BufReader::new(stream.try_clone()?);
                let mut writer = BufWriter::new(stream);
                let mut buf = [0; 512];
                let byte_num = reader.read(&mut buf)?;
                // println!("{}", byte_num);
                let request = String::from_utf8((&buf[0..byte_num]).to_vec()).expect("Found invalid utf-8");
                // println!("{}", request);
                let cmd = serde_json::from_str(&request)?;
                match cmd {
                    Command::Set { key, value } => {
                        store.set(key, value)?;
                    }
                    Command::Get { key } => {
                        let response = match store.get(key.to_owned())? {
                            None => {
                                "Key not found".to_owned()
                            }
                            Some(value) => {
                                value
                            }
                        };
                        writer.write(response.as_bytes())?;
                        writer.flush()?;
                    }
                    Command::Rm { key } => {
                        match store.remove(key.to_owned()) {
                            Ok(()) => (),
                            Err(KvsError::KeyNotFoundError) => {
                                writer.write("Key not found".as_bytes())?;
                                writer.flush()?;
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

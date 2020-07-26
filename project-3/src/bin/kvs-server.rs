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

#[derive(Debug, StructOpt)]
struct ServerOpt {
    #[structopt(long, default_value = "kvs")]
    engine: String,

    #[structopt(long, default_value = "127.0.0.1:4000")]
    addr: String,
}

fn main() -> Result<()> {
    let opt = ServerOpt::from_args();
    if opt.engine != "kvs".to_owned() && opt.engine != "sled".to_owned() {
        return Err(KvsError::WrongEngineError { engine: opt.engine, });
    }

    let mut paths = std::fs::read_dir(current_dir()?).unwrap();

    let sled_exist = paths.find(|p| p.as_ref().unwrap().path().file_name().unwrap().to_string_lossy().starts_with("sled"));
    let kvs_exist = paths.find(|p| p.as_ref().unwrap().path().file_name().unwrap().to_string_lossy().starts_with("kvs"));

    if (opt.engine == "kvs".to_owned() && sled_exist.is_some()) || (opt.engine == "sled".to_owned() && kvs_exist.is_some()) {
        return Err(KvsError::WrongEngineError { engine: opt.engine, });
    }

    if opt.engine == "kvs".to_owned() {
        run(KvStore::open(current_dir()?)?, opt)
    } else {
        run(SledStore::open(current_dir()?)?, opt)
    }
}

fn run(mut store: impl KvsEngine, opt: ServerOpt) -> Result<()> {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);
    let logger = builder.build().unwrap();

    let listener = TcpListener::bind(&opt.addr)?;

    info!(logger, "initiate the database server"); 
    info!(logger, "version: {} engine: {} address: {}", env!("CARGO_PKG_VERSION"), opt.engine, opt.addr);

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

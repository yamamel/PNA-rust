use clap::{App, Arg};
use kvs::{KvStore, Command};
use kvs::{KvsError, Result};
use std::process::exit;
use std::env::current_dir;
use std::net::TcpStream;
use std::io::{Write, Read};
use serde_json;

fn main() -> Result<()> {
    let matches = App::new("kvs-client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("A kvs database client")
        .arg(
            Arg::with_name("addr")
                .long("addr")
                .help("Set the ip and port of the server")
                .value_name("IP-PORT")
                .takes_value(true),
        )
        .subcommand(
            App::new("set")
                .about("set a key-value pair to the kvs")
                .args(&[
                    Arg::with_name("KEY").index(1).required(true),
                    Arg::with_name("VALUE").index(2).required(true),
                ]),
        )
        .subcommand(
            App::new("get")
                .about("get a key-value pair from the kvs")
                .arg(Arg::with_name("KEY").index(1).required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("remove a key-value pair from the kvs")
                .arg(Arg::with_name("KEY").index(1).required(true)),
        )
        .subcommand(App::new("compact"))
        .get_matches();

    let address = match matches.value_of("addr") {
        Some(a) => {
            a
        }
        None =>{
            "127.0.0.1:4000"
        }
    };

    let mut stream = TcpStream::connect(address)?;


    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            let value = matches.value_of("VALUE").expect("VALUE is missing");
            // let mut kvstore = KvStore::open(current_dir()?)?;
            // kvstore.set(key.to_owned(), value.to_owned());
            let cmd = Command::Set { key: key.to_owned(), value: value.to_owned() };
            let cmd = serde_json::to_string(&cmd)?;
            stream.write(cmd.as_bytes())?;
            stream.flush()?;
            Ok(())
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            // let mut kvstore = KvStore::open(current_dir()?)?;
            // match kvstore.get(key.to_owned())? {
            //     None => {
            //         println!("Key not found");
            //         exit(0);
            //     }
            //     Some(value) => {
            //         println!("{}", value);
            //         Ok(())
            //     }
            // }
            let cmd = Command::Get { key: key.to_owned() };
            let cmd = serde_json::to_string(&cmd)?;
            stream.write(cmd.as_bytes())?;
            stream.flush()?;
            let mut response = String::new();
            stream.read_to_string(&mut response)?;
            println!("{}", response);
            Ok(())
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            // let mut kvstore = KvStore::open(current_dir()?)?;
            // match kvstore.remove(key.to_owned()) {
            //     Ok(()) => Ok(()),
            //     Err(KvsError::KeyNotFoundError) => {
            //         println!("Key not found");
            //         exit(1);
            //     }
            //     Err(e) => Err(e),
            // }
            let cmd = Command::Rm { key: key.to_owned() };
            let cmd = serde_json::to_string(&cmd)?;
            stream.write(cmd.as_bytes())?;
            stream.flush()?;
            let mut response = String::new();
            stream.read_to_string(&mut response)?;
            if response.len() > 0 {
                println!("{}", response);
            }
            Ok(())
        }
        _ => {
            exit(1);
        }
    }
}

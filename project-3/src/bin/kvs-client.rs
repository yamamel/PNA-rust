use clap::{App, Arg};
use kvs::KvStore;
use kvs::{KvsError, Result};
use std::process::exit;
use std::env::current_dir;
use std::net::TcpStream;
use std::io::Write;

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
            let _key = matches.value_of("KEY").expect("KEY is missing");
            let _value = matches.value_of("VALUE").expect("VALUE is missing");
            // let mut kvstore = KvStore::open(current_dir()?)?;
            // kvstore.set(key.to_owned(), value.to_owned());
            stream.write(b"set")?;
            Ok(())
        }
        ("get", Some(matches)) => {
            let _key = matches.value_of("KEY").expect("KEY is missing");
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
            stream.write(b"get")?;
            Ok(())
        }
        ("rm", Some(matches)) => {
            let _key = matches.value_of("KEY").expect("KEY is missing");
            // let mut kvstore = KvStore::open(current_dir()?)?;
            // match kvstore.remove(key.to_owned()) {
            //     Ok(()) => Ok(()),
            //     Err(KvsError::KeyNotFoundError) => {
            //         println!("Key not found");
            //         exit(1);
            //     }
            //     Err(e) => Err(e),
            // }
            stream.write(b"rm")?;
            Ok(())
        }
        _ => {
            exit(1);
        }
    }
}

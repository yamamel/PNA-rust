use clap::{App, Arg};
use kvs::KvStore;
use kvs::{KvsError, Result};
use std::process::exit;
use std::env::current_dir;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
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

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            let value = matches.value_of("VALUE").expect("VALUE is missing");
            let mut kvstore = KvStore::open(current_dir()?)?;
            kvstore.set(key.to_owned(), value.to_owned())
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            let mut kvstore = KvStore::open(current_dir()?)?;
            match kvstore.get(key.to_owned())? {
                None => {
                    println!("Key not found");
                    exit(0);
                }
                Some(value) => {
                    println!("{}", value);
                    Ok(())
                }
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            let mut kvstore = KvStore::open(current_dir()?)?;
            match kvstore.remove(key.to_owned()) {
                Ok(()) => Ok(()),
                Err(KvsError::KeyNotFoundError) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => Err(e),
            }
        }
        _ => {
            exit(1);
        }
    }
}

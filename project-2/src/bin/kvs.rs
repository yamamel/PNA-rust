use clap::{App, Arg};
use kvs::KvStore;
use kvs::{KvsError, Result};
use std::process::exit;
// use tempfile::TempDir;
// use walkdir::WalkDir;
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
            // eprintln!("unimplemented");
            // exit(1);
            let mut kvstore = KvStore::open(current_dir()?)?;
            kvstore.set(key.to_owned(), value.to_owned())
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY is missing");
            // eprintln!("unimplemented");
            // exit(1);
            let mut kvstore = KvStore::open(current_dir()?)?;
            match kvstore.get(key.to_owned())? {
                None => {
                    println!("Key not found");
                    // Err(KvsError::KeyNotFoundError)
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
            // eprintln!("unimplemented");
            // exit(1);
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
        ("compact", Some(_)) => {
            let mut kvstore = KvStore::open(current_dir()?)?;
            kvstore.compact()?;
            Ok(())
        }
        _ => {
            exit(1);
        }
    }

    // let mut temp_dir = current_dir()?;
    // temp_dir.push("tmp/");
    // let mut store = KvStore::open(&temp_dir).unwrap();

    // let dir_size = || {
    //     let entries = WalkDir::new(&temp_dir).into_iter();
    //     let len: walkdir::Result<u64> = entries
    //         .map(|res| {
    //             res.and_then(|entry| entry.metadata())
    //                 .map(|metadata| metadata.len())
    //         })
    //         .sum();
    //     len.expect("fail to get directory size")
    // };

    // let mut current_size = dir_size();
    // for iter in 0..1000 {
    //     for key_id in 0..1000 {
    //         let key = format!("key{}", key_id);
    //         let value = format!("{}", iter);
    //         store.set(key, value).unwrap();
    //     }

    //     let new_size = dir_size();
    //     if new_size > current_size {
    //         current_size = new_size;
    //         continue;
    //     }
    //     // Compaction triggered.

    //     drop(store);
    //     // reopen and check content.
    //     let mut store = KvStore::open(&temp_dir).unwrap();
    //     for key_id in 0..1000 {
    //         let key = format!("key{}", key_id);
    //         println!("{}: {:?}", &key, store.get(key.clone()).unwrap());
    //         // assert_eq!(store.get(key)?, Some(format!("{}", iter)));
    //     }
    //     return Ok(());
    // }

    // panic!("No compaction detected");
}

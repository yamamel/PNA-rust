use clap::{App, Arg};
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("set")
            .about("set a key-value pair to the kvs")
            .args(&[
                Arg::with_name("KEY")
                    .index(1)
                    .required(true),
                Arg::with_name("VALUE")
                    .index(2)
                    .required(true),
            ]),
        )
        .subcommand(
            App::new("get")
            .about("get a key-value pair from the kvs")
            .arg(
                Arg::with_name("KEY")
                    .index(1)
                    .required(true),
            )
        )
        .subcommand(
            App::new("rm")
            .about("remove a key-value pair from the kvs")
            .arg(
                Arg::with_name("KEY")
                    .index(1)
                    .required(true),
            )
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let _key = matches.value_of("KEY").unwrap();
            let _value = matches.value_of("VALUE").unwrap();
            eprintln!("unimplemented");
            exit(1);
        },
        ("get", Some(matches)) => {
            let _key = matches.value_of("KEY").unwrap();
            eprintln!("unimplemented");
            exit(1);
        },
        ("rm", Some(matches)) => {
            let _key = matches.value_of("KEY").unwrap();
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            exit(1);
        }
    }


}
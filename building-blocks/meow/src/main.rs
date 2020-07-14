// #[cfg(feature = "yaml")]
// use clap::load_yaml;
// use clap::App;

// #[macro_use]
// extern crate clap;

// fn main() {

//     let yaml = load_yaml!("../cli.yml");
//     let m = App::from(yaml).get_matches();

//     match m.value_of("argument1") {
//         // ...
//     }
// }

// #[macro_use]
// extern crate dotenv_codegen;

// fn main() {
//     println!("{}", dotenv!("PORT"));
// }

use std::env;

fn main() {
    let key = "HOME";
    match env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    }
}
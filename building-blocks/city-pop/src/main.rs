extern crate docopt;

use std::io::{self, Write};
use std::process;
use docopt::Docopt;
use csv;
use std::fs;
use std::path::Path;
use std::error::Error;


static USAGE: &'static str = "
Usage: city-pop [options] <data-path> <city>
city-pop --help

Options:
-h, --help     Show this usage message.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_data_path: Option<String>,
    arg_city: String,
}

#[derive(Debug)]
struct PopulationCount {
    city: String,
    country: String,
    count: u64,
}

#[derive(Debug, RustcDecodable)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,

    population: Option<u64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

fn search<P: AsRef<Path>>(file_path: &Option<P>, city: &str) -> Result<Vec<PopulationCount>, Box<Error + Send + Sync>> {
    let mut found = vec![];
    
    let input: Box<io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        Some(ref file_path) => Box::new(fs::File::open(file_path)?),
    };
    let mut rdr = csv::Reader::from_reader(input);
    for row in rdr.decode::<Row>() {
        let row = row?;
        if row.city == city {
            match row.population {
                None => {}
                Some(count) => {
                    found.push(PopulationCount {
                        city: row.city,
                        country: row.country,
                        count: count,
                    });
                }
            }
        }
    }

    if found.is_empty() {
        Err(From::from("No matching cities with a population were found"))
    } else {
        Ok(found)
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|err| err.exit());
    
    for pop in search(&args.arg_data_path, &args.arg_city).unwrap() {
        println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
    }
}
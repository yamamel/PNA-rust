use clap::arg_enum;
use structopt::StructOpt;
use std::path::PathBuf;
use std::process::exit;

arg_enum! {
    #[derive(Debug)]
    enum Subcommand {
        Set,
        Rm,
        Get,
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Important argument.
    #[structopt(possible_values = &Subcommand::variants(), case_insensitive = true)]
    subcommand: Subcommand,

    #[structopt(name = "KEY", parse(from_os_str))]
    key: PathBuf,

    #[structopt(name = "VALUE", parse(from_os_str), required_if("subcommand", "Subcommand::Set"))]
    value: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    
    match opt.subcommand {
        Subcommand::Set => {
            eprintln!("unimplemented");
            exit(1);
        }
        Subcommand::Get => {
            eprintln!("unimplemented");
            exit(1);
        }
        Subcommand::Rm => {
            eprintln!("unimplemented");
            exit(1);
        }
    }
}
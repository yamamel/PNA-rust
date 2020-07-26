use structopt::StructOpt;
use kvs::Command;
use kvs::Result;
use std::net::TcpStream;
use std::io::{Write, Read, BufWriter, BufReader};
use serde_json;
use std::process::exit;

#[derive(StructOpt, Debug)]
struct ClientOpt {
    #[structopt(subcommand)]
    cmd: Command,
    
    #[structopt(long, default_value = "127.0.0.1:4000", global = true)]
    addr: String,
}

fn main() -> Result<()> {
    let opt = ClientOpt::from_args();

    let stream = TcpStream::connect(opt.addr)?;
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);


    match opt.cmd {
        Command::Set { key: _, value: _ } => {
            let cmd = serde_json::to_string(&opt.cmd)?;
            writer.write(cmd.as_bytes())?;
            writer.flush()?;
            Ok(())
        }
        Command::Get { key: _ } => {       
            let cmd = serde_json::to_string(&opt.cmd)?;
            writer.write(cmd.as_bytes())?;
            writer.flush()?;
            let mut response = String::new();
            reader.read_to_string(&mut response)?;
            println!("{}", response);
            Ok(())
        }
        Command::Rm { key: _ } => {
            let cmd = serde_json::to_string(&opt.cmd)?;
            writer.write(cmd.as_bytes())?;
            writer.flush()?;
            let mut response = String::new();
            reader.read_to_string(&mut response)?;
            if response.len() > 0 {
                eprintln!("{}", response);
                exit(1);
            }
            Ok(())
        }
    }
}

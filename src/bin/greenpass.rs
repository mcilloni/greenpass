use std::{
    fs::read,
    io::{self, prelude::*, stdin},
    process::exit,
};

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.0.1")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// File to process. Omit or specify `-` to read from stdin
    #[clap(default_value = "-")]
    file: String,
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();

    stdin().read_to_end(&mut buf)?;

    Ok(buf)
}

fn main_do() -> std::result::Result<(), anyhow::Error> {
    let Opts { file } = Opts::parse();

    let buf = if file == "-" {
        read_stdin()?
    } else {
        read(file)?
    };

    if !buf.is_empty() {
        let buf_str = String::from_utf8(buf)?;

        println!("{:#?}", greenpass::parse(&buf_str)?);
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_do() {
        eprintln!("error: {}", e);
        exit(-1);
    }
}

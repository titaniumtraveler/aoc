use crate::runner::AocRunner;
use advent_of_code::cli::{Cli, Store};
use clap::Parser;
use std::{
    fs::File,
    io::{self, stdout, Read},
};
use thiserror::Error;

mod macros;
mod runner;

fn main() -> Result<(), Error> {
    let mut runner = AocRunner;
    let mut buf = Vec::new();
    let mut store = open_store(&mut buf)?;
    // Use a function to make sure all errors are dropped before Store drops.
    cli(&mut runner, &mut store);
    if let Err(error) = save_store(&store) {
        eprintln!("Failed to save store: {error}");
        eprintln!("Writing store to stdout");
        serde_json::to_writer(stdout(), &store).unwrap();
    }
    Ok(())
}

fn open_store(buf: &mut Vec<u8>) -> Result<Store, Error> {
    if let Ok(mut file) = File::open("./Store.cbor") {
        file.read_to_end(buf)?;
        Ok(serde_cbor::from_slice(buf)?)
    } else {
        eprintln!(r#"Failed to read Store from "./Store.cbor"."#);
        eprintln!("Loading empty Store");
        Ok(Store::default())
    }
}

fn save_store(store: &Store) -> Result<(), Error> {
    let file = File::create("./Store.cbor")?;
    serde_cbor::to_writer(file, &store)?;
    Ok(())
}

fn cli(runner: &mut AocRunner, store: &mut Store) {
    match Cli::parse().run(runner, store) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{error}");

            eprintln!("{error:?}");
        }
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    Serde(#[from] serde_cbor::Error),
}

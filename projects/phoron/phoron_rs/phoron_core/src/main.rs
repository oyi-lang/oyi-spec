use phoron_core::{deserializer::Deserializer, rw::reader::Reader};
use std::env;
use std::fs::File;

fn usage() {
    eprintln!("USAGE: phoron_core <CLASSFILE>");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<String>>();

    if args.len() != 1 {
        usage();
        std::process::exit(1);
    }

    let mut deserializer = Deserializer::new(Reader::new(File::open(&args[0])?));
    let classfile = deserializer.deserialize()?;
    println!("{:#?}", classfile);

    Ok(())
}
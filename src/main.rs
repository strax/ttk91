use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} filename.b91", &args[0]);
        process::exit(1)
    }
    let filename = &args[1];
    let path = Path::new(filename);
    match File::open(path) {
        Err(_) => {
            eprintln!("No such file: {}", path.display());
            process::exit(1)
        }
        Ok(f) => run(&f),
    }
}

fn run(f: &File) -> io::Result<()> {
    let mut reader = BufReader::new(f);
    io::copy(&mut reader, io::stdout().by_ref())?;
    Ok(())
}

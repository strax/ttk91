use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process;

mod ttk91;

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
        Ok(mut f) => run(&mut f),
    }
}

fn run(f: &mut File) -> io::Result<()> {
    // Read first 9 bytes to validate input
    let mut header = [0; 9];
    f.read(&mut header)?;
    if !ttk91::vm::validate_header(&header) {
        eprintln!("TTK91 header not detected");
        process::exit(1)
    }
    Ok(())
}

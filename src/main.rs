extern crate byteorder;
#[macro_use]
extern crate num_derive;
extern crate num_traits;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::Path;
use std::process;

mod b91;
mod vm;

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
    // Read program to memory
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data)?;
    let mut memory = [0u32; 32768];
    // let op = ttk91::vm::next_op(&mut cursor)?;
    // ttk91::vm::eval(&mut vm, &data)
    // println!("{:?}", op);
    let object_module = b91::parser::parse(&data);
    println!("{:?}", object_module);
    let mut machine = vm::Machine::new(100);
    machine.load_object_module(&object_module);
    machine.run();
    println!("{:?}", machine);
    Ok(())
}

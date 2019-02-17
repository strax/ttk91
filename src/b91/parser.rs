use std::error;
use std::io::{Cursor, Read, BufRead, Lines};
use std::result;
use std::collections::HashMap;
use super::{ObjectModule, CodeBlock, DataBlock, SymbolTable};

type Result<T> = result::Result<T, Box<error::Error>>;

const TOKEN_HEADER: &'static str = "___b91___";
const TOKEN_CODE: &'static str = "___code___";
const TOKEN_DATA: &'static str = "___data___";
const TOKEN_SYMBOLTABLE: &'static str = "___symboltable___";
const TOKEN_END: &'static str = "___end___";

fn parse_tuple<'a>(s: &'a str) -> Option<(&'a str, &'a str)> {
  match s.split(' ').collect::<Vec<_>>()[..] {
    [start, end] => Some((start, end)),
    _ => None
  }
}

fn parse_offset_info(s: &str) -> (usize, usize) {
  match parse_tuple(s) {
    Some((start, end)) => (start.parse().unwrap(), end.parse().unwrap()),
    _ => panic!("Could not parse start and end offsets")
  }
}

fn parse_code_block(lines: &mut Iterator<Item = String>) -> CodeBlock {
  let (start, end) = parse_offset_info(lines.next().unwrap().as_str());
  let instructions = lines
    .take(end - start + 1)
    .map(|s| s.parse().expect(&format!("Invalid instruction {}", s)))
    .collect::<Vec<u32>>()
    .into_boxed_slice();
  CodeBlock { start, end, instructions }
}

fn parse_data_block(lines: &mut Iterator<Item = String>) -> DataBlock {
  let (start, end) = parse_offset_info(lines.next().unwrap().as_str());
  let data = lines
    .take(end - start + 1)
    .map(|s| s.parse().expect(&format!("Invalid data value {}, expected an unsigned integer instead", s)))
    .collect::<Vec<u32>>()
    .into_boxed_slice();
  DataBlock { start, end, data }
}

fn parse_symbol_table(lines: &mut Iterator<Item = String>) -> SymbolTable {
  let mut table = HashMap::new();
  for line in lines.take_while(|s| s != TOKEN_END) {
    let (key, value) = parse_tuple(&line).unwrap();
    table.insert(key.to_string(), value.parse().unwrap());
  }
  table
}

pub fn parse(data: &[u8]) -> ObjectModule {
  let mut lines = Cursor::new(data).lines().map(|x| x.unwrap());
  assert!(lines.next().unwrap() == TOKEN_HEADER, "Object file header missing");

  // Parse code block
  assert!(lines.next().unwrap() == TOKEN_CODE, "Expected ___code___");
  let code = parse_code_block(&mut lines);

  assert!(lines.next().unwrap() == TOKEN_DATA, "Expected ___data___");
  let data = parse_data_block(&mut lines);

  assert!(lines.next().unwrap() == TOKEN_SYMBOLTABLE, "Expected ___symboltable___");
  let symbol_table = parse_symbol_table(&mut lines);

  ObjectModule { code, data, symbol_table }
}
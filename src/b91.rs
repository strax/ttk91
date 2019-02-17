pub mod parser;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct ObjectModule {
  pub code: CodeBlock,
  pub data: DataBlock,
  pub symbol_table: SymbolTable
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeBlock {
  pub start: usize,
  pub end: usize,
  pub instructions: Box<[u32]>
}

#[derive(Debug, PartialEq, Eq)]
pub struct DataBlock {
  pub start: usize,
  pub end: usize,
  pub data: Box<[u32]>
}

pub type SymbolTable = HashMap<String, usize>;
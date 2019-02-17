use crate::vm;

use vm::ops::Op;
use vm::AddressingMode;
use num_traits::FromPrimitive;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Instruction {
  pub opcode: Op,
  pub rj: u8,
  pub m: AddressingMode,
  pub ri: u8,
  pub addr: u16,
}

impl Instruction {
  pub fn from_u32(n: u32) -> Instruction {
    let opcode = Op::from_u8((n >> 24) as u8).unwrap();
    let rj = ((n >> 21) & 0b111) as u8;
    let m = AddressingMode::from_u8(((n >> 19) & 0b11) as u8).unwrap();
    let ri = ((n >> 16) & 0b111) as u8;
    let addr = n as u16;
    Instruction { opcode, rj, m, ri, addr }
  }
}
mod ops;

const HEADER: &[u8] = b"___b91___";

pub fn validate_header(input: &[u8]) -> bool {
  &input[0..9] == HEADER
}

#[derive(Debug)]
pub struct State {
  // Registers R0-R7
  pub r0: u32,
  pub r1: u32,
  pub r2: u32,
  pub r3: u32,
  pub r4: u32,
  pub r5: u32,
  // Stack Pointer = R6
  pub sp: u32,
  // Frame Pointer = R7
  pub fp: u32,

  // Control unit registers
  // Program Counter
  pub pc: u32,
  // Instruction Register
  pub ir: u32,
  // Temporary Register
  pub tr: u32,
  // State Register
  pub sr: u32,
}

impl State {
  pub fn new() -> State {
    State {
      r0: 0,
      r1: 0,
      r2: 0,
      r3: 0,
      r4: 0,
      r5: 0,
      sp: 0,
      fp: 0,
      pc: 0,
      ir: 0,
      tr: 0,
      sr: 0,
    }
  }
}

#[derive(Debug)]
#[repr(u8)]
pub enum AddressingMode {
  Immediate = 0b00,
  Direct = 0b01,
  Indirect = 0b10,
}

#[derive(Debug)]
pub struct Instruction {
  oper: ops::Op,
  rj: u8,
  m: AddressingMode,
  ri: u8,
  addr: u16,
}

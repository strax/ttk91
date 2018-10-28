use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RegisterFile {
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

impl RegisterFile {
  pub fn new() -> RegisterFile {
    RegisterFile {
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

  pub fn set(&mut self, dst: u8, value: u32) {
    *(&mut self[dst]) = value;
  }

  pub fn get(&self, src: u8) -> u32 {
    self[src]
  }
}

impl Index<u8> for RegisterFile {
  type Output = u32;

  fn index(&self, index: u8) -> &u32 {
    match index {
      0 => &0,
      1 => &self.r1,
      2 => &self.r2,
      3 => &self.r3,
      4 => &self.r4,
      5 => &self.r5,
      6 => &self.sp,
      7 => &self.fp,
      _ => panic!("Invalid register number"),
    }
  }
}

impl IndexMut<u8> for RegisterFile {
  fn index_mut(&mut self, i: u8) -> &mut u32 {
    match i {
      0 => &mut self.r0,
      1 => &mut self.r1,
      2 => &mut self.r2,
      3 => &mut self.r3,
      4 => &mut self.r4,
      5 => &mut self.r5,
      6 => &mut self.sp,
      7 => &mut self.fp,
      _ => panic!("Invalid register number"),
    }
  }
}

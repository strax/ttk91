use byteorder::{BigEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use std::io::{Cursor, Read, Result};
use b91::*;

pub mod ops;
pub mod instruction;
pub mod mmu;

use self::mmu::MMU;

#[derive(Debug)]
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
}

#[derive(Debug, FromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum AddressingMode {
  Immediate = 0b00,
  Direct = 0b01,
  Indirect = 0b10,
}

#[derive(Debug)]
pub struct Machine {
  registers: Box<RegisterFile>,
  mmu: MMU
}

impl Machine {
  pub fn new(memory_size: usize) -> Machine {
    let mmu = MMU::new(memory_size);
    let registers = Box::new(RegisterFile::new());
    Machine { mmu, registers }
  }

  pub fn load_object_module(&mut self, object_module: &ObjectModule) -> () {
    // Load instructions into memory
    let CodeBlock {start, end, instructions} = &object_module.code;
    self.mmu.as_slice()[*start..(*end + 1)].copy_from_slice(instructions);
    // Init FP
    self.registers.fp = object_module.code.end as u32;

    // Load data into memory
    let DataBlock { start, end, data } = &object_module.data;
    self.mmu.as_slice()[*start..(*end + 1)].copy_from_slice(data);
    // Init SP
    self.registers.sp = object_module.data.end as u32;
  }

  pub fn run(&mut self) -> () {
    let mut pc = 0;
    while pc <= self.registers.fp {
      // Fetch instruction
      let instruction = instruction::Instruction::from_u32(self.mmu.read(pc as usize));
      println!("{:?}", instruction);
      pc += 1;
    }
  }
}
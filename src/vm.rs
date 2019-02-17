use crate::b91::*;
use num_derive::{FromPrimitive};
use num_traits::{FromPrimitive};
use std::process;

pub mod debugger;
pub mod instruction;
pub mod io;
pub mod mmu;
pub mod ops;
pub mod register_file;
pub mod supervisor;
mod control;

use self::instruction::*;
use self::io::*;
use self::mmu::*;
use self::ops::*;
use self::register_file::*;
use self::supervisor::*;
use self::control::Control;

pub trait Hypervisor {
  fn run(&mut self) -> ();
}

#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum AddressingMode {
  Immediate = 0b00,
  Direct = 0b01,
  Indirect = 0b10,
}

#[derive(Debug)]
pub struct Machine {
  registers: Box<RegisterFile>,
  mmu: MMU,
}

impl Hypervisor for Machine {
  fn run(&mut self) -> () {
    while self.registers.pc <= self.registers.fp {
      self.tick();
    }
  }
}

impl Machine {
  pub fn new(memory_size: usize) -> Machine {
    let mmu = MMU::new(memory_size);
    let registers = Box::new(RegisterFile::new());
    Machine { mmu, registers }
  }

  pub fn load_object_module(&mut self, object_module: &ObjectModule) -> () {
    // Load instructions into memory
    let CodeBlock {
      start,
      end,
      instructions,
    } = &object_module.code;
    self.mmu.as_slice()[*start..(*end + 1)].copy_from_slice(instructions);
    // Init FP
    self.registers.fp = object_module.code.end as u32;

    // Load data into memory
    let DataBlock { start, end, data } = &object_module.data;
    self.mmu.as_slice()[*start..(*end + 1)].copy_from_slice(data);
    // Init SP
    self.registers.sp = object_module.data.end as u32;
  }

  /// Executes a single tick of the CPU.
  pub fn tick(&mut self) -> () {
    let instruction = Instruction::from_u32(self.mmu.read(self.registers.pc as usize));
    self.registers.pc += 1;
    self.execute(&instruction);
  }

  fn fetch_value(&mut self, instruction: &Instruction) -> u32 {
    let base = self.registers[instruction.ri];
    let offset = instruction.addr as u32;
    let ea = base + offset;
    match instruction.m {
      AddressingMode::Immediate => ea,
      AddressingMode::Direct => self.mmu.read(ea as usize),
      AddressingMode::Indirect => {
        let ea = self.mmu.read(ea as usize);
        self.mmu.read(ea as usize)
      }
    }
  }
}

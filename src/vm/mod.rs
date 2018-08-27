use b91::*;
use byteorder::{BigEndian, ReadBytesExt};
use num_traits::FromPrimitive;
use std::io::{Cursor, Read, Result};
use std::process;

pub mod alu;
pub mod instruction;
pub mod io;
pub mod mmu;
pub mod ops;
pub mod register_file;
pub mod supervisor;

use self::instruction::*;
use self::io::*;
use self::mmu::*;
use self::ops::*;
use self::register_file::*;
use self::supervisor::*;

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

  pub fn run(&mut self) -> () {
    while self.registers.pc <= self.registers.fp {
      println!("=> {:?}", self.registers);
      // Fetch instruction
      let instruction = Instruction::from_u32(self.mmu.read(self.registers.pc as usize));
      self.registers.pc += 1;
      self.execute(&instruction);
    }
  }

  fn execute(&mut self, instruction: &Instruction) -> () {
    println!("{:?} {:?}", self.registers.pc, instruction);
    match instruction.opcode {
      Op::NOP => (),
      Op::IN => {
        let device = Device::from_u32(self.fetch_value(instruction)).unwrap();
        match device {
          Device::Keyboard => {
            let value = read_keyboard();
            let rj = &mut self.registers[instruction.rj];
            *rj = value;
          }
          _ => panic!("Invalid device"),
        }
      }
      Op::STORE => {
        let source = self.registers[instruction.rj];
        let destination = self.fetch_value(instruction);
        self.mmu.write(destination as usize, source);
      }
      Op::LOAD => {
        let x = self.fetch_value(instruction);
        self.registers[instruction.rj] = x;
      }
      Op::OUT => {
        let device = Device::from_u32(self.fetch_value(instruction)).unwrap();
        let x = self.registers[instruction.rj];
        match device {
          Device::CRT => println!("{:?}", x),
          _ => panic!("Invalid device"),
        }
      }
      Op::JZER => {
        let x = self.registers[instruction.rj];
        if x == 0 {
          self.registers.pc = self.fetch_value(instruction);
        }
      }
      Op::JUMP => {
        self.registers.pc = self.registers[instruction.rj];
      }
      Op::ADD => {
        let b = self.fetch_value(&instruction);
        let mut a = &mut (self.registers[instruction.rj]);
        println!("ALU: ADD {:p} {}", a, b);
        alu::add(a, b)
      }
      Op::SVC => {
        let service = Service::from_u32(self.fetch_value(instruction)).unwrap();
        match service {
          Service::Halt => {
            println!("Process halted");
            process::exit(0)
          }
          _ => panic!("Unknown supervisor call"),
        }
      }
      _ => panic!(format!("No handler for opcode {:?}", instruction.opcode)),
    }
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

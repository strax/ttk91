use super::{Machine, Instruction, Op, Device, Service};
use std::process;
use num_traits::{FromPrimitive};

pub trait Control {
    fn execute(&mut self, instruction: &Instruction) -> ();
}

impl Control for Machine {
    fn execute(&mut self, instruction: &Instruction) -> () {
        match instruction.opcode {
            Op::NOP => (),
            Op::IN => {
                /*let device = Device::from_u32(machine.fetch_value(instruction)).unwrap();
                match device {
                    Device::Keyboard => {
                        let value = read_keyboard();
                        machine.registers.set(instruction.rj, value);
                    }
                    _ => panic!("Invalid device"),
                }*/
                unimplemented!()
            }
            Op::STORE => {
                let source = self.registers[instruction.rj];
                let destination = self.fetch_value(instruction);
                self.mmu.write(destination as usize, source);
            }
            Op::LOAD => {
                let value = self.fetch_value(instruction);
                self.registers.set(instruction.rj, value);
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
                let a = self.fetch_value(&instruction);
                let b = self.registers.get(instruction.rj);
                self.registers.set(instruction.rj, b + a);
            }
            Op::SUB => {
                let a = self.fetch_value(&instruction);
                let b = self.registers.get(instruction.rj);
                self.registers.set(instruction.rj, b - a);
            }
            Op::MUL => {
                let a = self.fetch_value(&instruction);
                let b = self.registers.get(instruction.rj);
                self.registers.set(instruction.rj, b * a);
            }
            Op::DIV => {
                let a = self.fetch_value(&instruction);
                let b = self.registers.get(instruction.rj);
                self.registers.set(instruction.rj, b / a);
            }
            Op::SVC => {
                let service = Service::from_u32(self.fetch_value(instruction)).unwrap();
                match service {
                    Service::Halt => {
                        println!("(process halted)");
                        process::exit(0)
                    }
                    _ => panic!("Unknown supervisor call"),
                }
            }
            _ => panic!(format!("No handler for opcode {:?}", instruction.opcode)),
        }
    }
}


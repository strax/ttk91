use io;
use prettytable::{Cell, Row, Table};
use process;
use vm::instruction::Instruction;
use vm::register_file::RegisterFile;
use vm::Hypervisor;
use vm::Machine;

pub struct Debugger<'a> {
  machine: &'a mut Machine,
}

enum Command {
  Next(),
  Exit(),
  Regs(),
  Ins(),
  Help(),
}

impl<'a> Debugger<'a> {
  pub fn new(machine: &'a mut Machine) -> Debugger<'a> {
    Debugger { machine }
  }

  fn read_command(&self) -> Option<Command> {
    eprint!("{}> ", self.machine.registers.pc);
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    match buf.as_ref() {
      "next" | "n" => Some(Command::Next()),
      "exit" | "q" => Some(Command::Exit()),
      "regs" | "r" => Some(Command::Regs()),
      "ins" | "i" => Some(Command::Ins()),
      "help" => Some(Command::Help()),
      _ => None,
    }
  }

  fn next_command(&self) -> Command {
    self.read_command().unwrap_or_else(|| self.next_command())
  }

  fn print_registers(&self) -> () {
    let mut table = Table::new();
    // Add a row per time
    table.add_row(row!["Register", "Value"]);
    table.add_row(row!["R0", format!("{:#010x}", self.machine.registers.r0)]);
    table.add_row(row!["R1", format!("{:#010x}", self.machine.registers.r1)]);
    table.add_row(row!["R2", format!("{:#010x}", self.machine.registers.r2)]);
    table.add_row(row!["R3", format!("{:#010x}", self.machine.registers.r3)]);
    table.add_row(row!["R4", format!("{:#010x}", self.machine.registers.r4)]);
    table.add_row(row!["R5", format!("{:#010x}", self.machine.registers.r0)]);
    table.add_row(row!["SP", format!("{:#010x}", self.machine.registers.sp)]);
    table.add_row(row!["FP", format!("{:#010x}", self.machine.registers.fp)]);
    table.add_row(row!["PC", format!("{:#010x}", self.machine.registers.pc)]);
    table.add_row(row!["IR", format!("{:#010x}", self.machine.registers.ir)]);
    table.add_row(row!["TR", format!("{:#010x}", self.machine.registers.tr)]);
    table.add_row(row!["SR", format!("{:#010x}", self.machine.registers.sr)]);
    // Print the table to stdout
    table.printstd();
  }

  fn print_instruction(&mut self) -> () {
    let instruction =
      Instruction::from_u32(self.machine.mmu.read(self.machine.registers.pc as usize));
    println!("{:?}", instruction);
  }

  fn print_help(&self) -> () {
    let message = r#"
Available commands:
next          Execute the current instruction and increment PC by 1
exit          Quit the process
regs          Show the machine's register file
ins           Show the instruction pointed to by PC
help          Show this help text
"#;

    println!("{}", message);
  }
}

impl<'a> Hypervisor for Debugger<'a> {
  fn run(&mut self) -> () {
    loop {
      match self.next_command() {
        Command::Next() => self.machine.tick(),
        Command::Exit() => process::exit(0),
        Command::Regs() => self.print_registers(),
        Command::Ins() => self.print_instruction(),
        Command::Help() => self.print_help(),
      }
    }
  }
}

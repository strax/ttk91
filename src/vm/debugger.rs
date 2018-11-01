use io;
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
      "exit" => Some(Command::Exit()),
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
    println!("{:?}", self.machine.registers);
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

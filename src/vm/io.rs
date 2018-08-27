use std::io;

#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
pub enum Device {
  CRT = 0,
  Keyboard = 1,
}

pub fn read_keyboard() -> u32 {
  eprint!("(input): ");
  let mut buf = String::new();
  io::stdin().read_line(&mut buf).unwrap();
  buf.pop();
  buf.parse().unwrap()
}

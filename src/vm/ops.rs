#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Op {
  NOP = 0x00,
  STORE = 0x01,
  LOAD = 0x02,
  IN = 0x03,
  OUT = 0x04,

  ADD = 0x11,
  SUB = 0x12,
  MUL = 0x13,
  DIV = 0x14,
  MOD = 0x15,

  AND = 0x16,
  OR = 0x17,
  XOR = 0x18,
  SHL = 0x19,
  SHR = 0x1a,
  NOT = 0x1b,
  SHRA = 0x1c,

  COMP = 0x1f,

  JUMP = 0x20,
  JNEG = 0x21,
  JZER = 0x22,
  JPOS = 0x23,
  JNNEG = 0x24,
  JNZER = 0x25,
  JNPOS = 0x26,

  JLES = 0x27,
  JEQU = 0x28,
  JGRE = 0x29,
  JNLES = 0x2A,
  JNEQU = 0x2B,
  JNGRE = 0x2C,

  CALL = 0x31,
  EXIT = 0x32,
  PUSH = 0x33,
  POP = 0x34,
  PUSHR = 0x35,
  POPR = 0x36,

  SVC = 0x70,
}
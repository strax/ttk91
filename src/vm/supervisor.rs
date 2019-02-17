use num_derive::{FromPrimitive};

#[derive(Debug, FromPrimitive, Eq, PartialEq, Copy, Clone)]
pub enum Service {
  Halt = 11,
}

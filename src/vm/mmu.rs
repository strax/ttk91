// Memory Management Unit
#[derive(Debug)]
pub struct MMU {
  pub mar: u32,
  pub mbr: u32,
  pub limit: u32,
  backing: Box<[u32]>
}

impl MMU {
  pub fn new(size: usize) -> MMU {
    let backing = vec![0u32;size].into_boxed_slice();
    MMU { backing, mar: 0, mbr: 0, limit: size as u32 }
  }

  pub fn read(&mut self, addr: usize) -> u32 {
    if addr < self.limit as usize {
      self.mar = addr as u32;
      self.mbr = self.backing[addr];
      self.mbr
    } else {
      // TODO: Set exception flag
      panic!("Address out of bounds")
    }
  }

  pub fn write(&mut self, addr: usize, value: u32) -> () {
    if addr < self.limit as usize {
      self.mar = addr as u32;
      self.mbr = value;
      self.backing[addr] = value;
    } else {
      panic!("Address out of bounds")
    }
  }

  pub fn as_slice(&mut self) -> &mut [u32] {
    self.backing.as_mut()
  }
}
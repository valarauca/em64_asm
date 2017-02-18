



///Describ the size of a register
#[repr(u8)]
#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum RegSize {
  Bit8,
  Bit16,
  Bit32,
  Bit64,
	Bit80,
	BitMMX,
  Bit128,
  Bit256,
  Bit512
}
impl RegSize {
  #[inline(always)]
  pub fn byte_size(&self) -> usize {
    match self {
      &RegSize::Bit8 => 1,
      &RegSize::Bit16 => 2,
      &RegSize::Bit32 => 4,
      &RegSize::Bit64 => 8,
	    &RegSize::Bit80 => 10,
	    &RegSize::BitMMX => 8,
      &RegSize::Bit128 => 16,
      &RegSize::Bit256 => 32,
      &RegSize::Bit512 => 64
    }
  }
}
    

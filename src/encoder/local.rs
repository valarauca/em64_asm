
use super::super::fnv::FnvHasher;
use super::super::args::RegSize;

use std::default::Default;
use std::hash::Hasher;

/// Used to buffer ops
/// 
/// Encodes metadata about the opcode in question so they can be
/// better aligned and packed to account for decode stalls, and
/// chip oddities.
pub struct LocalBuff {
  prefix_stall: bool,
  max: usize,
  min: usize,
  nop: bool,
  val: Option<RegSize>,
  data: Vec<u8>,
  hash: u64,
}
impl Default for LocalBuff {
  #[inline]
  fn default() -> LocalBuff {
    LocalBuff {
      hash: 0,
      prefix_stall: false,
      max: 0,
      min: 0,
      nop: false,
      val: None,
      data: Vec::with_capacity(16)
    }
  }
}
impl Local {
  #[inline]
  pub fn prefix_stall(&self) -> bool {
    self.prefix_stall.clone()
  }
  #[inline]
  pub fn get_nop_request(&self) -> bool {
    self.nop.clone()
  }
  #[inline]
  pub fn get_uops(&self) -> (usize,usize) {
    (self.max.clone(),self.min.clone())
  }
  #[inline]
  pub fn get_encoded_val(&self) -> usize {
    match &self.val {
      &Option::None => 0,
      &Option::Some(RegSize::Bit8) => 1,
      &Option::Some(RegSize::Bit16) => 2,
      &Option::Some(RegSize::Bit32) => 4,
      &Option::Some(_) => 8,
    }
  }
  #[inline]
  pub fn get_hash(&self) -> u64 {
    self.hash.clone()
  }
  #[inline]
  pub fn len(&self) -> usize {
    self.data.len()
  }
  #[inline]
  pub fn as_slice<'a>(&'a self) -> &'a [u8] {
    self.data.as_slice()
  }
  #[inline]
  pub fn as_mut_vec<'a>(&'a mut self) -> &'a mut Vec<u8> {
    &mut self.data
  }

  #[inline]
  pub fn hash_data(&mut self) {
    let mut fnv = FnvHasher::default();
    let hash: u64 = {
      fnv.write(self.as_slice());
      fnv.finish()
    };
    self.hash = hash;
  }


  /// clears a buffer to be re-cycled
  pub fn reset(&mut self) {
    self.hash = 0;
    self.pref_stall = false;
    self.max = 0;
    self.min = 0;
    self.nop = false;
    self.val = None;
    self.data.truncate(0);
  }
}


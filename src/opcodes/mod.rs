
use super::args::{
  Access,
  parse_z0,
  parse_i,
  parse_mi,
  parse_mr,
  parse_rm,
  parse_rr,
  parse_rrvv,
  parse_rvm,
  parse_rvmvv,
  Disp,
  Register,
  Mem,
};

use super::platform::{
  Arch,
  Encode,
  Ext,
  Generic
};

mod a;

pub trait Opcode<Arch>: Sized+Clone {
  fn nop_after(&self) -> bool;
  fn max_uops(&self) -> usize;
  fn min_uops(&self) -> usize;
  fn encode(&self, x: &mut Vec<u8>);
}


#[derive(Clone)]
pub struct Op<A: Arch,O: Opcode<A>> {
  data: O,
}

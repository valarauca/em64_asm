
use super::super::{
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
  Arch,
  Encode,
  Ext,
  Disp,
  Mem,
};

use std::mem;


pub struct ADC {
  lock: bool,
  data: Access
}
impl Opcode<Generic> for ADC {
  fn nop_after(&self) -> bool {
    false
  }
  fn max_uops(&self) -> usize {
    4
  }
  fn min_uops(&self) -> usize {
    2
  }
  fn encode(&self, x: &mut Vec<u8>) {
    match self.data
      &Access::I(Disp::I8(ref val)) => {
        x.push(0x14);
        x.extend_from_slice(val.to_slice());
      },
      &Access::I(Disp::I16(ref val)) => {
        x.push(0x15);
        x.extend_from_slice(val.to_slice());
      },
      &Access::I(Disp::I32(ref val)) => {
        x.push(0x15);
        x.extend_from_slice(val.to_slice());
      },
      &Access::MI(Mem::Value(Register::rax),Disp::I32(ref val)) => {
        x.push(0x48);
        x.push(0x15);
        x.extend_from_slice(val.to_slice());
      },
}

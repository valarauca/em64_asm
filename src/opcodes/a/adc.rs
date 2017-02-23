
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
  Encode,
  Ext,
  Disp,
  Mem,
  Opcode,
  RegSize
};

use std::mem;


pub struct ADC {
  lock: bool,
  data: Access
}
impl OpCode {
 
  fn u_op_count(&self) -> usize {
    match &self.data {
      &Access::I(_) |
      &Access::MI(Mem::Value(_),_) |
      &Access::MR(Mem::Value(_),_) |
      &Access::RM(_,Mem::Value(_)) |
      &Access::RR(_,_) => 1,
      _ => 2
    }
  }

  fn set_offset(&mut self, x: Disp) -> Result<(),()> {
    Err(())
  }

  fn is_nop_after(&self) -> bool {
    false
  }

  fn is_lcp(&self) -> bool {
    false
  }

  fn is_large_uop(&self) -> bool {
    match &self.data {
      &Access::I(Disp::I32(_)) |
      &Access::MI(_,Disp::I32(_)) => true,
      _ => false
    }
  }

  fn is_branch(&self) -> bool {
    false
  }

  fn is_external_call(&self) -> bool {
    false
  }

  fn len(&self, buffer: &mut Vec<u8>) -> Result<usize,()> {
    match &self.data {
      &Access::I(Disp::I8(_)) => 2,
      &Access::I(Disp::I16(_)) => 3,
      &Access::I(Disp::I32(_)) => 5,
      &Access::I(Disp::
        
      

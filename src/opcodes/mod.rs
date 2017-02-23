
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
  RegSize,
};

use super::platform::{
  Encode,
  Ext,
};

mod a;

/// Details how the Opcode is encoded
pub trait Opcode {

  /// How many uOPs is the instruction
  fn u_op_count(&self) -> usize;

  /// How many bytes is this instruction
  fn len(&self) -> Result<usize,()>;

  /// If this is a branch this lets you set its offset
  fn set_offset(&mut self, x: Disp) -> Result<(),()>;
  
  /// Should a NOP be encoded after this
  fn is_nop_after(&self) -> bool;

  /// Does this have a Length Change Prefix
  fn is_lcp(&self) -> bool;

  /// Does this encode a large DISP/IMM data which
  /// may go over a cache line
  fn is_large_uop(&self) -> bool;

  /// Is a branch
  fn is_branch(&self) -> bool;
  
  /// I have no clue how to handle this
  fn is_external_call(&self) -> bool;

  /// Encode the operation into a buffer
  fn encode_op(&self, buffer: &mut Vec<u8>) -> Result<(),()>;
}


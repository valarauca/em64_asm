


/// Describs an operation
pub trait Op<Arch>: Sized {

  /// How large the op will be when encoded (in bytes)
  fn encoded_len(&self) -> usize;

  /// Should a NOP be added afterwards to improve decoding
  fn require_nop_after(&self) -> bool;

  fn max_micro_ops(&self) -> usize;
  fn min_micro_ops(&self) -> usize;

  /// Can this op be fused with other ops
  fn macro_op_fusion(&self) -> Option<&'static [Self]>;
}


use super::Register;


/// Faults that may occur in this module
pub enum Fault {

  /// Register is wrongly sized
  TooSmall(Register),
  
  /// Memory reference is void
  VoidRef,

  /// Register sizes in a Mem aren't the same size
  NotEqual(Register,Register)
}

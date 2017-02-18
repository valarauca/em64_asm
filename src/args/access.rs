
use super::{
  Register,
  Mem
};

pub enum Access {
  Z0,
  I(Disp),
  RM(Register,Mem),
  MR(Register,Register),
  MI(Mem, Disp),
  RVM(Register,Register,Mem),
  FV(Register,Register, Mem),
}

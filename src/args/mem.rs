
use super::{
  Register,
  Disp,
  Scale,
  RegSize
};

/// Faults that may occur in Register Representation
pub enum MemFault {
  TooSmall(Register),
  NotEqual(Register,Register),
  VoidRef
}

/// All the fun x64 Address Generation forms
pub enum Mem {
  Value(Register),
  Direct(Register),
  DirectDisp(Register,Disp),
  Scaled(Register,Scale,Disp),
  Complex(Register,Register,Scale,Disp),
  Raw(Disp)
}
impl Mem {
  
  /*
   * Make assumptions about Memory
   *
   */
  pub fn is_value(&self) -> bool {
    match self {
      &Mem::Value(_) => true,
      _ => false
    }
  }
  pub fn is_direct_ref(&self) -> bool {
    match self {
      &Mem::Direct(_) => true,
      _ => false
    }
  }
  pub fn is_raw_ref(&self) -> bool {
    match self {
      &Mem::Raw(_) => true,
      _ => false
    }
  }
  pub fn is_direct_disp_ref(&self) -> bool {
    match self {
      &Mem::DirectDisp(_,_) => true,
      _ => false
    }
  }
  pub fn is_complex_ref(&self) -> bool {
    match self {
      &Mem::Complex(_,_,_,_) => true,
      _ => false
    }
  }
  pub fn is_disp_none(&self) -> bool {
    match self {
      &Mem::Value(_) |
      &Mem::Direct(_) |
      &Mem::DirectDisp(_,Disp::None) |
      &Mem::Raw(Disp::None) |
      &Mem::Scaled(_,_,Disp::None) |
      &Mem::Complex(_,_,_,Disp::None) => true,
      _ => false
    }
  }

  /*
   * Get internal data
   *
   */
  pub fn get_value(&self) -> Option<Register> {
    match self {
      &Mem::Value(ref r) => Some(r.clone()),
      _ => None
    }
  }
  pub fn get_direct(&self) -> Option<Register> {
    match self {
      &Mem::Direct(ref r) => Some(r.clone()),
      _ => None
    }
  }
  pub fn get_raw(&self) -> Option<Disp> {
    match self {
      &Mem::Raw(ref e) => Some(e.clone()),
      _ => None
    }
  }
  pub fn get_base(&self) -> Option<Register> {
    match self {
      &Mem::Complex(ref b,_,_,_,) => Some(b.clone()),
      _ => None
    }
  }
  pub fn get_index(&self) -> Option<Register> {
    match self {
      &Mem::Complex(_, ref i, _, _) |
      &Mem::Scaled(ref i, _, _) => Some(i.clone()),
      _ => None
    }
  }

  /*
   * Do actual work
   */
  pub fn get_reg_size(&self) -> Result<RegSize,MemFault> {
    match self {
      &Mem::Value(ref r) |
      &Mem::Direct(ref r) |
      &Mem::Scaled(ref r, _, _) |
      &Mem::DirectDisp(ref r, _) => Ok(r.size()),
      &Mem::Complex(ref b, ref i, _, _) => {
        let base = b.size();
        let index = i.size();
        if base == index {
          Ok(base)
        } else {
          Err(MemFault::NotEqual(b.clone(),i.clone()))
        }
      }
      &Mem::Raw(ref r) => match r.size_of() {
        Option::None => Err(MemFault::VoidRef),
        Option::Some(s) => Ok(s)
      }
    }
  }

}

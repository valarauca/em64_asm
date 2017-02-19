
use super::{
  Register,
  Disp,
  Scale,
  RegSize,
  Fault,
  parse_reg,
  parse_512bit_reg,
  parse_256bit_reg,
  parse_128bit_reg,
  parse_64bit_reg,
  parse_32bit_reg,
  parse_16bit_reg,
  parse_8bit_reg,
  parse_mmx_reg,
  parse_x87_reg,
  parse_vec_reg,
  parse_long_ptr_reg,
  parse_scale,
  parse_const
};


/// All the fun x64 Address Generation forms
#[derive(Clone,Debug,PartialEq,Eq)]
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
  pub fn get_reg_size(&self) -> Result<RegSize,Fault> {
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
          Err(Fault::NotEqual(b.clone(),i.clone()))
        }
      }
      &Mem::Raw(ref r) => match r.size_of() {
        Option::None => Err(Fault::VoidRef),
        Option::Some(s) => Ok(s)
      }
    }
  }
}

named!(ob, ws!(tag!(b"[")));
named!(cb, ws!(tag!(b"]")));
named!(add, ws!(tag!(b"+")));
named!(mul, ws!(tag!(b"*")));

named!(parse_mem_value<Mem>, do_parse!(
  a: ws!(parse_reg) >>
  (Mem::Value(a))
));
named!(parse_mem_direct<Mem>, do_parse!(
  ob >>
  a: ws!(parse_long_ptr_reg) >>
  cb >>
  (Mem::Direct(a))
));
type DispLoc = (Register,Disp);
named!(parse_disp_loc<DispLoc>, do_parse!(
  a: alt!(
    do_parse!(
      d: ws!(parse_const) >>
      add >>
      r: ws!(parse_long_ptr_reg) >>
      ( (r,d) )
    ) |
    do_parse!(
      r: ws!(parse_long_ptr_reg) >>
      add >>
      d: ws!(parse_const) >>
      ( (r,d) )
    )
  ) >>
  (a)
));
named!(parse_mem_disp<Mem>, do_parse!(
  ob >>
  a: ws!(parse_disp_loc) >>
  cb >>
  (Mem::DirectDisp(a.0, a.1))
));
named!(parse_complex_sum<Mem>, do_parse!(
  ob >>
  a: ws!(parse_long_ptr_reg) >>
  add >>
  b: ws!(parse_long_ptr_reg) >>
  cb >>
  (Mem::Complex(a,b,Scale::Val1,Disp::None))
));
named!(parse_complex_disp_sum<Mem>,do_parse!(
  ob >>
  a: alt!(
    do_parse!(
      i: ws!(parse_long_ptr_reg) >>
      add >>
      b: ws!(parse_long_ptr_reg) >>
      add >>
      d: ws!(parse_const) >>
      ( (i,b,d) )
    ) |
    do_parse!(
      i: ws!(parse_long_ptr_reg) >>
      add >>
      d: ws!(parse_const) >>
      add >>
      b: ws!(parse_long_ptr_reg) >>
      ((i,b,d))
    ) |
    do_parse!(
      d: ws!(parse_const) >>
      add >>
      i: ws!(parse_long_ptr_reg) >>
      add >>
      b: ws!(parse_long_ptr_reg) >>
      ((i,b,d))
    )
  ) >>
  cb >>
  (Mem::Complex(a.0,a.1,Scale::Val1,a.2))
));
type Scaled = (Register,Scale);
named!(parse_with_scaled<Scaled>, do_parse!(
  ws!(tag!(b"(")) >>
  a: alt!(
    do_parse!(
      r: ws!(parse_long_ptr_reg) >>
      mul >>
      s: ws!(parse_scale) >>
      ((r,s))
    )|
    do_parse!(
      s: ws!(parse_scale) >>
      mul >>
      r: ws!(parse_long_ptr_reg) >>
      ((r,s))
    )
  ) >>
  ws!(tag!(b")")) >>
  (a)
));
named!(parse_complex_scaled_no_disp<Mem>, do_parse!(
  ob >>
  a: alt!(
    do_parse!(
      r: ws!(parse_long_ptr_reg) >>
      add >>
      s: ws!(parse_with_scaled) >>
      (Mem::Complex(r,s.0,s.1,Disp::None))
    )|
    do_parse!(
      s: ws!(parse_with_scaled) >>
      add >>
      r: ws!(parse_long_ptr_reg) >>
      (Mem::Complex(r,s.0,s.1,Disp::None))
    )
  ) >>
  cb >>
  (a)
));
named!(parse_complex_everything<Mem>,do_parse!(
  ob >>
  a: alt!(
    do_parse!(
      s: ws!(parse_with_scaled) >>
      add >>
      d: ws!(parse_disp_loc) >>
      ((s.0,d.0,s.1,d.1))
    ) |
    do_parse!(
      d: ws!(parse_disp_loc) >>
      add >>
      s: ws!(parse_with_scaled) >>
      ((d.0,s.0,s.1,d.1))
    )|
    do_parse!(
      c: ws!(parse_const) >>
      add >>
      s: ws!(parse_with_scaled) >>
      add >>
      r: ws!(parse_long_ptr_reg) >>
      ((r,s.0,s.1,c))
    )|
    do_parse!(
      r: ws!(parse_long_ptr_reg) >>
      add >>
      s: ws!(parse_with_scaled) >>
      add >>
      c: ws!(parse_const) >>
      ((r,s.0,s.1,c))
    )
  ) >>
  cb >>
  (Mem::Complex(a.0,a.1,a.2,a.3))
));
      
named!(pub parse_mem<Mem>, do_parse!(
  a: alt!(
    complete!(parse_mem_value) |
    complete!(parse_mem_direct) |
    complete!(parse_mem_disp) |
    complete!(parse_complex_sum) |
    complete!(parse_complex_disp_sum) |
    complete!(parse_complex_scaled_no_disp) |
    complete!(parse_complex_everything)
  ) >>
  (a)
));

#[test]
fn test_mem_parsing() {
  
  macro_rules! gen_test {
    (@SHOULDFAIL
      Input: $dut: expr;
    ) => {
      let x = parse_mem($dut);
      if x.is_err() {
        //thumbs up
      } else {
        panic!("For {:?} this should be an error. Got {:?}", $dut, x);
      }
    };
    (
      Input: $dut: expr;
      Output: $val: expr;
    ) => {
      let x = parse_mem($dut);
      if x.is_done() {
        let (_,val) = x.unwrap();
        if val != $val {
          panic!("For {:?} expected {:?} got {:?}", $dut, $val, val);
        }
      } else {
        panic!("For {:?} expected {:?} got {:?}", $dut, $val, x);
      }
    };
  }

  /*
   * Raw Value Tests
   */
  gen_test! {
    Input: b" rax ";
    Output: Mem::Value(Register::rax);
  }
  gen_test! {
    Input: b" zmm30 ";
    Output: Mem::Value(Register::zmm30);
  }
  gen_test! {
    Input: b" st0 ";
    Output: Mem::Value(Register::st0);
  }
  gen_test! {
    Input: b" r8l ";
    Output: Mem::Value(Register::r8l);
  }


  gen_test! {
    Input: b" [ rdi ] ";
    Output: Mem::Direct(Register::rdi);
  }
  gen_test! {
    Input: b" [r12d] ";
    Output: Mem::Direct(Register::r12d);
  }
  gen_test! {
    Input: b" [eax]";
    Output: Mem::Direct(Register::eax);
  }
  gen_test! {@SHOULDFAIL
    Input: b" [xmm0] ";
  }
  gen_test! {@SHOULDFAIL
    Input: b" [ mmx5 ]";
  }
  gen_test! {@SHOULDFAIL
    Input: b" [st1]";
  }
  gen_test! {@SHOULDFAIL
    Input: b" [ al ] ";
  }


  gen_test! {
    Input: b" [ rax + 15 ] ";
    Output: Mem::DirectDisp(Register::rax, Disp::I8(15i8));
  }
  gen_test! {
    Input: b" [r12w+ -1]";
    Output: Mem::DirectDisp(Register::r12w, Disp::I8(-1i8));
  }
}



/// The value of the Scale bits in a SIB byte
#[repr(u8)]
#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Scale {
  Val1 = 0,
  Val2 = 64,
  Val4 = 128,
  Val8 = 192
}
impl Into<u8> for Scale {
  #[inline(always)]
  fn into(self) -> u8 {
    use std::mem;
    unsafe{ mem::transmute(self) }
  }
}
impl Scale {
  /// Start building the SIB value
  #[inline(always)]
  pub fn to_sib_seed(&self) -> u8 {
    self.clone().into()
  }
}

named!(pub parse_scale<Scale>, do_parse!(
  x: alt!(
    do_parse!(tag!(b"1") >> (Scale::Val1)) |
    do_parse!(tag!(b"2") >> (Scale::Val2)) |
    do_parse!(tag!(b"4") >> (Scale::Val4)) |
    do_parse!(tag!(b"8") >> (Scale::Val8))
  ) >>
  (x)
));

#[test]
fn test_into_sib() {
  assert_eq!(Scale::Val1.to_sib_seed(), 0b0000_0000u8);
  assert_eq!(Scale::Val2.to_sib_seed(), 0b0100_0000u8);
  assert_eq!(Scale::Val4.to_sib_seed(), 0b1000_0000u8);
  assert_eq!(Scale::Val8.to_sib_seed(), 0b1100_0000u8);
}

#[test]
fn test_parse_scale() {
  
  use super::super::super::nom::IResult;

  let dut = b"1 ";
  let expect = Scale::Val1;
  match parse_scale(dut) {
    IResult::Done(rem,val) => {
      assert_eq!(rem, b" ");
      assert_eq!(val, expect);
    },
    IResult::Incomplete(n) => {
      panic!("Parse error {:?} expected {:?}", n, expect);
    },
    IResult::Error(e) => {
      panic!("Parse error {:?} expected {:?}", e, expect);
    }
  };
  
  let dut = b"2 ";
  let expect = Scale::Val2;
  match parse_scale(dut) {
    IResult::Done(rem,val) => {
      assert_eq!(rem, b" ");
      assert_eq!(val, expect);
    },
    IResult::Incomplete(n) => {
      panic!("Parse error {:?} expected {:?}", n, expect);
    },
    IResult::Error(e) => {
      panic!("Parse error {:?} expected {:?}", e, expect);
    }
  };


  let dut = b"4 ";
  let expect = Scale::Val4;
  match parse_scale(dut) {
    IResult::Done(rem,val) => {
      assert_eq!(rem, b" ");
      assert_eq!(val, expect);
    },
    IResult::Incomplete(n) => {
      panic!("Parse error {:?} expected {:?}", n, expect);
    },
    IResult::Error(e) => {
      panic!("Parse error {:?} expected {:?}", e, expect);
    }
  };

  let dut = b"8 ";
  let expect = Scale::Val8;
  match parse_scale(dut) {
    IResult::Done(rem,val) => {
      assert_eq!(rem, b" ");
      assert_eq!(val, expect);
    },
    IResult::Incomplete(n) => {
      panic!("Parse error {:?} expected {:?}", n, expect);
    },
    IResult::Error(e) => {
      panic!("Parse error {:?} expected {:?}", e, expect);
    }
  };
}



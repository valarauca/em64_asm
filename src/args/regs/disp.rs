use super::super::super::nom::{
  is_digit,
  IResult,
  ErrorKind,
  Needed
};
use super::RegSize;
use std::default::Default;
use std::i8;
use std::i16;
use std::i32;
use std::i64;
use std::str;
use std::mem;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub enum Disp {
  None,
  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
}

#[inline(always)]
fn to_str<'a>(x: &'a [u8]) -> &'a str {
  unsafe{ str::from_utf8_unchecked(x) }
}
#[inline(always)]
fn parse_decimal<'a>(i: &'a [u8]) -> IResult<&'a [u8], Disp> {
  if i.len() == 0 {
    return IResult::Incomplete(Needed::Size(1));
  }
  let mut index = 0;
  while is_digit(i[index]) || ((index == 0) && (&i[0..1] == b"-")) {
    index += 1;
    if i.len() == index {
      return IResult::Error(ErrorKind::Eof);
    }
  }
  let s = to_str(&i[0..index]);
  match i8::from_str_radix(s,10) {
    Ok(x) => IResult::Done(&i[index..],Disp::I8(x)),
    Err(_) => match i16::from_str_radix(s,10) {
      Ok(x) => IResult::Done(&i[index..],Disp::I16(x)),
      Err(_) => match i32::from_str_radix(s,10) {
        Ok(x) => IResult::Done(&i[index..],Disp::I32(x)),
        Err(_) => match i64::from_str_radix(s,10) {
          Ok(x) => IResult::Done(&i[index..],Disp::I64(x)),
          Err(_) => IResult::Error(ErrorKind::Tag)
        }
      }
    }
  }
}
impl Default for Disp {
  #[inline(always)]
  fn default() -> Disp {
    Disp::None
  }
}
impl Disp {
  
  /// Get the size of a displacement
  #[inline]
  pub fn size_of(&self) -> Option<RegSize> {
    match self {
      &Disp::None => None,
      &Disp::I8(_) => Some(RegSize::Bit8),
      &Disp::I16(_) => Some(RegSize::Bit16),
      &Disp::I32(_) => Some(RegSize::Bit32),
      &Disp::I64(_) => Some(RegSize::Bit64)
    }
  }

  /// Get the size in bytes of the displacement
  #[inline]
  pub fn byte_size(&self) -> usize {
    match self.size_of() {
      Option::Some(reg) => reg.byte_size(),
      _ => 0
    }
  }

  /// Attempts to convert the type into NONE
  #[inline]
  pub fn into_none(&self) -> Option<Disp> {
    match self {
      &Disp::None |
      &Disp::I8(0i8) |
      &Disp::I16(0i16) |
      &Disp::I32(0i32) |
      &Disp::I64(0i64) => Some(Disp::None),
      _ => None
    }
  }

  /// Can this value be converted into an i8 value
  #[inline]
  pub fn into_i8(&self) -> Option<Disp> {
    match self {
      &Disp::None => Some(Disp::I8(0i8)), 
      &Disp::I8(_) => Some(self.clone()),
      &Disp::I16(val) => {
        let a = val as i8;
        let b = a as i16;
        if b == val {
          Some(Disp::I8(a))
        } else {
          None
        }
      },
      &Disp::I32(val) => {
        let a = val as i8;
        let b = a as i32;
        if b == val {
          Some(Disp::I8(a))
        } else {
          None
        }
      },
      &Disp::I64(val) => {
        let a = val as i8;
        let b = a as i64;
        if b == val {
          Some(Disp::I8(a))
        } else {
          None
        }
      }
    }
  }
  

  /// Attempts to override `Disp::_` into `Disp::i16`
  #[inline]
  pub fn into_i16(&self) -> Option<Disp> {
    match self {
      &Disp::None => Some(Disp::I16(0i16)), 
      &Disp::I8(ref val) => Some(Disp::I16(val.clone() as i16)),
      &Disp::I16(_) => Some(self.clone()),
      &Disp::I32(val) => {
        let a = val as i16;
        let b = a as i32;
        if b == val {
          Some(Disp::I16(a))
        } else {
          None
        }
      },
      &Disp::I64(val) => {
        let a = val as i16;
        let b = a as i64;
        if b == val {
          Some(Disp::I16(a))
        } else {
          None
        }
      },
    }
  }
  
  /// Attempts to override `Disp::_` into `Disp::i32`
  #[inline]
  pub fn into_i32(&self) -> Option<Disp> {
    match self {
      &Disp::None => Some(Disp::I32(0i32)), 
      &Disp::I8(ref val) => Some(Disp::I32(val.clone() as i32)),
      &Disp::I16(ref val) => Some(Disp::I32(val.clone() as i32)),
      &Disp::I32(_) => Some(self.clone()),
      &Disp::I64(val) => {
        let a = val as i32;
        let b = a as i64;
        if b == val {
          Some(Disp::I32(a))
        } else {
          None
        }
      },
    }
  }

  /// Attempts to override `Disp::_` into `Disp::i32`
  #[inline]
  pub fn into_i64(&self) -> Option<Disp> {
    match self {
      &Disp::None => Some(Disp::I64(0i64)), 
      &Disp::I8(ref val) => Some(Disp::I64(val.clone() as i64)),
      &Disp::I16(ref val) => Some(Disp::I64(val.clone() as i64)),
      &Disp::I32(ref val) => Some(Disp::I64(val.clone() as i64)),
      &Disp::I64(_) => Some(self.clone()),
    }
  }

  /// Attempt to reduce to the next smallest item
  #[inline]
  pub fn reduce(&self) -> Option<Disp> {
    match self {
      &Disp::None => None,
      &Disp::I8(_) => self.into_none(),
      &Disp::I16(_) => self.into_i8(),
      &Disp::I32(_) => self.into_i16(),
      &Disp::I64(_) => self.into_i32()
    }
  }

  /// Reduce something to it's minimum size
  pub fn strength_reduce(self) -> Disp {
    let mut x = self;
    loop {
      let y = x.reduce();
      match y {
        Option::None => return x,
        Option::Some(z) => x = z
      }
    }
  }

  pub fn to_slice<'a>(&'a self) -> &'a [u8] {
    match self {
      &Disp::None => b"",
      &Disp::I8(ref val) => {
        let mut tup: (&i8, usize) = (val,1);
        unsafe{ mem::transmute(tup) }
      },
      &Disp::I16(ref val) => {
        let mut tup: (&i16, usize) = (val,2);
        unsafe{ mem::transmute(tup) }
      },
      &Disp::I32(ref val) => {
        let mut tup: (&i32, usize) = (val,4);
        unsafe{ mem::transmute(tup) }
      },
      &Disp::I64(ref val) => {
        let mut tup: (&i64, usize) = (val,8);
        unsafe{ mem::transmute(tup) }
      },
    }
  }
}

named!(pub parse_const<Disp>, do_parse!(
  val: ws!(parse_decimal) >> 
  (val.strength_reduce())
));


#[test]
fn test_size_of_disp() {
  assert_eq!(Disp::None.size_of(), Option::None);
  assert_eq!(Disp::I8(1i8).size_of(), Option::Some(RegSize::Bit8));
  assert_eq!(Disp::I16(2i16).size_of(), Option::Some(RegSize::Bit16));
  assert_eq!(Disp::I32(15i32).size_of(), Option::Some(RegSize::Bit32));
}

#[test]
fn test_byte_size_disp() {
  assert_eq!(Disp::None.byte_size(),0);
  assert_eq!(Disp::I8(1i8).byte_size(),1);
  assert_eq!(Disp::I16(2i16).byte_size(),2);
  assert_eq!(Disp::I32(15i32).byte_size(),4);
}

#[test]
fn test_default_disp() {
  let d = Disp::default();
  assert_eq!(d, Disp::None);
}

#[test]
fn test_into_none() {
  assert_eq!(Disp::None.into_none(), Option::Some(Disp::None));
  
  assert_eq!(Disp::I8(1i8).into_none(), Option::None);
  assert_eq!(Disp::I8(0i8).into_none(), Option::Some(Disp::None));
  
  assert_eq!(Disp::I16(1i16).into_none(), Option::None);
  assert_eq!(Disp::I16(0i16).into_none(), Option::Some(Disp::None));
  
  assert_eq!(Disp::I32(1i32).into_none(), Option::None);
  assert_eq!(Disp::I32(0i32).into_none(), Option::Some(Disp::None));
  
}


/*
 * This test actually validates the other conversion methods
 */
#[test]
fn test_into_i8() {

  use std::i8;
  use std::i16;
  use std::i32;

  assert_eq!(Disp::None.into_i8(), Option::Some(Disp::I8(0i8)));
  
  assert_eq!(Disp::I8(1i8).into_i8(), Option::Some(Disp::I8(1i8)));
  assert_eq!(Disp::I8(0i8).into_i8(), Option::Some(Disp::I8(0i8)));
  assert_eq!(Disp::I8(-15i8).into_i8(), Option::Some(Disp::I8(-15i8)));
  assert_eq!(Disp::I8(i8::MIN).into_i8(), Option::Some(Disp::I8(i8::MIN)));
  assert_eq!(Disp::I8(i8::MAX).into_i8(), Option::Some(Disp::I8(i8::MAX)));
  
  assert_eq!(Disp::I16(1i16).into_i8(), Option::Some(Disp::I8(1i8)));
  assert_eq!(Disp::I16(0i16).into_i8(), Option::Some(Disp::I8(0i8)));
  assert_eq!(Disp::I16(-15i16).into_i8(), Option::Some(Disp::I8(-15i8)));
  assert_eq!(Disp::I16(i16::MIN).into_i8(), None);
  assert_eq!(Disp::I16(i16::MAX).into_i8(), None);

  assert_eq!(Disp::I32(1i32).into_i8(), Option::Some(Disp::I8(1i8)));
  assert_eq!(Disp::I32(0i32).into_i8(), Option::Some(Disp::I8(0i8)));
  assert_eq!(Disp::I32(-112i32).into_i8(), Option::Some(Disp::I8(-112i8)));
  assert_eq!(Disp::I32(i32::MIN).into_i8(), None);
  assert_eq!(Disp::I32(i32::MAX).into_i8(), None);
  assert_eq!(Disp::I32(127i32).into_i8(), Option::Some(Disp::I8(127i8)));
  assert_eq!(Disp::I32(128i32).into_i8(), Option::None);
  assert_eq!(Disp::I32(-128i32).into_i8(), Option::Some(Disp::I8(-128i8)));
  assert_eq!(Disp::I32(-129i32).into_i8(), Option::None);
}


/*
 * Testing that parsing works as expected
 *
 */
#[test]
fn test_disp_parse() {

  let dut = b" 0 ";
  let (_,val) = parse_const(dut).unwrap();
  assert_eq!(val, Disp::None);

  let dut = b" -1 ";
  let (_,val) = parse_const(dut).unwrap();
  assert_eq!(val, Disp::I8(-1i8));

  let dut = b" 128 ";
  let (_,val) = parse_const(dut).unwrap();
  assert_eq!(val, Disp::I16(128i16));
}



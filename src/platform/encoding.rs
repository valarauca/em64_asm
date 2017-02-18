
use std::ops::BitAnd;

///Describes the encoding of a register
///
/// Bit masks can be AND'd together to form the the output mask
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u32)]
pub enum Encode {

  /// This is the error condition
  NONE    = 0x00000000u32,

  /// X86 requires. Using a REX prefix causes an error
  X86_R   = 0x00000001u32,

  /// REX required
	REX     = 0x00000010u32,

  /// This can only be encoded with VEX
  VEX_R   = 0x00000100u32,

  /// This can only be encoded in EVEX
  EVEX    = 0x00001000u32,

  /// XOP is a real wild card
	XOP     = 0x00010000u32,

  /// This can be encoded in X86_R or REX mode
  X86     = 0x00000011u32,

  /// This can be encoded X86, REX, VEX, or EVEX
  VEX     = 0x00001111u32,

  /// This can be encoded in REX, VEX, or EVEX
  VEX_REX = 0x00001110u32,

  /// Everything, special value used for seeding
  EVERYTHING = 0x11111111u32

}
impl Into<u32> for Encode {

  /// Cover the enum into a value
  #[inline(always)]
  fn into(self) -> u32 {
    use std::mem;
    unsafe{ mem::transmute(self) }
  }
}
impl BitAnd<u32> for Encode {
  type Output = bool;

  /// this is used internally for checking if a bit mask matches
  #[inline(always)]
  fn bitand(self, other: u32) -> Self::Output {
    let x: u32 = self.into();
    (x & other) > 0
  }
}
impl From<u32> for Encode {

  /// apply the bit masking again used internally
  #[inline(always)]
  fn from(x: u32) -> Encode {
    if Encode::X86_R & x {
      return Encode::X86_R;
    }
    if Encode::REX & x {
      return Encode::REX;
    }
    if Encode::VEX_R & x{
      return Encode::VEX_R;
    }
    if Encode::XOP & x {
      return Encode::XOP;
    }
    if Encode::EVEX & x {
      return Encode::EVEX;
    }
    return Encode::NONE;
  }
}
impl BitAnd<Encode> for Encode {
  type Output = Encode;
  
  /// The actual AND between Encode types
  #[inline]
  fn bitand(self, other: Encode) -> Self::Output {
    let s: u32 = self.into();
    let o: u32 = other.into();
    Encode::from(s&o)
  }
}

macro_rules! tester {
  (
    $a: expr, $b: expr, $ret: expr
  ) => {
        let dut: Encode = $a&$b;
        if dut != $ret {
          panic!("{:?} and {:?} = {:?} but observed {:?}", $a, $b, $ret, dut);
        }
  }
}


#[test]
fn test_encode() {
  tester!(Encode::X86,Encode::VEX, Encode::X86_R);
  tester!(Encode::X86,Encode::REX, Encode::REX);
  tester!(Encode::X86,Encode::X86_R, Encode::X86_R);
  tester!(Encode::X86_R, Encode::REX, Encode::NONE);
}

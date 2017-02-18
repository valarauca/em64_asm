//! Archs work via a dynamic trait system.
//! 
//! Individual Opcodes implement multiple 

use super::{
  Ext,
  FeatureSet
};



/// This trait is only assigned to platforms
pub trait Arch { 

  fn extensions() -> FeatureSet<'static>;
}

macro_rules! define_plat {
  ($kind: ident; $plat: ident => $($name: ident),*) => {
    pub struct $kind;
    const $plat: FeatureSet<'static> = &[$(Ext::$name),*];
    impl Arch for $kind {
      #[inline(always)]
      fn extensions() -> FeatureSet<'static> {
        $plat
      }
    }
  }
}

define_plat! {
  Generic; generic_ext => X86, X64, X87, MMX, SSE, SSE2
}
define_plat!{
  Haswell; haswell_ext =>
    X86, X64, X87, CMOV, CLFSH, MMX, SSE, SSE2,
    SSE3, SSSE3, SSE41, SSE42, POPCNT, AES, AVX, AVX2,
    PCLMULQDQ, RDRAND, FMA3, BMI1, BMI2, F16C, HLE, SHA
}
define_plat!{
  Boardwell; boardwell_ext =>
    X86, X64, X87, CMOV, CLFSH, MMX, SSE, SSE2,
    SSE3, SSSE3, SSE41, SSE42, POPCNT, AES, AVX, AVX2,
    PCLMULQDQ, RDRAND, FMA3, BMI1, BMI2, F16C, HLE, RTM,
    ADX, SHA, RDSEED, CLFLUSHOPT, MPX
}





#[allow(unused_imports)]
use super::{
  RegSize
};
use super::super::super::platform::{
  Encode,
  Ext
};


macro_rules! gen_reg {
  ($($name: ident => $val: expr => $size: ident => $ext: ident => $encode: ident => $repr: expr),*) => {
    #[repr(u16)]
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum Register {
      $($name),*
    }
    impl Register {

      #[inline(always)]
      pub fn is_ip(&self) -> bool {
        match self.clone() {
          Register::rip |
          Register::eip => true,
          _ => false
        }
      }

      #[inline(never)]
      pub fn requires(&self) -> Ext {
        match self.clone() {
          $(Register::$name => Ext::$ext),*
        }
      }

      #[inline(never)]
      pub fn name(&self) -> &'static str {
        match self.clone() {
          $(Register::$name => $val),*
        }
      }

      #[inline(never)]
      pub fn size(&self) -> RegSize {
        match self.clone() {
          $(Register::$name => RegSize::$size),*
        }
      }

      #[inline(never)]
      pub fn encoding(&self) -> Encode {
        match self.clone() {
          $(Register::$name => Encode::$encode),*
        }
      }
      #[inline(never)]
      pub fn repr(&self) -> u8 {
        match self.clone() {
          $(Register::$name => $repr),*
        }
      }

      #[inline(always)]
      pub fn is_bit8(&self) -> bool {
        match self.size() {
          RegSize::Bit8 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit16(&self) -> bool {
        match self.size() {
          RegSize::Bit16 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit32(&self) -> bool {
        match self.size() {
          RegSize::Bit32 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit64(&self) -> bool {
        match self.size() {
          RegSize::Bit64 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit128(&self) -> bool {
        match self.size() {
          RegSize::Bit128 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit256(&self) -> bool {
        match self.size() {
          RegSize::Bit256 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_bit512(&self) -> bool {
        match self.size() {
          RegSize::Bit512 => true,
          _ => false
        }
      }
      #[inline(always)]
      pub fn is_vector(&self) -> bool {
        match self.size() {
          RegSize::Bit512 |
          RegSize::Bit256 |
          RegSize::Bit128 => true,
          _ => false
        }
      }
    }
    
    /*
    named!(pub parse_reg<Register>, alt!(
        $(complete!(do_parse!(tag!($val) >> (Register::$name))))|*
    ));
    */
    #[test]
    fn test_registers() {
      use super::super::super::nom::IResult;
        $(
          let val = Register::$name;
          let dut = $val;
          match parse_reg(dut.as_bytes()) {
            IResult::Done(_,x) => {
              if val != x {
                panic!("Expected {:?}, observed {:?}", val, x);
              }
            },      
            IResult::Error(e) => panic!("Error parsing: {:?}. Error: {:?}", $val, e),
            IResult::Incomplete(n) => panic!("Error parsing: {:?}. Error: {:?}", $val, n)
          };
        )*
    }
  }
}

gen_reg!{
 zmm31 => "zmm31" => Bit512 => AVX512_F => EVEX => 0, 
 zmm30 => "zmm30" => Bit512 => AVX512_F => EVEX => 0, 
 zmm29 => "zmm29" => Bit512 => AVX512_F => EVEX => 0, 
 zmm28 => "zmm28" => Bit512 => AVX512_F => EVEX => 0, 
 zmm27 => "zmm27" => Bit512 => AVX512_F => EVEX => 0, 
 zmm26 => "zmm26" => Bit512 => AVX512_F => EVEX => 0, 
 zmm25 => "zmm25" => Bit512 => AVX512_F => EVEX => 0, 
 zmm24 => "zmm24" => Bit512 => AVX512_F => EVEX => 0, 
 zmm23 => "zmm23" => Bit512 => AVX512_F => EVEX => 0, 
 zmm22 => "zmm22" => Bit512 => AVX512_F => EVEX => 0, 
 zmm21 => "zmm21" => Bit512 => AVX512_F => EVEX => 0, 
 zmm20 => "zmm20" => Bit512 => AVX512_F => EVEX => 0, 
 zmm19 => "zmm19" => Bit512 => AVX512_F => EVEX => 0, 
 zmm18 => "zmm18" => Bit512 => AVX512_F => EVEX => 0, 
 zmm17 => "zmm17" => Bit512 => AVX512_F => EVEX => 0, 
 zmm16 => "zmm16" => Bit512 => AVX512_F => EVEX => 0, 
 zmm15 => "zmm15" => Bit512 => AVX512_F => EVEX => 0, 
 zmm14 => "zmm14" => Bit512 => AVX512_F => EVEX => 0, 
 zmm13 => "zmm13" => Bit512 => AVX512_F => EVEX => 0, 
 zmm12 => "zmm12" => Bit512 => AVX512_F => EVEX => 0, 
 zmm11 => "zmm11" => Bit512 => AVX512_F => EVEX => 0, 
 zmm10 => "zmm10" => Bit512 => AVX512_F => EVEX => 0, 
 ymm31 => "ymm31" => Bit256 => AVX512_F => EVEX => 0, 
 ymm30 => "ymm30" => Bit256 => AVX512_F => EVEX => 0, 
 ymm29 => "ymm29" => Bit256 => AVX512_F => EVEX => 0, 
 ymm28 => "ymm28" => Bit256 => AVX512_F => EVEX => 0, 
 ymm27 => "ymm27" => Bit256 => AVX512_F => EVEX => 0, 
 ymm26 => "ymm26" => Bit256 => AVX512_F => EVEX => 0, 
 ymm25 => "ymm25" => Bit256 => AVX512_F => EVEX => 0, 
 ymm24 => "ymm24" => Bit256 => AVX512_F => EVEX => 0, 
 ymm23 => "ymm23" => Bit256 => AVX512_F => EVEX => 0, 
 ymm22 => "ymm22" => Bit256 => AVX512_F => EVEX => 0, 
 ymm21 => "ymm21" => Bit256 => AVX512_F => EVEX => 0, 
 ymm20 => "ymm20" => Bit256 => AVX512_F => EVEX => 0, 
 ymm19 => "ymm19" => Bit256 => AVX512_F => EVEX => 0, 
 ymm18 => "ymm18" => Bit256 => AVX512_F => EVEX => 0, 
 ymm17 => "ymm17" => Bit256 => AVX512_F => EVEX => 0, 
 ymm16 => "ymm16" => Bit256 => AVX512_F => EVEX => 0, 
 ymm15 => "ymm15" => Bit256 => AVX2 => VEX_REX => 15, 
 ymm14 => "ymm14" => Bit256 => AVX2 => VEX_REX => 14, 
 ymm13 => "ymm13" => Bit256 => AVX2 => VEX_REX => 13, 
 ymm12 => "ymm12" => Bit256 => AVX2 => VEX_REX => 12, 
 ymm11 => "ymm11" => Bit256 => AVX2 => VEX_REX => 11, 
 ymm10 => "ymm10" => Bit256 => AVX2 => VEX_REX => 10, 
 xmm31 => "xmm31" => Bit256 => AVX512_F => EVEX => 31, 
 xmm30 => "xmm30" => Bit256 => AVX512_F => EVEX => 30, 
 xmm29 => "xmm29" => Bit256 => AVX512_F => EVEX => 29, 
 xmm28 => "xmm28" => Bit256 => AVX512_F => EVEX => 28, 
 xmm27 => "xmm27" => Bit256 => AVX512_F => EVEX => 27, 
 xmm26 => "xmm26" => Bit256 => AVX512_F => EVEX => 26, 
 xmm25 => "xmm25" => Bit256 => AVX512_F => EVEX => 25, 
 xmm24 => "xmm24" => Bit256 => AVX512_F => EVEX => 24, 
 xmm23 => "xmm23" => Bit256 => AVX512_F => EVEX => 23, 
 xmm22 => "xmm22" => Bit256 => AVX512_F => EVEX => 22, 
 xmm21 => "xmm21" => Bit256 => AVX512_F => EVEX => 21, 
 xmm20 => "xmm20" => Bit256 => AVX512_F => EVEX => 20, 
 xmm19 => "xmm19" => Bit256 => AVX512_F => EVEX => 19, 
 xmm18 => "xmm18" => Bit256 => AVX512_F => EVEX => 18, 
 xmm17 => "xmm17" => Bit256 => AVX512_F => EVEX => 17, 
 xmm16 => "xmm16" => Bit256 => AVX512_F => EVEX => 16, 
 xmm15 => "xmm15" => Bit256 => SSE => VEX_REX => 15, 
 xmm14 => "xmm14" => Bit256 => SSE => VEX_REX => 14, 
 xmm13 => "xmm13" => Bit256 => SSE => VEX_REX => 13, 
 xmm12 => "xmm12" => Bit256 => SSE => VEX_REX => 12, 
 xmm11 => "xmm11" => Bit256 => SSE => VEX_REX => 11, 
 xmm10 => "xmm10" => Bit256 => SSE => VEX_REX => 10, 
 zmm9 => "zmm9" => Bit512 => AVX512_F => EVEX => 9, 
 zmm8 => "zmm8" => Bit512 => AVX512_F => EVEX => 8, 
 zmm7 => "zmm7" => Bit512 => AVX512_F => EVEX => 7, 
 zmm6 => "zmm6" => Bit512 => AVX512_F => EVEX => 6, 
 zmm5 => "zmm5" => Bit512 => AVX512_F => EVEX => 5, 
 zmm4 => "zmm4" => Bit512 => AVX512_F => EVEX => 4, 
 zmm3 => "zmm3" => Bit512 => AVX512_F => EVEX => 3, 
 zmm2 => "zmm2" => Bit512 => AVX512_F => EVEX => 2, 
 zmm1 => "zmm1" => Bit512 => AVX512_F => EVEX => 1, 
 zmm0 => "zmm0" => Bit512 => AVX512_F => EVEX => 0, 
 ymm9 => "ymm9" => Bit256 => AVX2 => VEX_REX => 9, 
 ymm8 => "ymm8" => Bit256 => AVX2 => VEX_REX => 8, 
 ymm7 => "ymm7" => Bit256 => AVX2 => VEX => 7, 
 ymm6 => "ymm6" => Bit256 => AVX2 => VEX => 6, 
 ymm5 => "ymm5" => Bit256 => AVX2 => VEX => 5, 
 ymm4 => "ymm4" => Bit256 => AVX2 => VEX => 4, 
 ymm3 => "ymm3" => Bit256 => AVX2 => VEX => 3, 
 ymm2 => "ymm2" => Bit256 => AVX2 => VEX => 2, 
 ymm1 => "ymm1" => Bit256 => AVX2 => VEX => 1, 
 ymm0 => "ymm0" => Bit256 => AVX2 => VEX => 0, 
 xmm9 => "xmm9" => Bit256 => SSE => VEX_REX => 9, 
 xmm8 => "xmm8" => Bit256 => SSE => VEX_REX => 8, 
 xmm7 => "xmm7" => Bit256 => SSE => REX => 7, 
 xmm6 => "xmm6" => Bit256 => SSE => REX => 6, 
 xmm5 => "xmm5" => Bit256 => SSE => REX => 5, 
 xmm4 => "xmm4" => Bit256 => SSE => REX => 4, 
 xmm3 => "xmm3" => Bit256 => SSE => REX => 3, 
 xmm2 => "xmm2" => Bit256 => SSE => REX => 2, 
 xmm1 => "xmm1" => Bit256 => SSE => REX => 1, 
 xmm0 => "xmm0" => Bit256 => SSE => REX => 0,
 r8l => "r8l" => Bit8 => X64 => REX => 8,
 r8w => "r8w" => Bit16 => X64 => REX => 8,
 r8d => "r8d" => Bit32 => X64 => REX => 8,
 r9l => "r9l" => Bit8 => X64 => REX => 9,
 r9w => "r9w" => Bit16 => X64 => REX => 9,
 r9d => "r9d" => Bit32 => X64 => REX => 9,
 r10l => "r10l" => Bit8 => X64 => REX => 10,
 r10w => "r10w" => Bit16 => X64 => REX => 10,
 r10d => "r10d" => Bit32 => X64 => REX => 10,
 r11l => "r11l" => Bit8 => X64 => REX => 11,
 r11w => "r11w" => Bit16 => X64 => REX => 11,
 r11d => "r11d" => Bit32 => X64 => REX => 11,
 r12l => "r12l" => Bit8 => X64 => REX => 12,
 r12w => "r12w" => Bit16 => X64 => REX => 12,
 r12d => "r12d" => Bit32 => X64 => REX => 12,
 r13l => "r13l" => Bit8 => X64 => REX => 13,
 r13w => "r13w" => Bit16 => X64 => REX => 13,
 r13d => "r13d" => Bit32 => X64 => REX => 13,
 r14l => "r14l" => Bit8 => X64 => REX => 14,
 r14w => "r14w" => Bit16 => X64 => REX => 14,
 r14d => "r14d" => Bit32 => X64 => REX => 14,
 r15l => "r15l" => Bit8 => X64 => REX => 15,
 r15w => "r15w" => Bit16 => X64 => REX => 15,
 r15d => "r15d" => Bit32 => X64 => REX => 15,
 r8 => "r8" => Bit64 => X64 => REX => 8,
 r9 => "r9" => Bit64 => X64 => REX => 9,
 rip => "rip" => Bit64 => X86 => REX => 0xFF,
 r10 => "r10" => Bit64 => X64 => REX => 10,
 r11 => "r11" => Bit64 => X64 => REX => 11,
 r12 => "r12" => Bit64 => X64 => REX => 12,
 r13 => "r13" => Bit64 => X64 => REX => 13,
 r14 => "r14" => Bit64 => X64 => REX => 14,
 r15 => "r15" => Bit64 => X64 => REX => 15,
 rax => "rax" => Bit64 => X64 => REX => 0,
 rcx => "rcx" => Bit64 => X64 => REX => 1,
 rdx => "rdx" => Bit64 => X64 => REX => 2,
 rbx => "rbx" => Bit64 => X64 => REX => 3,
 rsp => "rsp" => Bit64 => X64 => REX => 4,
 rbp => "rbp" => Bit64 => X64 => REX => 5,
 rsi => "rsi" => Bit64 => X64 => REX => 6,
 rdi => "rdi" => Bit64 => X64 => REX => 7,
 eip => "eip" => Bit32 => X86 => X86 => 0xFF,
 eax => "eax" => Bit32 => X86 => X86 => 0,
 ecx => "ecx" => Bit32 => X86 => X86 => 1,
 edx => "edx" => Bit32 => X86 => X86 => 2,
 ebx => "ebx" => Bit32 => X86 => X86 => 3,
 esp => "esp" => Bit32 => X86 => X86 => 4,   
 ebp => "ebp" => Bit32 => X86 => X86 => 5,
 esi => "esi" => Bit32 => X86 => X86 => 6,
 edi => "edi" => Bit32 => X86 => X86 => 7,
 bpl => "bpl" => Bit8 => X86 => REX => 5,
 dil => "dil" => Bit8 => X86 => REX => 7,
 sil => "sil" => Bit8 => X86 => REX => 6,
 spl => "spl" => Bit8 => X86 => REX => 4,
 al => "al" => Bit8 => X86 => X86 => 0,
 cl => "cl" => Bit8 => X86 => X86 => 1,
 dl => "dl" => Bit8 => X86 => X86 => 2,
 bl => "bl" => Bit8 => X86 => X86 => 3,
 ah => "ah" => Bit8 => X86 => X86_R => 4,
 ch => "ch" => Bit8 => X86 => X86_R => 5,
 dh => "dh" => Bit8 => X86 => X86_R => 6,
 bh => "bh" => Bit8 => X86 => X86_R => 7,
 ax => "ax" => Bit16 => X86 => X86 => 0,
 dx => "dx" => Bit16 => X86 => X86 => 1,
 cx => "cx" => Bit16 => X86 => X86 => 2,
 bx => "bx" => Bit16 => X86 => X86 => 3,
 bp => "bp" => Bit16 => X86 => X86 => 5,
 di => "di" => Bit16 => X86 => X86 => 7,
 sp => "sp" => Bit16 => X86 => X86 => 4,
 si => "si" => Bit16 => X86 => X86 => 6,
 st0 => "st0" => Bit80 => X87 => X86 => 0, 
 st1 => "st1" => Bit80 => X87 => X86 => 1,
 st2 => "st2" => Bit80 => X87 => X86 => 2,
 st3 => "st3" => Bit80 => X87 => X86 => 3, 
 st4 => "st4" => Bit80 => X87 => X86 => 4,
 st5 => "st5" => Bit80 => X87 => X86 => 5,
 st6 => "st6" => Bit80 => X87 => X86 => 6, 
 st7 => "st7" => Bit80 => X87 => X86 => 7,
 mmx0 => "mmx0" => BitMMX => MMX => X86 => 0,
 mmx1 => "mmx1" => BitMMX => MMX => X86 => 1,
 mmx2 => "mmx2" => BitMMX => MMX => X86 => 2,
 mmx3 => "mmx3" => BitMMX => MMX => X86 => 3,
 mmx4 => "mmx4" => BitMMX => MMX => X86 => 4,
 mmx5 => "mmx5" => BitMMX => MMX => X86 => 5,
 mmx6 => "mmx6" => BitMMX => MMX => X86 => 6,
 mmx7 => "mmx7" => BitMMX => MMX => X86 => 7
}

macro_rules! generate_parser {
  (@PREFIXED
    Name: $name: ident;
    Prefix: $prefix: expr;
    Val: { $($arg: expr => $label: ident),* };
  ) => {
    named!(pub $name<Register>, do_parse!( 
      peek!(tag!($prefix)) >>
      val: alt!(
        $(complete!(do_parse!(tag!($arg) >> (Register::$label))))|*
      ) >>
      (val)
    ));
  };
  (
    Name: $name: ident;
    Val: { $($arg: expr => $label: ident),* };
  ) => {
    named!(pub $name<Register>, alt!(
      $(complete!(do_parse!(tag!($arg) >> (Register::$label))))|*
    ));
  };
}

generate_parser!{@PREFIXED
  Name: parse_128bit_reg;
  Prefix: b"xmm";
  Val: {
    b"xmm31" => xmm31,
    b"xmm30" => xmm30,
    b"xmm29" => xmm29,
    b"xmm28" => xmm28,
    b"xmm27" => xmm27,
    b"xmm26" => xmm26,
    b"xmm25" => xmm25,
    b"xmm24" => xmm24,
    b"xmm23" => xmm23,
    b"xmm22" => xmm22,
    b"xmm21" => xmm21,
    b"xmm20" => xmm20,
    b"xmm19" => xmm19,
    b"xmm18" => xmm18,
    b"xmm17" => xmm17,
    b"xmm16" => xmm16,
    b"xmm15" => xmm15,
    b"xmm14" => xmm14,
    b"xmm13" => xmm13,
    b"xmm12" => xmm12,
    b"xmm11" => xmm11,
    b"xmm10" => xmm10,
    b"xmm9" => xmm9,
    b"xmm8" => xmm8,
    b"xmm7" => xmm7,
    b"xmm6" => xmm6,
    b"xmm5" => xmm5,
    b"xmm4" => xmm4,
    b"xmm3" => xmm3,
    b"xmm2" => xmm2,
    b"xmm1" => xmm1,
    b"xmm0" => xmm0
  };
}
generate_parser!{@PREFIXED
  Name: parse_256bit_reg;
  Prefix: b"ymm";
  Val: {
    b"ymm31" => ymm31,
    b"ymm30" => ymm30,
    b"ymm29" => ymm29,
    b"ymm28" => ymm28,
    b"ymm27" => ymm27,
    b"ymm26" => ymm26,
    b"ymm25" => ymm25,
    b"ymm24" => ymm24,
    b"ymm23" => ymm23,
    b"ymm22" => ymm22,
    b"ymm21" => ymm21,
    b"ymm20" => ymm20,
    b"ymm19" => ymm19,
    b"ymm18" => ymm18,
    b"ymm17" => ymm17,
    b"ymm16" => ymm16,
    b"ymm15" => ymm15,
    b"ymm14" => ymm14,
    b"ymm13" => ymm13,
    b"ymm12" => ymm12,
    b"ymm11" => ymm11,
    b"ymm10" => ymm10,
    b"ymm9" => ymm9,
    b"ymm8" => ymm8,
    b"ymm7" => ymm7,
    b"ymm6" => ymm6,
    b"ymm5" => ymm5,
    b"ymm4" => ymm4,
    b"ymm3" => ymm3,
    b"ymm2" => ymm2,
    b"ymm1" => ymm1,
    b"ymm0" => ymm0
  };
}
generate_parser!{@PREFIXED
  Name: parse_512bit_reg;
  Prefix: b"zmm";
  Val: {
    b"zmm31" => zmm31,
    b"zmm30" => zmm30,
    b"zmm29" => zmm29,
    b"zmm28" => zmm28,
    b"zmm27" => zmm27,
    b"zmm26" => zmm26,
    b"zmm25" => zmm25,
    b"zmm24" => zmm24,
    b"zmm23" => zmm23,
    b"zmm22" => zmm22,
    b"zmm21" => zmm21,
    b"zmm20" => zmm20,
    b"zmm19" => zmm19,
    b"zmm18" => zmm18,
    b"zmm17" => zmm17,
    b"zmm16" => zmm16,
    b"zmm15" => zmm15,
    b"zmm14" => zmm14,
    b"zmm13" => zmm13,
    b"zmm12" => zmm12,
    b"zmm11" => zmm11,
    b"zmm10" => zmm10,
    b"zmm9" => zmm9,
    b"zmm8" => zmm8,
    b"zmm7" => zmm7,
    b"zmm6" => zmm6,
    b"zmm5" => zmm5,
    b"zmm4" => zmm4,
    b"zmm3" => zmm3,
    b"zmm2" => zmm2,
    b"zmm1" => zmm1,
    b"zmm0" => zmm0
  };
}
generate_parser!{@PREFIXED
  Name: parse_mmx_reg;
  Prefix: b"mmx";
  Val: {
    b"mmx0" => mmx0,
    b"mmx1" => mmx1,
    b"mmx2" => mmx2,
    b"mmx3" => mmx3,
    b"mmx4" => mmx4,
    b"mmx5" => mmx5,
    b"mmx6" => mmx6,
    b"mmx7" => mmx7
  };
}
generate_parser!{@PREFIXED
  Name: parse_x87_reg;
  Prefix: b"st";
  Val: {
    b"st0" => st0,
    b"st1" => st1,
    b"st2" => st2,
    b"st3" => st3,
    b"st4" => st4,
    b"st5" => st5,
    b"st6" => st6,
    b"st7" => st7
  };
}
generate_parser!{
  Name: parse_8bit_reg;
  Val: {
    b"r10l" => r10l,
    b"r11l" => r11l,
    b"r12l" => r12l,
    b"r13l" => r13l,
    b"r14l" => r14l,
    b"r15l" => r15l,
    b"bpl" => bpl,
    b"dil" => dil,
    b"sil" => sil,
    b"spl" => spl,
    b"r8l" => r8l,
    b"r9l" => r9l,
    b"al" => al,
    b"cl" => cl,
    b"dl" => dl,
    b"bl" => bl,
    b"ah" => ah,
    b"ch" => ch,
    b"dh" => dh,
    b"bh" => bh
  };
}
generate_parser!{
  Name: parse_16bit_reg;
  Val: {
    b"r10w" => r10w,
    b"r11w" => r11w,
    b"r12w" => r12w,
    b"r13w" => r13w,
    b"r14w" => r14w,
    b"r15w" => r15w,
    b"r8w" => r8w,
    b"r9w" => r9w,
    b"ax" => ax,
    b"dx" => dx,
    b"cx" => cx,
    b"bx" => bx,
    b"bp" => bp,
    b"di" => di,
    b"sp" => sp,
    b"si" => si
  };
}
generate_parser!{
  Name: parse_32bit_reg;
  Val: {
    b"r10d" => r10d,
    b"r11d" => r11d,
    b"r12d" => r12d,
    b"r13d" => r13d,
    b"r14d" => r14d,
    b"r15d" => r15d,
    b"r8d" => r8d,
    b"r9d" => r9d,
    b"eax" => eax,
    b"edx" => edx,
    b"ecx" => ecx,
    b"ebx" => ebx,
    b"ebp" => ebp,
    b"edi" => edi,
    b"esp" => esp,
    b"esi" => esi,
    b"eip" => eip
  };
}
generate_parser!{
  Name: parse_64bit_reg;
  Val: {
    b"r10" => r10,
    b"r11" => r11,
    b"r12" => r12,
    b"r13" => r13,
    b"r14" => r14,
    b"r15" => r15,
    b"r8" => r8,
    b"r9" => r9,
    b"rax" => rax,
    b"rdx" => rdx,
    b"rcx" => rcx,
    b"rbx" => rbx,
    b"rbp" => rbp,
    b"rdi" => rdi,
    b"rsp" => rsp,
    b"rsi" => rsi,
    b"rip" => rip
  };
}

/*
 * Special Parsers
 */
named!(pub parse_vec_reg<Register>, do_parse!(
  val: alt!(
    parse_512bit_reg |
    parse_256bit_reg |
    parse_128bit_reg |
    parse_mmx_reg 
  ) >>
  (val)
));    
named!(pub parse_reg<Register>, do_parse!(
  val: alt!(
    parse_mmx_reg |
    parse_x87_reg |
    parse_512bit_reg |
    parse_256bit_reg |
    parse_128bit_reg |
    parse_8bit_reg |
    parse_16bit_reg |
    parse_32bit_reg |
    parse_64bit_reg
  ) >>
  (val)
));
named!(pub parse_long_ptr_reg<Register>, do_parse!(
  val: alt!(
    parse_16bit_reg |
    parse_32bit_reg |
    parse_64bit_reg
  ) >>
  (val)
));

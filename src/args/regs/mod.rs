

mod reg_size;
pub use self::reg_size::RegSize;

mod reg;
pub use self::reg::{
  Register,
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
};

mod scale;
pub use self::scale::{
  Scale,
  parse_scale
};

mod disp;
pub use self::disp::Disp;

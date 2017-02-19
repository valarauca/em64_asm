
use super::{
  Disp,
  Register,
  parse_const,
  parse_mem,
  parse_reg,
  Mem,
  parse_vec_reg
};
pub enum Access {
  Z0,
  I(Disp),
  MI(Mem,Disp),
  MR(Mem,Register),
  RM(Register,Mem),
  RR(Register,Register),
  RRVV(Register,Register),
  RVM(Register,Register,Register),
  RVMVV(Register,Register,Register),
  T1S(Register,Register,Register,Disp),
}

/*
 * Build up parsers
 */
named!( end, ws!(tag!(b";")));
named!( comma, ws!(tag!(b",")));

named!(pub parse_z0<Access>, do_parse!(
  end >>
  (Access::Z0)
));
named!(pub parse_i<Access>, do_parse!(
  d: ws!(parse_const) >>
  end >>
  (Access::I(d))
));
named!(pub parse_mi<Access>, do_parse!(
  m: ws!(parse_mem) >>
  comma >>
  d: ws!(parse_const) >>
  end >>
  (Access::MI(m,d))
));
named!(pub parse_mr<Access>, do_parse!(
  m: ws!(parse_mem) >>
  comma >>
  r: ws!(parse_reg) >>
  end >>
  (Access::MR(m,r))
));
named!(pub parse_rm<Access>, do_parse!(
  r: ws!(parse_reg) >>
  comma >>
  m: ws!(parse_mem) >>
  end >>
  (Access::RM(r,m))
));
named!(pub parse_rr<Access>, do_parse!(
  r0: ws!(parse_reg) >>
  comma >>
  r1: ws!(parse_reg) >>
  end >>
  (Access::RR(r0,r1))
));
named!(pub parse_rrvv<Access>, do_parse!(
  r0: ws!(parse_vec_reg) >>
  comma >>
  r1: ws!(parse_vec_reg) >>
  end >>
  (Access::RRVV(r0,r1))
));
named!(pub parse_rvm<Access>, do_parse!(
  r0: ws!(parse_reg) >>
  comma >>
  r1: ws!(parse_reg) >>
  comma >>
  r2: ws!(parse_reg) >>
  end >>
  (Access::RVM(r0,r1,r2))
));
named!(pub parse_rvmvv<Access>, do_parse!(
  r0: ws!(parse_vec_reg) >>
  comma >>
  r1: ws!(parse_vec_reg) >>
  comma >>
  r2: ws!(parse_vec_reg) >>
  end >>
  (Access::RVMVV(r0,r1,r2))
));
  

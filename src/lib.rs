
#![recursion_limit="255"]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate nom;

mod platform;
mod opcodes;
mod args;

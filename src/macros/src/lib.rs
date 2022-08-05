#![recursion_limit = "8"]

extern crate syn;
extern crate quote;

mod tok_vec;
mod arg_rep;
mod fn_rep;

//use arg_rep::*;
//use tok_vec::*;
use fn_rep::*;
//use quote::*;

use proc_macro::{TokenStream};
//use std::str::FromStr;
#[macro_use]
#[proc_macro_attribute]
pub fn invokable(_attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream{
    let fnc = FnRep::fn_rep_from_stream(fn_ts);
    let mut handler = fnc.create_handler();
    handler.extend(fnc.tok_backup);
    return handler;
}
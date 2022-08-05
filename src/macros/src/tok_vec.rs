use proc_macro::{TokenStream,TokenTree};
pub type TokVec = Vec<TokenTree>;
use std::fmt;
pub trait TokVecTriats{
    fn from_stream(stream:TokenStream)->Self;
    fn fmt_tok_vec(f:&mut fmt::Formatter<'_>,vec:&TokVec)->fmt::Result;
    fn to_string(&self)->String;
}
impl TokVecTriats for TokVec{
    fn fmt_tok_vec(f:&mut fmt::Formatter<'_>,vec:&TokVec)->fmt::Result{
        for tok in vec{
            write!(f,"{}",tok.to_string())?;
        }
        return fmt::Result::Ok(());
    }
    fn from_stream(stream:TokenStream)->Self{
        let mut res = Vec::new();
        for tok in stream{
            res.push(tok);
        }
        return res;
    }
    fn to_string(&self)->String{
        use std::fmt::Write;
        let mut res = String::new();
        for tok in self{
            write!(&mut res,"{}",tok.to_string()).expect("formatting error!");
        }
        return res;
    }
}
// fn
// %name%
// (%args%)
// -            optional
// >            optional
// %ret%        optional
// {%body%} 
use crate::tok_vec::*;
use crate::arg_rep::*;
use std::fmt;
use proc_macro::{TokenStream,TokenTree};
pub struct FnRep{
    tok_backup:TokenStream,
    ret:Option<TokVec>,
    args:Vec<ArgRep>,
    name:String,
}
impl fmt::Display for FnRep{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"FnRep{{name:{}",self.name)?;
        match &self.ret{
            Some(s)=>{
                write!(f,",ret:")?;
                TokVec::fmt_tok_vec(f,s)?;
            },
            None=>{},
        }
        write!(f,",args:{{")?;
        for arg in &self.args{
            write!(f,"{}",arg)?;
        }
        write!(f,"}}}}")?;
        return fmt::Result::Ok(());
    }
}
impl FnRep{
    pub fn fn_rep_from_stream(fn_ts:TokenStream) -> FnRep{
        let tok_backup = fn_ts.clone();
        let mut tokens = TokVec::from_stream(fn_ts);
        //body
        tokens.pop();
        //return type
        //TODO:fix this to allow for return values
        let ret = if tokens.len() > 3{
            let tmp = TokVec::from_stream(match tokens.pop().unwrap(){TokenTree::Group(g)=>g.stream(),_=>panic!("unexpected token in place of functionreturn")});
            tokens.pop();tokens.pop(); Some(tmp)
        }else{None};
        //arguments
        let args_tok = tokens.pop().expect("not enough tokens to form a function");
        let args = TokVec::from_stream(
        match args_tok{
            TokenTree::Group(g)=>g.stream(),
            _=>panic!("unexpected token'{}' in place of function args!",args_tok)});
        //name
        let name = tokens.pop().expect("not enough tokens to form a function").to_string();
        return FnRep{tok_backup:tok_backup,ret:ret,args:ArgRep::from_arg_vec(args),name:name}
    }
}
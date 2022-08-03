use crate::tok_vec::*;
use proc_macro::{TokenTree};
pub struct ArgRep{
    name:String,
    arg_type:TokVec,
}
impl ArgRep{
    pub fn from_vec(tokens: TokVec)->ArgRep{
        let mut name_part = Vec::with_capacity(tokens.len());
        let mut arg_type = Vec::with_capacity(tokens.len());
        let mut in_type:bool = false;
        for token in tokens{
            match &token {
                TokenTree::Punct(p)=>{
                    if p.as_char() == ':'{in_type = true; continue;}
                },
                _=>{},
            }
            (match in_type{true=>&mut arg_type,false=>&mut name_part }).push(token);
        }
        assert!(name_part.len()<3);
        let name = name_part[name_part.len() - 1].to_string();
        return ArgRep{name:name,arg_type:arg_type};
    }
    pub fn from_arg_vec(tokens: TokVec)->Vec<ArgRep>{
        let mut args = Vec::new();
        let mut tmp = Vec::new(); 
        for tok in tokens{
            match tok{
                TokenTree::Punct(t)=>{
                    if t.as_char() == ','{
                        args.push(Self::from_vec(tmp));
                        tmp = Vec::new(); 
                        continue;
                    }
                    tmp.push(TokenTree::Punct(t));
                }
                _=>{
                    tmp.push(tok);
                }
            }
        }
        if tmp.len() > 0 {args.push(Self::from_vec(tmp))};
        return args;
    }
} 
use std::fmt;
impl fmt::Display for ArgRep{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"ArgRep{{name:{},type:",self.name)?;
        TokVec::fmt_tok_vec(f,&self.arg_type)?;
        write!(f,"}}")
    }
}

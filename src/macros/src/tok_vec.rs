use proc_macro::{TokenStream,TokenTree};
pub type TokVec = Vec<TokenTree>;
use std::fmt;
pub trait TokVecTraits{
    fn from_stream(stream:TokenStream)->Self;
    fn fmt_tok_vec(f:&mut fmt::Formatter<'_>,vec:&TokVec)->fmt::Result;
    fn to_string(&self)->String;
    fn separate_by_separator(s:Self,separator:char)->Vec<TokVec>;
}
impl TokVecTraits for TokVec{
    fn fmt_tok_vec(f:&mut fmt::Formatter<'_>,vec:&TokVec)->fmt::Result{
        for tok in vec{
            write!(f,"{}",tok)?;
        }
        fmt::Result::Ok(())
    }
    fn from_stream(stream:TokenStream)->Self{
        let mut res = Vec::new();
        for tok in stream{
            res.push(tok);
        }
        res
    }
    fn to_string(&self)->String{
        use std::fmt::Write;
        let mut res = String::new();
        for tok in self{
            write!(&mut res,"{} ",tok).expect("formatting error!");
        }
        res
    }
    fn separate_by_separator(s:Self,separator:char)->Vec<TokVec>{
        let mut tmp:TokVec = Vec::new();
        let mut res:Vec<TokVec> = Vec::new();
        for tok in s{
            match tok{
                TokenTree::Punct(ref p)=>{
                    if p.as_char() == separator{
                        res.push(tmp);
                        tmp = Vec::new();
                    }
                    else{tmp.push(tok)};
                },
                _=>{tmp.push(tok);},
            }
        }
        if !tmp.is_empty(){
            res.push(tmp);
        }
        res
    }
}
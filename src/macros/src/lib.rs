#![recursion_limit = "128"]
#[macro_use]
extern crate syn;
extern crate quote;
use quote::*;
use proc_macro::{TokenStream,TokenTree,Group};
fn get_args(arg_group:Group)->Vec<(Vec<TokenTree>,Vec<TokenTree>)>{
    let mut res = Vec::new();
    let mut tmp = Vec::new();
    for tok in arg_group.stream(){
        match tok{
            TokenTree::Punct(ref p)=>{
                if p.as_char() == ','{
                    res.push(separate_args(tmp));
                    tmp = Vec::new();
                }
                else{
                    tmp.push(tok);
                }
            },
            _=>tmp.push(tok),
        };
    }
    return res;
}
fn separate_args(tokens:Vec<TokenTree>)->(Vec<TokenTree>,Vec<TokenTree>){
    let mut first = Vec::new();
    let mut sec = Vec::new();
    let mut in_type:bool = false;
    for token in tokens{
        match token{
            TokenTree::Punct(ref p)=>{
                if p.as_char() == ':'{in_type = true;}
                else {
                    if in_type{
                        sec.push(token);
                    }
                    else{
                        first.push(token);
                    }
                }
            }
            _=>{
                if in_type{
                    sec.push(token);
                }
                else{
                    first.push(token);
                }
            }
        }
    }
    return (first,sec);
}
use std::str::FromStr;

#[proc_macro_attribute]
pub fn invokable(attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream{
    let mut args:Vec<(Vec<TokenTree>,Vec<TokenTree>)> = Vec::new();
    let mut name:String = "".to_string();
    let mut i = 0;
    for token in fn_ts{
        println!("{}",token);
        if i == 0{
            assert!(
                match token
                {TokenTree::Ident(i)=>{i.to_string() == "fn"},
                    _=>false}
            );
        }
        else if i == 1{
            assert!(
                match token
                {TokenTree::Ident(i)=>{name = i.to_string(); true},
                    _=>false}
            );
        }
        else if i == 2{
            assert!(
                match token
                {
                    TokenTree::Group(g)=>{
                    args = get_args(g);
                    true
                },
                _=>false
            }
            );
        }
        i+=1;
    }
    println!("FNC{{name:{}}}",name);
    let mut tokens = TokenStream::new();
    tokens.extend(TokenStream::from_str("pub extern \"C\" fn ").unwrap());
    tokens.extend(TokenStream::from_str(&("invokable_".to_string()+&name)));
    tokens.extend(TokenStream::from_str("(){}"));
    //tokens.extend(fn_ts);
    return tokens.into();
}

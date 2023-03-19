use crate::tok_vec::{TokVec, TokVecTraits};
use proc_macro::{TokenStream, TokenTree};
pub struct ArgRep {
    pub name: String,
    arg_type: TokVec,
}
use std::str::FromStr;
impl ArgRep {
    pub fn from_vec(tokens: TokVec) -> ArgRep {
        //argument names
        let mut name_part = Vec::with_capacity(tokens.len());
        //argument types
        let mut arg_type = Vec::with_capacity(tokens.len());
        let mut in_type: bool = false;
        for token in tokens {
            if let TokenTree::Punct(p) = &token {
                if p.as_char() == ':' {
                    in_type = true;
                    continue;
                }
            }
            (if in_type {
                &mut arg_type
            } else {
                &mut name_part
            })
            .push(token);
        }
        assert!(name_part.len() < 3);
        let name = name_part[name_part.len() - 1].to_string();
        ArgRep { name, arg_type }
    }
    pub fn from_arg_vec(tokens: TokVec) -> Vec<ArgRep> {
        let mut args = Vec::new();
        let mut tmp = Vec::new();
        let mut depth = 0;
        for tok in tokens {
            match tok {
                TokenTree::Punct(t) => {
                    match t.as_char() {
                        ',' => {
                            if depth == 0 {
                                args.push(Self::from_vec(tmp));
                                tmp = Vec::new();
                                continue;
                            }
                        }
                        '<' => depth += 1,
                        '>' => depth -= 1,
                        _ => (),
                    }
                    tmp.push(TokenTree::Punct(t));
                }
                _ => {
                    tmp.push(tok);
                }
            }
        }
        if !tmp.is_empty() {
            args.push(Self::from_vec(tmp))
        };
        args
    }
    pub fn get_type_string(&self) -> String {
        self.arg_type.to_string()
    }
    pub fn create_handler(&self) -> TokenStream {
        let type_string: String = self.arg_type.to_string();
        let name = &self.name;
        TokenStream::from_str(&format!(
            "let {name} = <{type_string}>::get_rust_rep({name});"
        ))
        .expect("Could not create token stream!")
    }
}
use std::fmt;
impl fmt::Display for ArgRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArgRep{{name:{},type:", self.name)?;
        TokVec::fmt_tok_vec(f, &self.arg_type)?;
        write!(f, "}}")
    }
}

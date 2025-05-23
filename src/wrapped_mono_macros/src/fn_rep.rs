//TODO: while makro system works,this is my first ever macro, and it is not a good written piece of software. It needs a complete rewrite.
use crate::arg_rep::ArgRep;
use crate::tok_vec::{TokVec, TokVecTraits};
use proc_macro::{TokenStream, TokenTree};
use std::fmt;
pub struct FnRep {
    pub tok_backup: TokenStream,
    ret: Option<TokenTree>,
    args: Vec<ArgRep>,
    name: String,
    is_pub: bool,
}
impl fmt::Display for FnRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FnRep{{name:{}", self.name)?;
        if let Some(s) = &self.ret {
            write!(f, ",ret:{s}")?;
        }
        write!(f, ",args:{{")?;
        for arg in &self.args {
            write!(f, "{arg}")?;
        }
        write!(f, "}}}}")?;
        fmt::Result::Ok(())
    }
}
//TODO: Rewrite this function to make it more sensible
fn tok_vec_pop_return(tv: &mut TokVec) -> TokenTree {
    let mut is_last_arrow = false;
    let mut res: Vec<TokenTree> = Vec::with_capacity(tv.len());
    while let Some(tok) = tv.pop() {
        match &tok {
            TokenTree::Punct(p) => {
                if p.as_char() == '>' {
                    is_last_arrow = true;
                    res.push(tok.clone());
                } else if p.as_char() == '-' && is_last_arrow {
                    res.pop();
                    res.pop();
                    let mut ser = Vec::with_capacity(res.len()); //iverted res
                    while let Some(tok) = res.pop() {
                        ser.push(tok);
                    }
                    let val = ser.pop().unwrap();
                    match val {
                        TokenTree::Punct(ref p) => {
                            if p.as_char() != '>' {
                                ser.push(val);
                            }
                        }
                        _ => ser.push(val),
                    }
                    let mut tmp = TokenStream::new();
                    tmp.extend(ser);
                    return TokenTree::Group(proc_macro::Group::new(
                        proc_macro::Delimiter::Parenthesis,
                        tmp,
                    ));
                }
                res.push(tok.clone());
            }
            _ => res.push(tok.clone()),
        }
    }
    panic!("Could not find return");
}
use std::str::FromStr;
impl FnRep {
    fn access(&self) -> &'static str {
        if self.is_pub {
            "pub"
        } else {
            ""
        }
    }
    pub fn fn_rep_from_stream(fn_ts: TokenStream) -> FnRep {
        let tok_backup = fn_ts.clone();
        let mut tokens = TokVec::from_stream(fn_ts);
        //check if public
        let mut is_pub = false;
        if let TokenTree::Ident(i) = &tokens[0] {
            if i.to_string() == "pub" {
                //println!("public function!");
                is_pub = true;
                tokens.remove(0);
            }
        }
        //body
        tokens.pop();
        //return type
        let ret = if tokens.len() > 3 {
            Some(tok_vec_pop_return(&mut tokens))
        } else {
            None
        };
        //arguments
        let args_tok = tokens.pop().expect("not enough tokens to form a function");
        let args = TokVec::from_stream(match args_tok {
            TokenTree::Group(g) => g.stream(),
            _ => panic!("unexpected token'{args_tok}' in place of function args!"),
        });
        //name
        let name = tokens
            .pop()
            .expect("not enough tokens to form a function")
            .to_string();
        FnRep {
            tok_backup,
            ret,
            name,
            args: ArgRep::from_arg_vec(args),
            is_pub,
        }
    }
    pub fn create_in_arg_list(&self) -> TokenStream {
        let mut inner: TokenStream = TokenStream::new();
        let len = self.args.len();
        for (curr, arg) in self.args.iter().enumerate() {
            let separator = if curr < len - 1 { ',' } else { ' ' };
            inner.extend(TokenStream::from_str(&format!(
                "{}:<{} as wrapped_mono::InteropReceive>::SourceType{}",
                arg.name,
                arg.get_type_string(),
                separator
            )));
        }
        let group = TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            inner,
        ));
        TokenStream::from(group)
    }
    /*
        function creating function type(e.g. pub type name_fnc_type = extern "C" fn(arg_type_1,arg_type_2,...)->return_type;
    */
    /// creates a helper target function type used during casting.
    pub fn create_function_type(&self) -> TokenStream {
        // create the begging of function signature
        let mut res = TokenStream::from_str(&format!(
            "{} type {}_fn_type = extern \"C\" fn",
            self.access(),
            &self.name
        ))
        .expect("Could not create token stream!");
        // create function signature argument part eg.(arg1,arg2,arg3)
        let mut fn_sig_params = TokenStream::new();
        // go trough all source function arguments
        for arg in &self.args {
            // append the source type for all arguments
            fn_sig_params.extend(TokenStream::from_str(&format!(
                "<{} as InteropReceive>::SourceType",
                &arg.get_type_string()
            )));
        }
        // extend the fucntion signature by its parameters
        res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            fn_sig_params,
        ))));
        if let Some(return_type) = &self.ret {
            res.extend(TokenStream::from_str("->"));
            res.extend(TokenStream::from(return_type.clone()));
        }
        res.extend(TokenStream::from_str(";"));
        res
    }
    /*
        function creating a function wrapper around souce function
    */
    pub fn create_wrapper(&self) -> TokenStream {
        //println!("{}",self);
        //function signature
        let mut stream: TokenStream = TokenStream::from_str(&format!(
            "{} extern \"C\" fn {}_invokable",
            self.access(),
            &self.name
        ))
        .expect("Could not create token stream!");
        //function args
        stream.extend(self.create_in_arg_list());
        if let Some(ret) = &self.ret {
            stream.extend(TokenStream::from_str("->"));
            stream.extend(TokenStream::from(ret.clone()));
        }
        //argument handlers
        let mut inner: TokenStream = TokenStream::new();
        for arg in &self.args {
            inner.extend(arg.create_handler());
        }
        //inner function call
        //result if needed.
        inner.extend(TokenStream::from_str("use wrapped_mono::interop::InteropSend;"));
        inner.extend(TokenStream::from_str(&format!(
            "let fnc_call_res_val = {}",
            &self.name
        )));
        let mut call_args = TokenStream::new();
        //let curr = 0;
        //let arg_count = self.args.len();
        for arg in &self.args {
            //let c = if curr < arg_count - 1{','}else{' '};
            call_args.extend(TokenStream::from_str(&arg.name));
        }
        inner.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            call_args,
        ))));
        inner.extend(TokenStream::from_str(";"));
        if self.ret.is_some() {
            inner.extend(TokenStream::from_str("return unsafe{fnc_call_res_val.return_value_to_mono()};"));
        }

        stream.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Brace,
            inner,
        ))));
        stream.extend(self.create_function_type());
        //println!("{}",stream);
        stream
    }
}

//TODO: while makro system works, it is not a good written piece of software. It needs a complete rewrite.
use crate::tok_vec::*;
use crate::arg_rep::*;
use std::fmt;
use proc_macro::{TokenStream,TokenTree};
pub struct FnRep{
    pub tok_backup:TokenStream,
    ret:Option<TokenTree>,
    args:Vec<ArgRep>,
    name:String,
}
impl fmt::Display for FnRep{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        write!(f,"FnRep{{name:{}",self.name)?;
        match &self.ret{
            Some(s)=>{
                write!(f,",ret:{}",s)?;
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
//TODO: Rewrite this function to make it more sensible
fn tok_vec_pop_return(tv:&mut TokVec)->TokenTree{
    let mut is_last_arrow = false;
    let mut res:Vec<TokenTree> = Vec::with_capacity(tv.len());
    while let Some(tok) = tv.pop(){
        match &tok{
            TokenTree::Punct(p)=>{
                if p.as_char() == '>'{
                    is_last_arrow = true;
                    res.push(tok.clone());
                }
                else if p.as_char() == '-'{
                    if is_last_arrow{
                        res.pop();
                        res.pop();
                        let mut ser  = Vec::with_capacity(res.len()); //iverted res
                        while let Some(tok) = res.pop(){
                            ser.push(tok);
                        }
                        let val = ser.pop().unwrap();
                        match val{
                            TokenTree::Punct(ref p)=>{
                                if p.as_char() != '>'{
                                    ser.push(val);
                                }    
                            },
                            _=>ser.push(val),
                        }
                        let mut tmp = TokenStream::new();
                        tmp.extend(ser);
                        return TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Parenthesis,tmp));
                    }
                }
                res.push(tok.clone());
            },
            _=>res.push(tok.clone()),
        }
    }
    //println!("res:{}",res.to_string());
    //println!("ila:{}",is_last_arrow);
    panic!("Could not find return");
}
use std::str::FromStr;
impl FnRep{
    pub fn fn_rep_from_stream(fn_ts:TokenStream) -> FnRep{
        let tok_backup = fn_ts.clone();
        let mut tokens = TokVec::from_stream(fn_ts);        
        //body
        tokens.pop();
        //return type
        //TODO:fix this to allow for return values
        let ret = if tokens.len() > 3{
            Some(tok_vec_pop_return(&mut tokens))
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
    pub fn create_in_arg_list(&self)->TokenStream{
        let mut inner:TokenStream = TokenStream::new();
        let len = self.args.len();
        let mut curr = 0;
        for arg in &self.args{
            let separator = if curr < len - 1{','}else{' '};
            inner.extend(
                TokenStream::from_str(&format!("{}_in:<{} as InvokePass>::SourceType{}",arg.name,arg.get_type_string(),separator)));
            curr+=1;
        }
        let group = TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Parenthesis,inner));
        return TokenStream::from(group);
    }
    /*
        function creating function type(e.g. pub type name_fnc_type = extern "C" fn(arg_type_1,arg_type_2,...)->return_type;
    */
    pub fn create_function_type(&self)->TokenStream{
        //function type inner arg types
        let mut inner = TokenStream::new();
        //let curr = 0;
        //let arg_count = self.args.len();
        for arg in &self.args{
            //let c = if curr < arg_count - 1{','}else{' '};
            inner.extend(TokenStream::from_str(&format!("<{} as InvokePass>::SourceType",&arg.get_type_string())));
        }
        let mut res = TokenStream::from_str(&format!("pub type {}_fn_type = extern \"C\" fn",&self.name)).expect("Could not create token stream!");
        //function arguments
        res.extend(TokenStream::from(
            TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Parenthesis,inner))
        ));
        //return value place:res.extend("->");res.extend.(type);

        match &self.ret{
            Some(ret)=>{
                res.extend(TokenStream::from_str("-><"));
                res.extend(TokenStream::from(ret.clone()));
                res.extend(TokenStream::from_str("as InvokeReturn>::ReturnType"));
                
            }
            None=>(),
        }
        res.extend(TokenStream::from_str(";"));
        return res;
    }
    /*
        function creating a function wrapper around souce function
    */
    pub fn create_wrapper(&self) ->TokenStream{
        //function signature
        let mut stream:TokenStream = TokenStream::from_str(&format!("pub extern \"C\" fn {}_invokable",&self.name)).expect("Could not create token stream!");
        //function args
        stream.extend(self.create_in_arg_list());
        match &self.ret{
            Some(ret)=>{
                //println!("#|#\n{}\n#|#",ret);
                stream.extend(TokenStream::from_str("-><"));
                stream.extend(TokenStream::from(ret.clone()));
                stream.extend(TokenStream::from_str("as InvokeReturn>::ReturnType"));
            },
            None=>(),
        }
        //argument handlers
        let mut inner:TokenStream = TokenStream::new();
        for arg in &self.args{
            inner.extend(arg.create_handler());
        }
        //inner function call
        //result if needed.
        inner.extend(TokenStream::from_str(&format!("let fnc_call_res_val = {}",&self.name)));
        let mut call_args = TokenStream::new();
        //let curr = 0;
        //let arg_count = self.args.len();
        for arg in &self.args{
            //let c = if curr < arg_count - 1{','}else{' '};
            call_args.extend(TokenStream::from_str(&format!("{}",&arg.name)));
        }
        inner.extend(TokenStream::from(
            TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Parenthesis,call_args))
        ));
        inner.extend(TokenStream::from_str(&format!(";")));
        match &self.ret{
            Some(ret)=>{inner.extend(TokenStream::from_str(
                &format!("return <{} as InvokeReturn>::get_mono_rep(fnc_call_res_val);",ret)
            ));},
            _=>(),
        }
        stream.extend(TokenStream::from(
            TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace,inner))
        ));
        stream.extend(self.create_function_type());
        //println!("{}",stream);
        return stream;
    }
}
#![recursion_limit = "32"]

extern crate syn;
extern crate quote;

mod tok_vec;
mod arg_rep;
mod fn_rep;
mod method_invoke;
//use arg_rep::*;
//use tok_vec::*;
use fn_rep::*;
//use quote::*;
use crate::tok_vec::{TokVec,TokVecTraits};
use proc_macro::{TokenStream,TokenTree};
use std::str::FromStr;
///This function checks if function "path"(e.g. some_crate::some_module::some_function) 
///is valid (it contains only letters and  ':' signs), and returns the "path" in form of a string if it is.
///NOTE: it will not check if given function exists! it only check if that function path is valid.
///TODO: extend it to filter out such things as 
fn get_fn_path_string(input:TokVec)->Result<String,String>{
    let mut res:String = String::new();
    for tok in input{
        match tok{
            //check if identifier
            TokenTree::Ident(ident)=>{
                res += &ident.to_string();
            },
            //or punct that is ':'
            TokenTree::Punct(p)=>{
                match p.as_char(){
                    ':'=>res+=":",
                    _=>return Err(format!("function path can't contain '{}' charcter!",p.as_char())),
                }
            },
            _=>return Err(format!("function path can't contain '{}' token!",tok)),
        }
    }
    return Ok(res);
}
/// Macro equivalent of mono_add_internal_call with automatic support for type conversion. 
/// Allows you to expose a function as an internal call
/// # Example
/// ## CSharp
/// ```csharp
/// using System.CompilerServices;
/// namespace SomeNamespace{
///     class SomeClass{
///         [MethodImpl(MehodImplOption.InternalCall)]
///         void DoSomething(String arg1);
///     }
/// }
/// ```
/// ## Rust
/// ```rust
/// #[invokable]
/// fn do_something(input:String){
///     println!("done something:{}",input);  
/// }
/// fn expose_do_something(){
///     add_internal_call!("SomeNamespace.SomeClass::DoSomething()")   
/// }
/// ```
#[proc_macro]
pub fn add_internal_call(args: TokenStream) -> TokenStream {
    let mut tokens = TokVec::separate_by_separator(TokVec::from_stream(args),',');
    if tokens.len() != 2{
        return TokenStream::from_str(
            &format!("compile_error!(\"add_internal_call macro requires two arguments but got {} args!\")",tokens.len())
        ).expect("could not create token stream!");
    }
    //using unwrap instead of expect, beacuse previous condition garantees that size of tokens is 2, thus bigger than 0.
    let fnc_name = match get_fn_path_string(tokens.pop().unwrap()){
        Result::Ok(name)=>name,
        Result::Err(msg)=>return TokenStream::from_str(
            &format!("compile_error!(\"{}\")",msg)
        ).expect("could not create token stream!"),
    };
    let method = tokens.pop().unwrap().to_string();
    //unque name for 
    let res =  TokenStream::from_str(
        &format!("let cstr = std::ffi::CString::new({}).expect(\"Could note create cstring\");
        let fnc_ptr:*const core::ffi::c_void = unsafe{{ std::mem::transmute({}_invokable as {}_fn_type) }};
        unsafe{{ wrapped_mono::binds::mono_add_internal_call(cstr.as_ptr(),fnc_ptr) }};
        drop(cstr);",&method,&fnc_name,&fnc_name)).expect("Could not create token stream");
    //println!("{}",res);
    return res;
}
///Macro creating a wrapper around a function making it able to be exposed as internal call.
/// # Restrictions
/// Arguments of function with [`macro@invokable`] atribute must be of types that implement InteropRecive trait.
/// Return type of the function must implement InvokeSend trait.
/// # Example
// Function:
/// ```rust
/// #[invokable]
/// fn print_message(message:String){
///     println!("{}",message); 
/// }
/// ```
/// Will create a wrapper and a function type needed to expose it it mono runtime
/// ```rust
/// extern "C" fn print_message_invokable(message:*mut <String as InteropRecive>::SourceType){
///     let message = <String>::get_rust_rep(message);
///     let res = print_message(message);
/// }
/// pub type extern fn print_message_fn_type = extern "C" fn (<String as InteropRecive>::SourceType); 
/// ```
#[proc_macro_attribute]
pub fn invokable(_attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream{
    let fnc = FnRep::fn_rep_from_stream(fn_ts);
    let mut handler = fnc.create_wrapper();
    handler.extend(fnc.tok_backup);
    return handler;
}
///Invokes method guessing its signature based on types of provided varaibles. Argments should be passed in the following way: 1 - method to invoke, 2 - Option<Object> as "this", None for static, after that any other parameters.
#[proc_macro]
pub fn method_invoke(args: TokenStream) -> TokenStream {
    let mut tokens = TokVec::separate_by_separator(TokVec::from_stream(args),',');
    let params = tokens.split_off(2);
    assert!(tokens.len() == 2);
    let mut res = TokenStream::from_str(&format!("let mut params:Vec<*mut core::ffi::c_void> = Vec::with_capacity({});",params.len())).expect("Could not create token stream!");
    for param in &params{
        let name = param.to_string();
        res.extend(TokenStream::from_str(&format!(
            "let mut param_{} = get_mono_rep_val({});
            \nparams.push(ref_to_cvoid_ptr(&mut {}));",name,name,name
        )));
    }
    res.extend(TokenStream::from_str(&format!(
        "let res = unsafe{{{}.invoke_unsafe({},&params)}};",&tokens[0].to_string(),&tokens[1].to_string()
    )));
    for param in &params{
        let name = param.to_string();
        res.extend(TokenStream::from_str(&format!(
            "drop(param_{});",name
        )));
    }
    res.extend(TokenStream::from_str("res"));
    let res = TokenStream::from(
        TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace,res))
    );
    //println!("'{}'",res.to_string());
    //panic!("TODO")
    return res;
} 
const TS_CR_FAIL:&str = "Colud not create TokenStream!";
///Autoimplement [`InteropRecive`] trait for any type containing only [`IteropRecive`] implementing memebers. Currently supports only structs.
#[proc_macro_derive(InteropRecive)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut input = TokVec::from_stream(input);
    assert!(input.len() == 3);
    let itype = match &input[0]{
        TokenTree::Ident(ident)=>ident.to_string(),
        _=>panic!("type token in derive input is not an identifier!"),
    };
    let iname = match &input[1]{
        TokenTree::Ident(ident)=>ident.to_string(),
        _=>panic!("name token in derive input is not an identifier!"),
    };
    let inner = TokVec::from_stream(match input.pop().expect("Inpossible condition reached. Poping from vec of length 3 yelded nothing."){
        TokenTree::Group(g)=>{
            match g.delimiter(){
                proc_macro::Delimiter::Brace=>g.stream(),
                _=>panic!("unsupported delimiter type!"),
            }
        },
        _=>panic!("inside of a type declaration empty!"),
    });
    let mut type_res = TokenStream::new();
    let mut fn_impl_res = TokenStream::new();
    if itype == "struct"{
        let inner = TokVec::separate_by_separator(inner,',');
        let mut ret_self = TokenStream::new();
        let mut i = 0;
        for memeber in inner{
            let mname = memeber[0].to_string();
            let mtype = memeber[2].to_string();
            println!("name:'{}' type:'{}'",mname,mtype);
            type_res.extend(TokenStream::from_str(
                &format!("<{} as InteropRecive>::SourceType,",mtype)
            ).expect(TS_CR_FAIL));
            fn_impl_res.extend(TokenStream::from_str(
                &format!("let {} = <{} as InteropRecive>::get_rust_rep(arg.{});",mname,mtype,i)
            ).expect(TS_CR_FAIL));
            ret_self.extend(TokenStream::from_str(
                &format!("{}:{},",mname,mname)
            ).expect(TS_CR_FAIL));
            i+=1;
        }
        fn_impl_res.extend(TokenStream::from_str("return Self").expect(TS_CR_FAIL));
        fn_impl_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace,ret_self))));
        fn_impl_res.extend(TokenStream::from_str(";").expect(TS_CR_FAIL));
    }
    else if itype == "enum"{
        //TODO: support trivial(C - like) enums.
        panic!("enums not supported yet.");
    }
    else {panic!("{} is not a valid type!",itype);}
    let mut res = TokenStream::from_str(&format!("impl InteropRecive for {}",iname)).expect(TS_CR_FAIL);
    let mut inner_res = TokenStream::from_str(&"type SourceType = ").expect(TS_CR_FAIL);
    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Parenthesis,type_res))));

    inner_res.extend(TokenStream::from_str(&";").expect(TS_CR_FAIL));
    inner_res.extend(TokenStream::from_str("fn get_rust_rep(arg:Self::SourceType)->Self").expect(TS_CR_FAIL));

    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace,fn_impl_res))));
    res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(proc_macro::Delimiter::Brace,inner_res))));
    return res;
}
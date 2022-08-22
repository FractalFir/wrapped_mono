#![recursion_limit = "32"]

extern crate syn;
extern crate quote;

mod tok_vec;
mod arg_rep;
mod fn_rep;

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
            &format!("compile_error!(\"add_interal_call macro requires two arguments but got {} args!\")",tokens.len())
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
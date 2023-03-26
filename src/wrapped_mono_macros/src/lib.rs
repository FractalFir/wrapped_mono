#![recursion_limit = "32"]

extern crate quote;
extern crate syn;

mod arg_rep;
mod fn_rep;
mod tok_vec;
//use arg_rep::*;
//use tok_vec::*;
use fn_rep::*;
//use quote::*;
use crate::tok_vec::{TokVec, TokVecTraits};
use proc_macro::{TokenStream, TokenTree};
use std::str::FromStr;
///This function checks if function "path"(e.g. some_crate::some_module::some_function)
///is valid (it contains only letters and  ':' signs), and returns the "path" in form of a string if it is.
///NOTE: it will not check if given function exists! it only check if that function path is valid.
///TODO: extend it to filter out such things as
fn get_fn_path_string(input: TokVec) -> Result<String, String> {
    let mut res: String = String::new();
    for tok in input {
        match tok {
            //check if identifier
            TokenTree::Ident(ident) => {
                res += &ident.to_string();
            }
            //or punct that is ':'
            TokenTree::Punct(p) => match p.as_char() {
                ':' => res += ":",
                _ => {
                    return Err(format!(
                        "function path can't contain '{}' charcter!",
                        p.as_char()
                    ))
                }
            },
            _ => return Err(format!("function path can't contain '{}' token!", tok)),
        }
    }
    Ok(res)
}
/// Macro equivalent of mono_add_internal_call with automatic support for type conversion.
/// Allows you to expose a function as an internal call
/// # Parameters
/// | Name | Type | Purpose|
/// --- | --- | ---|
/// | *function_path* | string literal("") | Path to managed function to replace with internal call. Example: "NAMESPCE.CLASS::Method". Managed method to replace must have `[MethodImpl(MehodImplOption.InternalCall)]` atribute|
/// | *function* | rust function | Rust function with `invokable` macro. Must match signature of managed function, otherwise undefined beahviour may occcur.|
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
    let mut tokens = TokVec::separate_by_separator(TokVec::from_stream(args), ',');
    if tokens.len() != 2 {
        return TokenStream::from_str(&format!(
            "compile_error!(\"add_internal_call macro requires two arguments but got {} args!\")",
            tokens.len()
        ))
        .expect("could not create token stream!");
    }
    //using unwrap instead of expect, beacuse previous condition garantees that size of tokens is 2, thus bigger than 0.
    let fnc_name = match get_fn_path_string(tokens.pop().unwrap()) {
        Result::Ok(name) => name,
        Result::Err(msg) => {
            return TokenStream::from_str(&format!("compile_error!(\"{}\")", msg))
                .expect("could not create token stream!")
        }
    };
    let method = tokens.pop().unwrap().to_string();
    //unque name for
    let res =  TokenStream::from_str(
        &format!("let cstr = std::ffi::CString::new({}).expect(\"Could note create cstring\");
        let fnc_ptr:*const core::ffi::c_void = unsafe{{ std::mem::transmute({}_invokable as {}_fn_type) }};
        unsafe{{ wrapped_mono::binds::mono_add_internal_call(cstr.as_ptr(),fnc_ptr) }};
        drop(cstr);",&method,&fnc_name,&fnc_name)).expect("Could not create token stream");
    #[cfg(feature = "dump_macro_results")]
    dumping::dump_stream(&res);
    res
}
/// Macro creating a wrapper around a function making it able to be exposed as internal call.
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
pub fn invokable(_attr_ts: TokenStream, fn_ts: TokenStream) -> TokenStream {
    let fnc = FnRep::fn_rep_from_stream(fn_ts);
    let mut handler = fnc.create_wrapper();
    handler.extend(fnc.tok_backup);
    #[cfg(feature = "dump_macro_results")]
    dumping::dump_stream(&handler);
    handler
}
const TS_CR_FAIL: &str = "Colud not create TokenStream!";
const ENUM_NOT_TRIVIAL:&str = "Could not derive a trait for an non-trivial enum. Trivial enums must be value only and have user set values for compatibility reasons.";
//Extracts enum max value or return None.
fn extract_enum_data(inner: &Vec<Vec<TokenTree>>) -> Option<u32> {
    let mut max_val = 0;
    for member in inner {
        if member.len() != 3 {
            return None;
        };
        match &member[1] {
            TokenTree::Punct(p) => {
                if p.as_char() != '=' {
                    return None;
                }
            }
            _ => return None,
        }
        match &member[2] {
            TokenTree::Literal(l) => {
                let num = match l.to_string().parse() {
                    Ok(num) => num,
                    Err(_) => return None,
                };
                max_val = std::cmp::max(max_val, num);
            }
            _ => return None,
        }
    }
    Some(max_val)
}
/// Autoimplement `InteropRecive` trait for any type containing only `IteropRecive` implementing memebers. Currently supports only structs, and trivial enums(C-like enums) of size less than u64(C# max enum size).
/// # Rust enums
/// Can't be used  with rust-like enums(enums with non-value data), since there is no clear way how this should look like on the managed side.
/// # Safety
/// When converting from C# checks only if value is in enum range between 0 and enum max val.
/// # Enums
/// 1 can have only values
/// 2 have to have set values
/// 3 have to be size smaller than u64, otherwise 'evaluation of constant value failed' error will be thrown(This error is thrown on purpose,
/// since C# enums cant' be bigger than u64. This message means your enum is to big and will cause problems).
#[proc_macro_derive(InteropRecive)]
pub fn derive_recive(input: TokenStream) -> TokenStream {
    let mut input = TokVec::from_stream(input);
    while input.len() > 3 {
        input.remove(0);
    }
    let input_type = match &input[0] {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("type token in derive input is not an identifier!"),
    };
    let input_name = match &input[1] {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("name token in derive input is not an identifier!"),
    };
    let inner = TokVec::from_stream(
        match input
            .pop()
            .expect("Inpossible condition reached. Poping from vec of length 3 yelded nothing.")
        {
            TokenTree::Group(g) => match g.delimiter() {
                proc_macro::Delimiter::Brace => g.stream(),
                _ => panic!("unsupported delimiter type!"),
            },
            _ => panic!("inside of a type declaration empty!"),
        },
    );
    let mut type_res = TokenStream::new();
    let mut fn_impl_res = TokenStream::new();
    let inner = TokVec::separate_by_separator(inner, ',');
    if input_type == "struct" {
        let mut ret_self = TokenStream::new();
        for (i, memeber) in inner.into_iter().enumerate() {
            let member_name = memeber[0].to_string();
            let member_type = memeber[2].to_string();
            type_res.extend(
                TokenStream::from_str(&format!("<{} as InteropRecive>::SourceType,", member_type))
                    .expect(TS_CR_FAIL),
            );
            fn_impl_res.extend(
                TokenStream::from_str(&format!(
                    "let {member_name} = <{member_type} as InteropRecive>::get_rust_rep(arg.{i});"
                ))
                .expect(TS_CR_FAIL),
            );
            ret_self.extend(
                TokenStream::from_str(&format!("{member_name}:{member_name},")).expect(TS_CR_FAIL),
            );
        }
        fn_impl_res.extend(TokenStream::from_str("return Self").expect(TS_CR_FAIL));
        fn_impl_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Brace,
            ret_self,
        ))));
        fn_impl_res.extend(TokenStream::from_str(";").expect(TS_CR_FAIL));
    } else if input_type == "enum" {
        //Check that enum is trivial and its values are set.
        let max_val = extract_enum_data(&inner).expect(ENUM_NOT_TRIVIAL);
        type_res.extend(TokenStream::from_str("u64").expect(TS_CR_FAIL));
        fn_impl_res.extend(TokenStream::from_str(&format!("unsafe{{let ptr = &arg as *const u64; assert!(arg < {},\"Error:Recived enum out of range!\");
         const _: [(); 0 - !{{ const ASSERT: bool = (std::mem::size_of::<{}>() <= std::mem::size_of::<u64>()); ASSERT }} as usize] = [];
         let res = *(ptr as *mut {}); drop(arg); return res;}}",max_val,input_name,input_name)).expect(TS_CR_FAIL));
    } else {
        panic!("{} is not a valid type!", input_type);
    }
    let mut res =
        TokenStream::from_str(&format!("impl InteropRecive for {}", input_name)).expect(TS_CR_FAIL);
    let mut inner_res = TokenStream::from_str("type SourceType = ").expect(TS_CR_FAIL);
    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Parenthesis,
        type_res,
    ))));

    inner_res.extend(TokenStream::from_str(";").expect(TS_CR_FAIL));
    inner_res.extend(
        TokenStream::from_str("fn get_rust_rep(arg:Self::SourceType)->Self").expect(TS_CR_FAIL),
    );

    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Brace,
        fn_impl_res,
    ))));
    res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Brace,
        inner_res,
    ))));
    #[cfg(feature = "dump_macro_results")]
    dumping::dump_stream(&res);
    res
}
/// Autoimplement `InteropSend` trait for any type containing only `IteropSend` implementing members. Currently supports only structs, and trivial enums(C-like enums) of size less than u64(C# max enum size).
/// # Rust enums
/// Can't be used  with rust-like enums(enums with non-value data), since there is no clear way how this should look like on the managed side.
/// # Enums
/// 1 can have only values
/// 2 have to have set values
/// 3 have to be size smaller than u64, otherwise  'evaluation of constant value failed' error will be thrown(This error is thrown on purpose,
/// since C# enums cant' be bigger than u64. This message means your enum is to big and will cause problems).
#[proc_macro_derive(InteropSend)]
pub fn derive_send(input: TokenStream) -> TokenStream {
    let mut input = TokVec::from_stream(input);
    while input.len() > 3 {
        input.remove(0);
    }
    let input_type = match &input[0] {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("type token in derive input is not an identifier!"),
    };
    let input_name = match &input[1] {
        TokenTree::Ident(ident) => ident.to_string(),
        _ => panic!("name token in derive input is not an identifier!"),
    };
    let inner = TokVec::from_stream(
        match input
            .pop()
            .expect("Inpossible condition reached. Poping from vec of length 3 yelded nothing.")
        {
            TokenTree::Group(g) => match g.delimiter() {
                proc_macro::Delimiter::Brace => g.stream(),
                _ => panic!("unsupported delimiter type!"),
            },
            _ => panic!("inside of a type declaration empty!"),
        },
    );
    let mut type_res = TokenStream::new();
    let mut fn_impl_res = TokenStream::new();
    let inner = TokVec::separate_by_separator(inner, ',');
    if input_type == "struct" {
        let mut ret_self = TokenStream::new();
        for memeber in inner {
            let member_name = memeber[0].to_string();
            let member_type = memeber[2].to_string();
            type_res.extend(
                TokenStream::from_str(&format!("<{member_type} as InteropSend>::TargetType,"))
                    .expect(TS_CR_FAIL),
            );
            fn_impl_res.extend(TokenStream::from_str(
                &format!("let {member_name} = <{member_type} as InteropSend>::get_mono_rep(arg.{member_name});")
            ).expect(TS_CR_FAIL));
            ret_self.extend(TokenStream::from_str(&format!("{member_name},")).expect(TS_CR_FAIL));
        }
        fn_impl_res.extend(TokenStream::from_str("return ").expect(TS_CR_FAIL));
        fn_impl_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
            proc_macro::Delimiter::Parenthesis,
            ret_self,
        ))));
        fn_impl_res.extend(TokenStream::from_str(";").expect(TS_CR_FAIL));
    } else if input_type == "enum" {
        //Check that enum is trivial and its values are set.
        let _max_val = extract_enum_data(&inner).expect(ENUM_NOT_TRIVIAL);
        type_res.extend(TokenStream::from_str("u64").expect(TS_CR_FAIL));
        fn_impl_res.extend(TokenStream::from_str(&format!("unsafe{{let mut res:u64 = 0;
            *(&mut res as *mut u64 as *mut {input_name}) = arg;
            const _: [(); 0 - !{{ const ASSERT: bool = (std::mem::size_of::<{input_name}>() <= std::mem::size_of::<u64>()); ASSERT }} as usize] = [];
            return res;}}")).expect(TS_CR_FAIL));
    } else {
        panic!("{input_type} is not a valid type!");
    }
    let mut res =
        TokenStream::from_str(&format!("impl InteropSend for {input_name}")).expect(TS_CR_FAIL);
    let mut inner_res = TokenStream::from_str("type TargetType = ").expect(TS_CR_FAIL);
    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Parenthesis,
        type_res,
    ))));

    inner_res.extend(TokenStream::from_str(";").expect(TS_CR_FAIL));
    inner_res.extend(
        TokenStream::from_str("fn get_mono_rep(arg:Self)->Self::TargetType").expect(TS_CR_FAIL),
    );

    inner_res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Brace,
        fn_impl_res,
    ))));
    res.extend(TokenStream::from(TokenTree::Group(proc_macro::Group::new(
        proc_macro::Delimiter::Brace,
        inner_res,
    ))));
    #[cfg(feature = "dump_macro_results")]
    dumping::dump_stream(&res);
    res
}
#[cfg(feature = "dump_macro_results")]
mod dumping {
    static mut HAS_BEGUN_DUMP: bool = false;
    pub fn dump_stream(stream: &proc_macro::TokenStream) {
        use proc_macro::Span;
        use std::io::Write;
        let mut file = get_dump_file();
        writeln!(
            file,
            "//##############################################################################"
        );
        let span = Span::call_site();
        writeln!(
            file,
            "// inserted in file `{}` at `{}:{}`",
            span.source_file().path().display(),
            span.end().line,
            span.end().column
        );
        writeln!(file, "{}", stream);
    }
    pub fn get_dump_file() -> std::fs::File {
        use std::fs::OpenOptions;
        let path = "macro.dump";
        if unsafe { HAS_BEGUN_DUMP } {
            OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .expect("Could not open macro dump file!")
        } else {
            unsafe { HAS_BEGUN_DUMP = true };
            std::fs::File::create(path).expect("Could not create macro dump file!");
            OpenOptions::new()
                .write(true)
                .open(path)
                .expect("Could not create macro dump file!")
        }
    }
}

//TODO: maybe autoimplement interop box?

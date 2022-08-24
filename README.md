# wrapped_mono
 `wrapped_mono`is a safe lightweight wrapper around the mono library. It allows embeding the mono runtime(an open-source .NET runtime) inside rust code, and running code written in languges from the .NET framework. `wrapped_mono` allows for easy interop between managed and unmanaged code. Built-in macros automaticaly convert types when passing data between native code and code run inside the runtime.
## W.I.P
 While `wrapped_mono` is mostly finished, there are still few rough edges that need some polish. Some more obscure features do not have safe wrappers yet.
## What `wrapped_mono` **is not**
 `wrapped_mono` aims to be as lightweight as possible while still providing a safe and convnient API. While there are checks to ensure `wrapped_mono` works propely and in case of undefined behavior or crashes proper error messages will be printed,**not every mistake can be caught without a substantial effect on preformance**. That kinds of errors are **not handled**, and handling errors that can be caught relativly easily but still have some preformance impact can be disabled to gain even more preformance. It means that it is still possible to make certain kinds of mistakes (accesing objects after deleting them by deleting domain there are in, etc).
# Dependencies
 `wrapped_mono` depends on the mono library, since it is just a wrapper. Mono library can be downloaded <a href="https://www.mono-project.com/download/stable/">here</a>.
# Fetures and planned features
## Version 0.1
- [x] Runtime initialization/shutdown
- [x] Creating multpile domains
- [x] Loading assemblies from disk
- [x] Searching for a specific class in assembly
- [x] Creating new object of a given type
- [x] Getting method of a class
- [x] Calling a method - both static and not static
- [x] Utilities related to objects (getting their size, converting them to strings)
- [x] Boxing and unboxing values
- [x] Getting/Setting object field
- [x] Cloning objects
- [x] Managed string support
- [x] Array creation
- [x] Getting/Setting array at index
- [x] Exception creating
- [x] Raising Exceptions
- [x] Catching Exceptions
- [x] Getting common type classes
- [x] Loading config files
- [x] Signal chaining
- [x] Exposing unmanged functions as internal calls in managed code
- [x] Passing data between managed and unmanaged code
- [ ] Support for properities (getters,setters, etc)
- [ ] Implementation of Inertop Traits for all simple types.
- [ ] Full documentaion for all features - 95% done
## Version 0.2
- [ ] Arrays with more than one dimension. (Altough there is no support for multidimensional arrays, they can be still used by enabling unsafe_arrays fetures and indexing into them like into an 1 - dimensional array(they can't be created yet too))
- [ ] Autoimplementation of some Interop traits (Interop Send,InteropRevive) for structs.
## Version 0.3
- [ ] Debugging API
- [ ] Dynamic code generation
- [ ] Certain fetures of mono JIT(mostly debugging)
- [ ] Reading of assembly meatdata
- [ ] Profilier (Data about preformance)
- [ ] Seciurity API
- [ ] Features related to threads

# Examples
<p align = "center">
    <a href="#Loading">Loading basic assembly<a>&nbsp;
    <a href="#Creating new domains">Creating new domains<a>&nbsp;
    <a href="#Executing manged code">Executing manged code<a>&nbsp;
    <a href="# Exposing rust functions as internal calls">Exposing rust functions using internal calls<a>&nbsp;
</p>

## Loading basic assembly
**WARNING** Mono JIT can only be initialized once, and calling Domain::init_jit more times will lead to crash.
```rust
use wraped_mono::*;
fn main(){
    //Initializing mono JIT and creating root domain with name "root" and no version specifincation (default runtime version)
    let domain = jit::init("root",None);
    //Loading assembly 'Test.dll'
    let assembly = domain.asembly_open("Test.dll").unwrap();
}
```
## Creating new domains
**WARNING**!<br> creating root domain and initializing JIT is a necessary step that must be done before creating other domains.
```rust
fn main(){
    let domain = jit::init("root",None);
    //creating another domain 
    let domain = Domain::create();
}
```
## Executing manged code
```rust
fn main(){
    //initalizing jit
    let domain = jit::init("root",None);
    //opening assembly
    let assemmbly = domain.assembly_open("Some.dll").unwrap();
    //creating structure containing arguments to be passed as string- [] args
    let args:Vec<&str> = Vec::new();
    //calling main function in managed code
    jit::exec(dom,assembly,args);
}
```
## Exposing rust functions as internal calls
```cs
    class SomeClass{
        - [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void SomeFunction(string arg);
        - [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void OtherFunction(int arg);
        - [MethodImplAttribute(MethodImplOptions.InternalCall)]
        int ArrayFunction(int- [] arg);
        - [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void PointerFunction(System.IntPtr arg);
        - [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void CharFunction(char arg);
    }
```
```rust
    #- [invokable]
    fn some_function(arg:String){
        println!("recived arg:'{}'!",arg);
    }
    #- [invokable]
    fn other_function(arg:i32){
        println!("recived arg:'{}'!",arg);
    }
    #- [invokable]
    fn pointer_function(arg:*mut i64){
        println!("recived pointer:'{}'!",arg);
    }
     #- [invokable]
    fn char_function(arg:char){
        println!("recived UTF-8 char!(supports symbols like ó ö ❤️)",arg);
    }
    #- [invokable]
    fn array_function(arg:Array<i32>)->i32{
        let len = arg.len();
        for i in 0..len{
            println!("element number {} is :'{}'!",arg.get(i));
        }
        return len;
    }
    fn main(){
        /*
            jit initialization,domain creation, assembly loading, etc.
        */
        add_internal_call!("SomeClass::SomeFunction",some_function);
        add_internal_call!("SomeClass::OtherFunction",some_function);
        add_internal_call!("SomeClass::ArrayFunction",array_function);
        add_internal_call!("SomeClass::PointerFunction",pointer_function);
        add_internal_call!("SomeClass::CharFunction",char_function);
        /*
            managed code executed afer this point will call rust code when functions exposed as interall calls will be called
        */
    }
```
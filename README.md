# wrapped_mono
 `wrapped_mono` is a safe lightweight wrapper around the mono library. It allows embeding the mono runtime(an open-source .NET runtime) inside rust code, and running code written in languges from the .NET framework. `wrapped_mono` allows for easy interop between managed and unmanaged code. Built-in macros automaticaly convert types when passing data between native code and code run inside the runtime.
## W.I.P
 While `wrapped_mono` is mostly finished, there are still few rough edges that need some polish. Some more obscure features do not have safe wrappers yet.
## What `wrapped_mono` **is not**
 `wrapped_mono` aims to be as lightweight as possible while still providing a safe and convnient API. While there are checks to ensure `wrapped_mono` works propely and in case of undefined behavior or crashes proper error messages will be printed,**not every mistake can be caught without a substantial effect on preformance**. That kinds of errors are **not handled**, and handling errors that can be caught relativly easily but still have some preformance impact can be disabled to gain even more preformance. It means that it is still possible to make certain kinds of mistakes (accesing objects after deleting them by deleting domain there are in, etc).
# Dependencies
 `wrapped_mono` depends on the mono library, since it is just a wrapper. Mono library can be downloaded <a href="https://www.mono-project.com/download/stable/">here</a>.
# Supported platforms
 `wrapped_mono` works well with linux, but windows support is not finished(some files have to be manualy copied).
## Windows Issues
  Outside the crate, in the directory containing it a directory named `lib` must be created(for example, in case of a crate called `my_crate` `lib` shoudl be in the same directory `my_crate` is in). This folder is the default location of 'mscorlib.dll'. To change this location call `runtime::set_dirs` before `jit::init`. This folder must contain copy of file `mscorlib.dll` from `C:\Program Files\Mono\lib\mono\{MONO_RUNTIME_VERSION}\mscorlib.dll`. Root folder of a crate using `wrapped_mono` must contain copy of file `mono-2.0-sgen.dll` from `C:\Program Files\Mono\bin\mono-2.0-sgen.dll`.
## MacOS Support
  `wrapped_mono` likey works on MacOS with linux copilation flags, but since I have no acces to a mac computer, it was not tested yet. (because of that, compilation will stop with "your os is not supported" message, change `build.rs` to enable compiling on MacOS).
## Cross-Compilation
  Cross compiling `warpped_mono` is not supported yet(flags set by `build.rs` assume target system is also the host system), but support for it can be added by seeting proper flags.
## Any other platform
  In order to compile for any other platform 3 requirements must be met: this platform is supported by mono, rust can be compiled for it, and proper flags are set in `build.rs`.
# Documentaion
 `wrapped_mono` contains full documentaion inside it. In order to open documentation download `wrapped_mono` and run `cargo doc --open`.
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
- [X] Support for properities (getters,setters, etc)
- [X] Implementation of Inertop Traits for all simple types.
- [X] Full documentaion for all features
## Version 0.2
- [X] Arrays with more than one dimension. (Multi-dimensional arrays work, but they behave like 1D arrays. This may be a limitation of the mono runtime, or it could be solvable)
- [ ] Autoimplementation of some Interop traits (Interop Send,InteropRevive) for structs.
- [ ] Profilier (Data about preformance)
- [ ] Debugging API
- [ ] Certain fetures of mono JIT(mostly debugging)
## Version 0.3
- [ ] Dynamic code generation
- [ ] Reading of assembly meatdata
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
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void SomeFunction(string arg);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void OtherFunction(int arg);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        int ArrayFunction(int[] arg);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void PointerFunction(System.IntPtr arg);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void CharFunction(char arg);
    }
```
```rust
    #[invokable]
    fn some_function(arg:String){
        println!("recived arg:'{}'!",arg);
    }
    #[invokable]
    fn other_function(arg:i32){
        println!("recived arg:'{}'!",arg);
    }
    #- [invokable]
    fn pointer_function(arg:*mut i64){
        println!("recived pointer:'{}'!",arg);
    }
    #[invokable]
    fn char_function(arg:char){
        println!("recived UTF-8 char!(supports symbols like ó ö ❤️)",arg);
    }
    #[invokable]
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
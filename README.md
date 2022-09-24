# wrapped_mono
 `wrapped_mono` is a safe lightweight wrapper around the mono library. It allows embedding the mono runtime(an open-source .NET runtime) inside rust code, and running code written in languages from the .NET framework. `wrapped_mono` allows for easy interop between managed and unmanaged code. Built-in macros automaticaly convert types when passing data between native code and code run inside the runtime.
## W.I.P
 While `wrapped_mono` is mostly finished, there are still a few rough edges that need some polish. Some more obscure features do not have safe wrappers yet. In order to avoid problems with changes to API during development, add wrapped_mono to Cargo.toml in the following way:`wrapped_mono = {git = "https://github.com/FractalFir/wrapped_mono.git",tag = "VERSION_NUMBER"}`. This will make you stay at release *VERSION_NUMBER*.
## What `wrapped_mono` **is not**
 `wrapped_mono` aims to be as lightweight as possible while still providing a safe and convenient API. While there are checks to ensure `wrapped_mono` works properly and in case of undefined behavior or crashes proper error messages will be printed,**not every mistake can be caught without a substantial effect on performance**. That kinds of errors are **not handled**, and handling errors that can be caught relatively easily but still have some performance impact can be disabled to gain even more performance. It means that it is still possible to make certain kinds of mistakes (accessing objects after deleting them by deleting the domain there are in, etc.).
# Dependencies
 `wrapped_mono` depends on the mono library, since it is just a wrapper. Mono library can be downloaded <a href="https://www.mono-project.com/download/stable/">here</a>.
# Supported platforms
 `wrapped_mono` works well with Linux, but windows support is not finished(some files have to be manually copied).
## Windows Issues
  Outside the crate, in the directory containing it a directory named `lib` must be created(for example, in case of a crate called `my_crate` `lib` should be in the same directory `my_crate` is in). This folder is the default location of 'mscorlib.dll'. To change this location, call `runtime::set_dirs` before `jit::init`. This folder must contain a copy of file `mscorlib.dll` from `C:\Program Files\Mono\lib\mono\{MONO_RUNTIME_VERSION}\mscorlib.dll`. Root folder of a crate using `wrapped_mono` must contain copy of file `mono-2.0-sgen.dll` from `C:\Program Files\Mono\bin\mono-2.0-sgen.dll`.
## MacOS Support
  `wrapped_mono` likely works on MacOS with Linux compilation flags, but since I have no access to a Mac computer, it was not tested yet. (because of that, compilation will stop with "your OS is not supported" message, change `build.rs` to enable compiling on MacOS).
## Cross-Compilation
  Cross compiling `warpped_mono` is not supported yet(flags set by `build.rs` assume target system is also the host system), but support for it can be added by setting proper flags.
## Any other platform
  In order to compile for any other platform 3 requirements must be met: this platform is supported by mono, rust can be compiled for it, and proper flags are set in `build.rs`.
# Documentation
 `wrapped_mono` contains full documentation inside it. In order to open documentation, download `wrapped_mono` and run `cargo doc --open`.
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
- [X] Autoimplementation of some Interop traits (Interop Send,InteropRecive) for structs. 
- [X] Reading of assembly meatdata - primitive API, all features work, but API is hard to use(a lot of steps to retrive data).
- [ ] Rework of Method type, change invocation API to a safe one and implement Fn tratis/
- [ ] Delegate support
- [ ] Event support
## Version 0.3
- [ ] Profilier (Data about preformance) *partaily done*
- [ ] Debugging API
- [ ] Certain fetures of mono JIT(mostly debugging) 
## Version 0.4
- [ ] Dynamic code generation
- [ ] Seciurity API
- [ ] Features related to threads
- [ ] Reading of assembly meatdata - full API, easy to use
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
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void EnumFunction(SimpleEnum arg);
        [MethodImplAttribute(MethodImplOptions.InternalCall)]
        void StructFunction(Vec3 arg);
    }
    struct Vec3{
     float x;
     float y;
     float z;
    }
    enum SimpleEnum{
        Variant1 = 0,Variant2 = 7,
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
    #[invokable]
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
    #[invokable]
    fn enum_function(arg:SimpleEnum){
      match arg{
         SimpleEnum::Variant1=>println!("recived Variant1"),
         SimpleEnum::Variant2=>println!("recived Variant2"),
      }
    }
     #[invokable]
    fn struct_function(vec:Vec3){
      println!("recived vec3 with x:{},y:{},z:{}",vec.x,vec.y,vec.z);
    }
    #[derive(InteropSend,InteropRecive,Copy,Clone)]
    enum SimpleEnum{
     Variant1 = 1,
     Variant2 = 7,
    }
    #[derive(InteropSend,InteropRecive,Copy,Clone)]
    struct Vec3{
     x:f32,
     y:f32,
     z:f32,
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
### Invoke Managed Method
```csharp
class TestFunctions{
 public static int GetArg(int arg){
        return arg;
    }
}
```
```rust
    fn calling_method(){
        use wrapped_mono::{jit,class::Class,method::Method};
        use crate::interop::{get_mono_rep_val,ref_to_cvoid_ptr};
        use macros::*;
        let dom = jit::init("root",None);
        //open assembly
        let asm = dom.assembly_open("test/dlls/Test.dll").unwrap();
        let img = asm.get_image();
        //find class method is in
        let class = Class::from_name(&img,"","TestFunctions").expect("Could not get class");
        //get method from class
        let met = Method::get_method_from_name(&class,"GetArg",1).unwrap();
        let mut arg1:i32 = 7;
        //this macro gueses C# signature bassed on passed arguments
        let obj = method_invoke!(met,None,arg1).expect("Exception").expect("Got null on a non-nullable!");
        //unbox result to get value from object
        let res = obj.unbox::<i32>();
        assert!(res == arg1);
    }
```

# wrapped_mono
`wrapped_mono` is a safe, lightweight wrapper around the mono library. It allows embedding of the mono runtime inside a rust project. Inside this
embedded runtime code written in languages supporting the .NET framework, such as C# and F#, can be run. This allows usage of libraries written in those languages, and using them as a scripting language. The mono runtime is used by many game engines, and this wrapper allows using it with projects written in Rust too.
# WIP
## Lacking APIs
While `wrapped_mono` already has support for most of the features of the mono runtime, some minor APIs don't have finished and fully tested wrappers. Those unfinished APIs are usually niche(eg. advanced debugging, access to profiler(data about performance), access to assembly Metadata, dynamic code generation) and always have an alternative unsafe bindings that can be used.
## Safety checks
This API tries to follow rusts rules about safety and error handling as much as possible, but some checks are unfinished and can't catch all potential problems, or are not done, since they would introduce a serious performance hit, while only covering a niche case that is clearly marked in documentation. A good example of this kind of case is accessing an object after deleting the domain it is in or shutting down the runtime. Most of possible errors are checked for, and those checks can be disabled to speed up `wrapped_mono` even more, but this is not advised. Cost of those checks is usually negligible(less than 1% of the cost of calling a function), and they prevent a lot of potential mistakes. 
# Supported platforms
`wrapped_mono` supports Linux(tested on Fedora 37, Debian Bullseye and Arch), and Windows(tested on Windows 10). Other platforms, such as MacOS are not officially supported, but can be easily added by changing the `build.rs` to include platform-specific link flags.
Cross-compilation is not supported yet, but support for it is planned in the future.
# Why `wrapped_mono` requires rust nightly?
Rust nightly allows use of unfinished features of the rust language that are used by `wrapped_mono` to either increase safety(specialization used to preform method signature checks) or simplify usage of the APIs provided by `wrapped_mono`(Arrays using constant generics to represent their number of dimensions). Those features are necessary for `wrapped_mono` to work properly, but are used sparingly in as simple as possible cases. Each use of unfinished feature was made with a lot of caution, and should not be affected by any changes to the rust compiler.
# Dependencies
## External
* Mono library - the library this crate wraps around. Can be downloaded <a href="https://www.mono-project.com/download/stable/">here</a>. When installing, use default instructions from the website. Only needed on the system crate is compiled on (linked statically).
## Rust 
* `wrapped_mono_macros` - sub crate containing custom macros used by wrapped_mono. When cloned from github, bundled by default. Separate in order to make cargo happy.
* `document-features` - used for documentation 
* `lazy_static` - used to lazy-load classes such as *System.Delegate* when using the crate.
# Features
## Fully finished
- [X] Initialize mono runtime
- [X] Create different application domains to prevent code from different assemblies from interacting.
- [X] Load assemblies and get classes from them
- [X] Create new instances of a class
- [X] Get methods from a class - static, non-static, virtual - with signature checks
- [X] Call methods - static, virtual, on a specific object
- [X] Read and Set fields of a class - static, non-static
- [X] Use getter, setter and indexer properties of a class.
- [X] Box, unbox, clone, get hash, size, cast, convert Object to strings.
- [X] Raise and catch exceptions
- [X] Create n-dimensional Arrays, read and set their elements at any indices.
- [X] Pass basic types(integers, chars, floating-point numbers, pointers, arrays, strings, exceptions,objects,types,delegates) between managed and unmanaged code
- [X] Invoke deleagtes
- [X] Implement simple traits to pass any type between rust and C#/F# code!
- [X] Automatically implement interop helper traits for any structs made from other types implementing the helper traits.
- [X] Pass back and forth simple rust enums.
- [X] Implement C# method as internal calls to Rust code.
- [X] Automatically register all references to managed objects from rust code. Every reference to an object in rust code is registered, and Garbage Collector is automaticaly informed when it can and can't run.
- [X] Load custom conifg files.
## WIP features
- [ ] Profiler - get useful insights about how your C#/F# assembly runs - which objects are created, how often is each method called(Already supports couple different events, but contains some bugs)
- [ ] Assembly Metadata Access - Works, but is not finished.
## Planned features
- [ ] C#/F# assembly binding generator
- [ ] Dynamic code generation
- [ ] Security API
- [ ] Rust representation of `System.Thread`
- [ ] Debugging features
# Example
```rust
    use wrapped_mono::*;
    fn main(){
        // Initialize the runtime with default version(`None`), and root domian named "main_domain"
        let domain = jit::init("main_domain",None); 
        // Load assembly "SomeAssembly.dll" 
        let assembly = domain.assembly_open("SomeAssembly.dll").expect("Could not load assembly!"); 
        // Get the image, the part of assembly containing executable code(classes,methods, etc.)
        let image = assembly.get_image(); 
        // Get class named SomeClass in SomeNamespace
        let class = Class::from_name(&image,"SomeNamespace","SomeClass").expect("Could not find SomeClass!);
        // Create an instance of this class
        let instance = Object::new(&domain,&class);
        // Creating an instance of a class DOES NOT CALL ITS CONSTRUCTOR. The constructor is a method named '.ctor', that has to be called separately
        // Get a constructor method of SomeClass accepting an intiger and a string (2 parameters)
        let ctor:Method<i32,String> = Method::get_from_name(class,".ctor(int,System.String)",2);
        // Call the constructor
        ctor.invoke(Some(instance),(12,"SomeString".to_owned())).expect("Got an exception while calling the constuctor!");
        // Get a method "DoABackflip" form SomeClass with 1 parameter of type int returning a byte
        let met:Method<i32,String> = Method::get_from_name(class,"DoABackflip",1);
        // Call "DoABackflip" method on an instance
        let res = met.invoke(Some(instance),32).expect("Got an exception while calling DoABackflip!").expect("Got null from DoABackFlip");
        // Unbox the result to get a raw intiger from a boxed intiger
        let res = obj.unbox::<u8>();
        // Create a function with the special "invokable" atribute
        #[invokable]
        fn sqrt(input:f32){
            if input < 0{
                // can't get sqrt of a negative number, so create a managed exception and throw it
                Exception::arthmetic().raise();
            }
            return input.sqrt();
        }
        // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" atribute with a rust function
        add_internal_call!("SomeClass::SqrtInternalCall",sqrt);
        // This supports all types with `InteropRecive` trait
        #[invokable]
        fn avg(input:Array<1,f32>){
            let mut avg = 0.0;
            for i in 0..input.len(){
                let curr = input.get(&[i])// get the element at index i
                avg += curr/input.len();
            }
            return avg;
        }
        // Replace a method with "[MethodImplAttribute(MethodImplOptions.InternalCall)]" atribute with a rust function
        add_internal_call!("SomeClass::AvgInternalCall",sqrt);
    }
```

# wrapped-mono
## About
**wrapped-mono** is a wrapper around mono library, allowing for safe interreaction between rust code and mono runtime. **wrapped-mono** is WIP and does not support most of mono functionalities yet.
## Features
- [x] Mono JIT initalization  
- [x] Mono Domain creation
- [X] Loading mono assemblies
- [X] Getting image from assembly
- [ ] Getting classes from image
- [ ] Getting functions from image
- [ ] Getting functions from classes
- [ ] Static CLR function execution
- [ ] Constructing new class instances
- [ ] Calling CLR function on class instance
- [ ] Passing arguments to CLR functions
- [ ] Reciving data from CLR functions
- [ ] Getting accesing instance variable fields 
- [ ] Exposing rust functions to CLR using P/Invoke
- [?] Delegate Support
## Example
```
fn main(){
    //Creating new domain with name main and no version specifincation (default version)
    let domain = Domain::init_jit(Some("main"),None);
    let assembly = Assembly::open(domain,"Test.dll").expect("Could not load assembly!);
}
```
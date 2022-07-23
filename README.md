# wrapped-mono
<p align = "center">
     <a href="#About">About<a> 
     <a href="#Features">Features<a> 
     <a href="#Examples">Examples<a> 
</p>

## About
**wrapped-mono** is a wrapper around mono library, allowing for safe interreaction between rust code and mono runtime. **wrapped-mono** is WIP and does not support most of mono functionalities yet.
## Features
- [x] Mono JIT initalization  
- [x] Mono Domain creation
- [X] Loading mono assemblies - *works, altough is not yet stable and crasehs under certain conditions*
- [ ] Getting image from assembly
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
- [ ] Delegate Support
## Examples
<p align = "center">
    <a href="#Loading">Loading basic assembly<a>
    <a href="#Creating new domains">Creating new domains<a>
</p>

### Loading basic assembly
**WARNING** Mono JIT can only be initialized once, and calling Domain::init_jit more times will lead to crash.
```rust
fn main(){
    //Initizlizing mono JIT and creating root domain with name "main" and no version specifincation (default runtime version)
    let domain = Domain::init_jit(Some("main"),None);
    //Loading assembly 'Test.dll'
    let assembly = Assembly::open(domain,"Test.dll").expect("Could not load assembly!);
}
```
### Creating new domains
**WARNING**!<br> creating root domain and initializing JIT is a necesary step that must be done before creating other domains.
```rust
fn subdomain(){
    //creating domain with no name or config
    let domain = Domain::create();
    //creaing domain with name, but no config
    let named_domain = Domain::create_appdomain("John",None);
      //creaing domain with name, and config
    let coinfigured_domain = Domain::create_appdomain("Susan","cfg/Susan_cfg_file.sfg");
}
```
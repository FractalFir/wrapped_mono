use wrapped_mono::*;
fn managed_string_creation(){
    //Gets current domain
    let curr_domain = Domain::get_curr().expect("Could not get current domain");
    //Creates string in current domain
    let mstr = MString::new(&curr_domain,"string content");
}
fn main() {
    //initalizing mono runtime with root domain with name "domain_name" and defalut version (No version requrement)
    let main_domain = jit::init("domain_name",None);
    managed_string_creation();
}

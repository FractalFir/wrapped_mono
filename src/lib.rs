#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod binds;
pub mod domain;
pub mod assembly;
//those tests use this library in diffrent ways than normal user would
//(they must share one mono JIT across separate function that can be called in any order, and there is no guarante that any single one will be called).
//It forces ceratin libraries(lazy static) to be included for testing purposes, even tough they are not going to be used normaly.
#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use super::*;
    use crate::domain::{Domain,DomainTraits};
    use crate::assembly::{Assembly,AssemblyTraits};
    lazy_static! {pub static ref JIT_DOMAIN:Domain = Domain::init_jit(Some("main"),None);}
    #[test]
    fn jit_init(){
        //getting id of JIT_DOMAIN ensures jit is propely initialized(accesing lazy static calls init_jit)
        JIT_DOMAIN.get_id();
    }
    #[test]
    fn assembly_loading(){

        println!("loading assembly");
        let assembly:Assembly = Assembly::open(JIT_DOMAIN.clone(),"test_dlls/Test.dll").expect("could not load assembly!");
    }
    //TODO: fix errors related to multiple mono domains
    //#[test]
    fn multiple_domains(){
        JIT_DOMAIN.get_id();
        let Domain = Domain::create();
    }
    #[test]
    #[should_panic]
    fn missing_assembly_loading(){
        let assembly:Assembly = Assembly::open(JIT_DOMAIN.clone(),"test_dlls/Missing.dll").expect("could not load assembly!");
    }
}

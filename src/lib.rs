pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod binds;
pub mod domain;
pub mod assembly;
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Domain,DomainTraits};
    #[test]
    fn jit_initialization() {
        println!("creating domain!");
        let domain:Domain = Domain::init_jit(None,None);
        //domain.jit_cleanup();
    }
    #[test]
    fn assembly_loading(){
        
    }
}

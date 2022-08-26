
// comment out all #[cfg(test)] and bindgen dependency to renable bind generation
#[cfg(test)]
mod binds{
    extern crate bindgen;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;
    pub fn gen_binds(){
        let hdr_path = "src/cbinds/binds.h";
        let res_path = "src/binds/mod.rs";
        if std::path::Path::new(&res_path).exists(){
            return;
        }
        let bindings = bindgen::Builder::default()
        .header(hdr_path)
        .generate()
        .expect("Unable to generate mono bindings");
        let out_path = PathBuf::from(res_path);
        let mut file = File::create(out_path).expect("Couldn't create bindings file!");
        file.write_all(
            b"#![allow(improper_ctypes)]\n#![allow(non_upper_case_globals)]\n
            #![allow(non_camel_case_types)]\n
            #![allow(non_snake_case)]\n"
        ).expect("Could not write bindings prefix");
        bindings.write(Box::new(file)).expect("Couldn't write bindings!");
    }
}
#[cfg(test)]
mod tests{
    use std::process::Command;
    /*
    pub fn compile_assembly(src_path:&str,target_path:&str){
        let output = Command::new("mcs") 
        .arg(&format!("-out:{}",target_path))
        .arg("-debug:embeded")
        .arg(src_path)
        .output()
        .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0{
            panic!("{}",std::str::from_utf8(&stderr).unwrap());
        }
    }
    */
    pub fn compile_jit_test_assembly(){
        let output = Command::new("mcs") 
        .arg("-out:test/dlls/Jit.dll")
        .arg("test/Jit.cs")
        .output()
        .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0{
            panic!("{}",std::str::from_utf8(&stderr).unwrap());
        }
    }
    pub fn compile_pinvoke_test_assembly(){
        let output = Command::new("mcs") 
        .arg("-out:test/dlls/Pinvoke.dll")
        .arg("test/Pinvoke.cs")
        .output()
        .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0{
            panic!("{}",std::str::from_utf8(&stderr).unwrap());
        }
    }
    pub fn compile_test_lib(){
        let output = Command::new("mcs") 
        .arg("-target:library") 
        .arg("-out:test/dlls/Test.dll")
        .arg("test/Test.cs")
        .output()
        .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0{
            panic!("{}",std::str::from_utf8(&stderr).unwrap());
        }
    }
}
//TODO:extend this function(use symlinks?) to allow for multipile versions of mono to be used.
fn copy_win_dlls(){
    use std::path::Path;
    //check if "mono-2.0-sgen.dll" present, and copy it if not
    let msgen_taraget_path = Path::new("mono-2.0-sgen.dll");
    if !msgen_taraget_path.exists(){
        let msgen_source_path =  Path::new("C:\\Program Files\\Mono\\bin\\mono-2.0-sgen.dll");
        std::fs::copy(msgen_source_path,msgen_taraget_path).expect("Could not copy `mono-2.0-sgen.dll` file. Is mono propely installed on your system?");
    }
    //check if mscorlib.dll file is present and if not copy
    let mcl_target_path = Path::new("..\\lib\\mono\\4.5\\mscorlib.dll");
    if !mcl_target_path.exists(){
        let mcl_source_path = Path::new("C:\\Program Files\\Mono\\lib\\mono\\4.5\\mscorlib.dll");
        std::fs::copy(mcl_source_path,mcl_target_path).expect("Could not copy `mscorlib.dll` file. Is mono propely installed on your system?");
    }
}
fn main() {
    #[cfg(not(any(target_os = "linux",target_os = "windows")))]
    panic!("Target OS currently not supported");
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=mono-2.0");
    }
    #[cfg(target_os = "windows")]
    {
        //Windows support experimental
        println!("cargo:rustc-link-lib=mono-2.0-sgen");
        println!("cargo:rustc-link-search=C:\\ProgramFiles\\Mono\\lib");
        copy_win_dlls();
    }
    #[cfg(test)]
    binds::gen_binds();
   
    #[cfg(test)]
    std::fs::create_dir_all("test/dlls").expect("Could not create test directory");
    #[cfg(test)]
    {
        tests::compile_pinvoke_test_assembly();
        tests::compile_jit_test_assembly();
        tests::compile_test_lib();
    }
}

  
  



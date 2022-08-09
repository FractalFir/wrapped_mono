use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::Write;
fn compile_assembly(src_path:&str,target_path:&str){
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
fn compile_jit_test_assembly(){
    let output = Command::new("mcs") 
    .arg("-out:test/local/Jit.dll")
    .arg("test/Jit.cs")
    .output()
    .expect("Failed to execute command");
    let stderr = output.stderr;
    if stderr.len() > 0{
        panic!("{}",std::str::from_utf8(&stderr).unwrap());
    }
}
fn compile_pinvoke_test_assembly(){
    let output = Command::new("mcs") 
    .arg("-out:test/local/Pinvoke.dll")
    .arg("test/Pinvoke.cs")
    .output()
    .expect("Failed to execute command");
    let stderr = output.stderr;
    if stderr.len() > 0{
        panic!("{}",std::str::from_utf8(&stderr).unwrap());
    }
}
fn compile_test_lib(){
    let output = Command::new("mcs") 
    .arg("-target:library") 
    .arg("-out:test/local/Test.dll")
    .arg("test/Test.cs")
    .output()
    .expect("Failed to execute command");
    let stderr = output.stderr;
    if stderr.len() > 0{
        panic!("{}",std::str::from_utf8(&stderr).unwrap());
    }
}
fn gen_binds(){
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
        b"#![allow(non_upper_case_globals)]\n
        #![allow(non_camel_case_types)]\n
        #![allow(non_snake_case)]\n"
    ).expect("Could not write bindings prefix");

    bindings.write(Box::new(file)).expect("Couldn't write bindings!");
}
fn main() {
    std::fs::create_dir_all("test/local");
    println!("cargo:rustc-link-lib=mono-2.0");
    gen_binds();
    compile_assembly("test/Test.cs","test/local/Pinvoke.dll");
    compile_pinvoke_test_assembly();
    compile_jit_test_assembly();
    compile_test_lib();
}


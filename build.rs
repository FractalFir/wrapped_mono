use std::path::PathBuf;
fn comple_test_mono_lib(){

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
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
fn main() {
    println!("cargo:rustc-link-lib=mono-2.0");
    gen_binds();
    comple_test_mono_lib();
}


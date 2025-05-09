// comment out all #[cfg(test)] and bindgen dependency to renable bind generation
#[cfg(feature = "regen_binds")]
mod binds {
    extern crate bindgen;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    pub fn gen_binds() {
        let hdr_path = "src/cbinds/binds.h";
        let res_path = "src/binds/mod.rs";
        if std::path::Path::new(&res_path).exists() {
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
            #![allow(non_snake_case)]\n
            #![allow(clippy)]",
        )
        .expect("Could not write bindings prefix");
        bindings
            .write(Box::new(file))
            .expect("Couldn't write bindings!");
    }
}
#[cfg(feature = "build_test_dlls")]
mod tests {
    use std::process::Command;
    pub fn compile_jit_test_assembly() {
        let output = Command::new("mcs")
            .arg("-out:test/dlls/Jit.dll")
            .arg("test/Jit.cs")
            .output()
            .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0 {
            panic!("{}", std::str::from_utf8(&stderr).unwrap());
        }
    }
    pub fn compile_pinvoke_test_assembly() {
        let output = Command::new("mcs")
            .arg("-out:test/dlls/Pinvoke.dll")
            .arg("test/Pinvoke.cs")
            .output()
            .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0 {
            panic!("{}", std::str::from_utf8(&stderr).unwrap());
        }
    }
    pub fn compile_test_lib() {
        let output = Command::new("mcs")
            .arg("-target:library")
            .arg("-out:test/dlls/Test.dll")
            .arg("test/Test.cs")
            .output()
            .expect("Failed to execute command");
        let stderr = output.stderr;
        if stderr.len() > 0 {
            panic!("{}", std::str::from_utf8(&stderr).unwrap());
        }
    }
}
fn is_docs() -> bool {
    std::env::var("DOCS_RS").is_ok()
}
//This is a hack. It will not work if not building using default rust toolchain.
#[cfg(target_os = "windows")]
fn target_dir() -> String {
    use std::env;
    use std::path::Path;
    return Path::new(&env::var("OUT_DIR").unwrap())
        .parent()
        .unwrap() //wrapped-mono
        .parent()
        .unwrap() //build
        .parent()
        .unwrap() //debug
        .to_str()
        .unwrap()
        .to_owned();
}
fn main() {
    use std::fmt::Write;
    if is_docs() {
        return;
    }
    match os_specific::check_files_present() {
        Ok(_) => (),
        Err(errors) => {
            let mut res = "files: ".to_owned();
            for error in errors {
                write!(res, "\"{error}\" ").unwrap();
            }
            write!(res,"are missing. Is mono installed propely? It cna be downloaded here: https://www.mono-project.com/download/stable/ ").unwrap();
            //panic!("{}", res);
        }
    }
    os_specific::insert_link_args();
    os_specific::copy_dlls();
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    panic!("Target OS currently not supported");
    #[cfg(feature = "regen_binds")]
    binds::gen_binds();
    #[cfg(feature = "build_test_dlls")]
    std::fs::create_dir_all("test/dlls").expect("Could not create test directory");
    #[cfg(feature = "build_test_dlls")]
    {
        tests::compile_pinvoke_test_assembly();
        tests::compile_jit_test_assembly();
        tests::compile_test_lib();
    }
}
#[cfg(target_os = "linux")]
mod os_specific {
    pub fn check_files_present() -> Result<(), Vec<String>> {
        use std::path::Path;
        let mut errors = Vec::new();
        /*
        if !Path::new("/usr/include/mono-2.0/mono/").exists(){
            errors.push("/usr/include/mono-2.0/mono/".to_owned());
        }
        */
        if !Path::new("/usr/lib/mono/").exists() {
            errors.push("/usr/lib/mono/".to_owned());
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
    pub fn insert_link_args() {
        #[cfg(feature = "mono_lib_fom_env")]{
            /*
            This unsafe is only semantic, but this operation can lead to highly unsafe behavior during
            runtime due to a mismatch in the actual dll in use.
             */
            unsafe {
                if let Ok(env_val) = std::env::var("MONO_LIB_NAME"){
                    println!("cargo:rustc-link-lib={env_val}");
                }else{
                    panic!("The environment variable MONO_LIB_NAME must be set as you are using the feature `mono_lib_fom_env`");
                }
            }
        }
        #[cfg(not(feature = "mono_lib_fom_env"))]
        println!("cargo:rustc-link-lib=mono-2.0");

        println!("cargo:rustc-link-lib=stdc++");
        println!("cargo:rustc-link-lib=z");
    }
    #[allow(clippy::missing_const_for_fn)]
    pub fn copy_dlls() {}
}
#[cfg(target_os = "windows")]
mod os_specific {
    pub fn check_files_present() -> Result<(), Vec<String>> {
        use std::path::Path;
        let mut errors = Vec::new();
        if !Path::new("C:\\Program Files\\Mono\\lib\\mono").exists() {
            errors.push("C:\\Program Files\\Mono\\lib\\mono".to_owned());
        }
        if !Path::new("C:\\Program Files\\Mono\\lib\\mono\\4.8-api").exists() {
            errors.push("C:\\Program Files\\Mono\\lib\\mono\\4.8-api".to_owned());
        }
        if errors.is_empty() {
            return Ok(());
        }
        Err(errors)
    }
    pub fn insert_link_args() {
        println!("cargo:rustc-link-search=C:\\Program Files\\Mono\\lib");

        println!("cargo:rustc-link-lib=libmono-static-sgen");
        println!("cargo:rustc-link-lib=bcrypt");
        println!("cargo:rustc-link-lib=winmm");
        println!("cargo:rustc-link-lib=ole32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=oleaut32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=version");
    }
    pub fn copy_dlls() {
        use std::path::Path;
        let versions = [
            "2.0-api",
            "4.0",
            "4.0-api",
            "4.5",
            "4.5.1-api",
            "4.5.2-api",
            "4.6-api",
            "4.7.1-api",
            "4.7.2-api",
            "4.7-api",
            "4.8-api",
        ];
        let trgt_dir = crate::target_dir();
        for version in versions {
            let spath_str =
                "C:\\Program Files\\Mono\\lib\\mono\\".to_owned() + version + "\\mscorlib.dll";
            let spath = Path::new(&spath_str);
            let tpath_str = trgt_dir.to_owned() + "\\lib\\mono\\" + version + "\\mscorlib.dll";
            let tpath = Path::new(&tpath_str);
            _ = std::fs::create_dir_all(Path::new(
                &(trgt_dir.to_owned() + "\\lib\\mono\\" + version),
            ));
            match std::fs::copy(spath, tpath) {
                Ok(_) => (),
                Err(_) => panic!(
                    "Could not copy mscorlib.dll from \"{}\" to \"{}\". Is mono installed propely?",
                    spath_str, tpath_str
                ),
            }
        }
    }
}

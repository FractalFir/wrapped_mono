use crate::Assembly;
use crate::metadata::TypeDefinition;
use std::path::Path;
#[derive(Debug)]
pub enum BindgenError{
    IoError(std::io::Error),
    MissingTypeData,
}
impl From<std::io::Error> for BindgenError{
    fn from(err:std::io::Error)->Self{
        Self::IoError(err)
    }
}
pub fn generate_binds<P:AsRef<Path>>(path:P,assembly:Assembly)->Result<(),BindgenError>{
    use crate::metadata::TypeDefinitionTable;
    // Prepare
    std::fs::create_dir_all(path.as_ref())?;
    for entry in std::fs::read_dir(path.as_ref())?{
        std::fs::remove_file(entry?.path())?
    }
    // Generate
    let asm_name = assembly.get_name();
    let img = assembly.get_image();
    let tdt = TypeDefinitionTable::from_image(img).ok_or(BindgenError::MissingTypeData)?;
    for td in tdt.defs(){
        emmit_bind_for_tdt(path.as_ref(),td,&asm_name)?
    }
    Ok(())
}
fn emmit_bind_for_tdt(path:&Path,td:&TypeDefinition,asm_name:&str)->Result<(),BindgenError> {
    use std::io::Write;
    //Skip types with invalid names.
    if td.name().contains('<'){
        return Ok(());
    }
    let mut bind_path = path.to_path_buf();
    let mut namespace = if td.namespace().is_empty(){
        "mod"
    }
    else{
        td.namespace()
    };
    let namespace = namespace.to_owned() + ".rs";
    bind_path.push(namespace);
    let bind_path = bind_path;
    let mut out = std::fs::File::options().create(true).append(true).write(true).open(bind_path)?;
    write!(out,"// Bindings to object \"{name}\" in namespace \"{namespace}\".\n",name = td.name(), namespace = td.namespace())?;
    let name = td.name();
    let namespace = td.namespace();
    write!(out,"impl wrapped_mono::InteropClass for {name}{{
    fn get_mono_class()->wrapped_mono::Class{{
        extern crate lazy_static;
        use lazy_static::*;
        lazy_static!{{
            #[allow(non_upper_case_globals)]
            static ref {name}_CLASS:wrapped_mono::Class = {{
                 let img = Assembly::assembly_loaded(\"{asm_name}\")
                    .expect(\"Assembly \\\"{asm_name}\\\" is not loaded, could not get \\\"{name}\\\" class!\")
                    .get_image();
                Class::from_name_case(&img, \"{namespace}\", \"{name}\")
                    .expect(\"Could not get \\\"{name}\\\" class residing in namespace \\\"{namespace}\\\" form assembly \\\"{asm_name}\\\"!\")
            }};
        }}
        *{name}_CLASS
    }}
}}\n")?;
    write!(out,"struct {name}{{
    object:wrapped_mono::Object,
}}\n")?;
    write!(out,"impl wrapped_mono::ObjectTrait for {name}{{
    fn get_ptr(&self)->*mut wrapped_mono::binds::MonoObject{{
        self.object.get_ptr()
    }}
    unsafe fn from_ptr_unchecked(ptr:*mut wrapped_mono::binds::MonoObject)->Self{{
        let object = wrapped_mono::Object::from_ptr_unchecked(ptr);
        Self{{object}}
    }}
}}\n")?;
    Ok(())
}

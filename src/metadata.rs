use crate::binds::MonoTableInfo;
use crate::Image;
///Representaiton of a table with metadata.
pub struct MetadataTableInfo{
    pub table:*const MonoTableInfo,
    pub kind:MetadataTableKind,
}
pub type MetadataToken = u32;
///Enum representing all possible kinds of metadata tables.
#[repr(u32)] #[derive(PartialEq,Eq)] #[derive(Clone,Copy)]
pub enum MetadataTableKind{
    Module              =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULE,
    TypeRef             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEREF,
    TypeDef             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEDEF,
    FieldPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD_POINTER,
    Field               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD,
    MethodPointer       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD_POINTER,
    Method              =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD,
    ParamPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM_POINTER,
    Param               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM,
    InerfaceImpl        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_INTERFACEIMPL,
    MemberRef           =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MEMBERREF,
    Constant            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CONSTANT,
    CustomAttribute     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CUSTOMATTRIBUTE,
    FieldMarshal        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDMARSHAL,
    DeclSceurity        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_DECLSECURITY,
    ClassLayout         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_CLASSLAYOUT,
    FieldLatout         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDLAYOUT,
    StandaloneSig       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_STANDALONESIG,
    EventMap            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENTMAP,
    EventPointer        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT_POINTER,
    Event               =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT,
    PropertyMap         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTYMAP,
    PropertyPointer     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY_POINTER,
    Property            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY,
    MethodSemantics     =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSEMANTICS,
    MethodImpl          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODIMPL,
    ModuleRef           =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULEREF,
    TypeSpec            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPESPEC,
    ImplMap             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPLMAP,
    FieldRVA            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDRVA,
    Unused6             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED6,
    Unused7             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED7,
    Assembly            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLY,
    AssemblyProcessor   =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYPROCESSOR,
    AssemblyOS          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYOS,
    AssmeblyRef         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREF,
    AssmeblyRefProcessor=   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFPROCESSOR,
    AssmeblyRefOS       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFOS,
    File                =   crate::binds::MonoMetaTableEnum_MONO_TABLE_FILE,
    ExportedType        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_EXPORTEDTYPE,
    ManifestResource    =   crate::binds::MonoMetaTableEnum_MONO_TABLE_MANIFESTRESOURCE,
    NestedClass         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_NESTEDCLASS,
    GenericParam        =   crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAM,
    MethodSpec          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSPEC,
    GenericParamConstraint= crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAMCONSTRAINT,
    Unused8             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED8,
    Unused9             =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED9,
    Unused10            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED10,
    Document            =   crate::binds::MonoMetaTableEnum_MONO_TABLE_DOCUMENT,
    MethodBody          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODBODY,
    LocalScope          =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALSCOPE,
    LocalVariable       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALVARIABLE,
    LocalConstant       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALCONSTANT,
    ImportScope         =   crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPORTSCOPE,
    MachineMethod       =   crate::binds::MonoMetaTableEnum_MONO_TABLE_STATEMACHINEMETHOD,
}
impl MetadataTableInfo{
    /// Creates [`MetadataTableInfo`] from a [`MonoTableInfo`] pointer.
    /// # Safety
    /// *table* must be a valid [`MonoTableInfo`] pointer, and must match kind.
    pub unsafe fn from_ptr(table:*const MonoTableInfo,kind:MetadataTableKind)->MetadataTableInfo{
        Self{table,kind}
    }
    ///Get ammout of rows in a table.
    pub fn get_table_rows(&self)->i32{
        unsafe{crate::binds::mono_table_info_get_rows(self.table)}
    }
    ///Gets the token at *column* in *row*
    pub fn decode_row_col(&self,row:i32,column:u32)->MetadataToken{
        unsafe{crate::binds::mono_metadata_decode_row_col(self.table,row,column)}
    }
}
///Representaion of data about assembly.
pub struct AssemblyMetadata{
    pub hash_alg:HashAlgorithm,
    pub major_version:u32,
    pub minor_version:u32,
    pub build_number:u32,
    pub rev_number:u32,
    pub flags:AssemblyFlags,
    pub public_key:u32,
    name:String,
    culture:String,
}
impl AssemblyMetadata{
    fn from_meta_table(table:&MetadataTableInfo,img:&Image)->AssemblyMetadata{
        assert!(table.kind == MetadataTableKind::Assembly);
        AssemblyMetadata{
            hash_alg:       HashAlgorithm::from_u32(table.decode_row_col(0,0)),
            major_version:  table.decode_row_col(0,1),
            minor_version:  table.decode_row_col(0,2),
            build_number:   table.decode_row_col(0,3),
            rev_number:     table.decode_row_col(0,4),
            flags:          AssemblyFlags{flags:table.decode_row_col(0,5)},
            public_key:     table.decode_row_col(0,6),
            name:           img.metadata_string_heap(table.decode_row_col(0,7)),
            culture:        img.metadata_string_heap(table.decode_row_col(0,8)),
        }
    }
    ///Gets [`AssemblyMetadata`]
    pub fn from_image(img:&Image)->AssemblyMetadata{
        Self::from_meta_table(&img.get_table_info(MetadataTableKind::Assembly),img)
    }
    //Returns name string.
    pub fn get_name(&self)->String{
        self.name.to_owned()
    }
    ///Returns cultutre string.
    pub fn get_culture(&self)->String{
        self.culture.to_owned()
    }
}
///Representation of assembly flags. More info <a href="https://docs.microsoft.com/en-us/dotnet/api/system.reflection.assemblyflags?view=net-6.0"> here </a>
pub struct AssemblyFlags{pub flags:u32}
#[allow(non_snake_case)]
impl AssemblyFlags{
    ///Checks is `WindowsRuntime` flag is set.
    pub fn is_set_WindowsRuntime(&self)->bool{
        (self.flags & 512) != 0
    }
    ///Checks is `Retargtable` flag is set.
    pub fn is_set_Retargtable(&self)->bool{
        (self.flags & 256) != 0
    }
    ///Checks if `PublicKey` flag is set.
    pub fn is_set_PublicKey(&self)->bool{
        (self.flags & 1) != 0
    }
    ///Checks if `DisableJitCompileOptimizer` flag is set.
    pub fn is_set_DisableJitCompileOptimizer(&self)->bool{
        (self.flags & 16384) != 0
    }
    ///Checks if `EnableJitCompileTracking` flag is set.
    pub fn is_set_EnableJitCompileTracking(&self)->bool{
        (self.flags & 32768) != 0
    }
    ///Returns the `ContentType` mask bits.
    pub fn content_type_mask(&self)->[bool;2]{
        [(self.flags & 2048) != 0,(self.flags & 1024) != 0]
    }
}
#[warn(non_snake_case)]
impl std::fmt::Display for AssemblyFlags{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"AssemblyFlags{{WindowsRuntime:{}, ",self.is_set_WindowsRuntime())?;
        write!(f,"Retargtable:{}, ",                  self.is_set_Retargtable())?;
        write!(f,"PublicKey:{}, ",                    self.is_set_PublicKey())?;
        write!(f,"DisableJitCompileOptimizer:{}, ",   self.is_set_DisableJitCompileOptimizer())?;
        write!(f,"EnableJitCompileTracking:{}, ",     self.is_set_EnableJitCompileTracking())?;
        write!(f,"ContentTypeMask:{:?}}} ",           self.content_type_mask())
    }
}
///Assembly hash algotith type. More info <a href="docs.microsoft.com/en-us/dotnet/api/system.configuration.assemblies.assemblyhashalgorithm?view=net-6.0"> here </a>
#[repr(u32)]
pub enum HashAlgorithm{
    MD5     =   32771,
    None    =   0,
    SHA1    =   32772,
    SHA256  =   32780,
    SHA384  =   32781,
    SHA512  =   32782,
}
impl HashAlgorithm{
    fn from_u32(u:u32)->HashAlgorithm{
        match u{
            32771   =>HashAlgorithm::MD5,
            0       =>HashAlgorithm::None,
            32772   =>HashAlgorithm::SHA1,
            32780   =>HashAlgorithm::SHA256,
            32781   =>HashAlgorithm::SHA384,
            32782   =>HashAlgorithm::SHA256,
            _=>panic!("{} is not a valid HashAlgorithm",u),
        }
    }
}
impl std::fmt::Display for HashAlgorithm{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        let s = match self{
            HashAlgorithm::MD5=>    "MD5",
            HashAlgorithm::None=>   "None",
            HashAlgorithm::SHA1=>   "SHA1",
            HashAlgorithm::SHA256=> "SHA256",
            HashAlgorithm::SHA384=> "SHA386",
            HashAlgorithm::SHA512=> "SHA512",
        };
        write!(f,"{}",s)
    }
}
impl std::fmt::Display for AssemblyMetadata{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"AssemblyMetadata{{")?;
        write!(f,"hash_alg:{}, ",        self.hash_alg)?;
        write!(f,"major_version:{}, ",   self.major_version)?;
        write!(f,"minor_version:{}, ",   self.minor_version)?;
        write!(f,"build_number:{}, ",    self.build_number)?;
        write!(f,"rev_number:{}, ",      self.rev_number)?;
        write!(f,"flags:{}, ",           self.flags)?;
        write!(f,"public_key:{}, ",      self.public_key)?;
        write!(f,"Name:{}, ",            &self.name)?;
        write!(f,"Culture:{}}}",         &self.culture)
    }
}
pub struct AssemblyOSMetadata{
    platform: String,
    major_version:u32,
    minor_version:u32,
}
impl AssemblyOSMetadata{
    fn from_meta_table(table:&MetadataTableInfo,img:&Image)->AssemblyOSMetadata{
        assert!(table.kind == MetadataTableKind::AssemblyOS);
        AssemblyOSMetadata{
            platform:img.metadata_string_heap(table.decode_row_col(0,0)),
            major_version:table.decode_row_col(0,1),
            minor_version:table.decode_row_col(0,2),
        }
    }
    ///Gets [`AssemblyMetadata`]
    pub fn from_image(img:&Image)->Option<AssemblyOSMetadata>{
        let table = img.get_table_info(MetadataTableKind::AssemblyOS);
        if table.get_table_rows()>0{
            Some(Self::from_meta_table(&table,img))
        }
        else{None}
    }
    //Returns platform string.
    pub fn get_platform(&self)->String{
        self.platform.to_owned()
    }
}
impl std::fmt::Display for AssemblyOSMetadata{
    fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
        write!(f,"AssemblyOSMetadata{{")?;
        write!(f,"Platform:{}, ",       &self.platform)?;
        write!(f,"MajorVersion:{}, ",   self.major_version)?;
        write!(f,"MinorVersion:{}}}",   self.minor_version)
    }
}

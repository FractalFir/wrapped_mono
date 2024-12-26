use crate::binds::MonoTableInfo;
use crate::Image;
///Representation of a table with metadata.
pub struct MetadataTableInfo {
    pub table: *const MonoTableInfo,
    pub kind: MetadataTableKind,
}
pub type MetadataToken = u32;
///Enum representing all possible kinds of metadata tables.
#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MetadataTableKind {
    Module = crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULE,
    TypeRef = crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEREF,
    TypeDef = crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPEDEF,
    FieldPointer = crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD_POINTER,
    Field = crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELD,
    MethodPointer = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD_POINTER,
    Method = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHOD,
    ParamPointer = crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM_POINTER,
    Param = crate::binds::MonoMetaTableEnum_MONO_TABLE_PARAM,
    InerfaceImpl = crate::binds::MonoMetaTableEnum_MONO_TABLE_INTERFACEIMPL,
    MemberRef = crate::binds::MonoMetaTableEnum_MONO_TABLE_MEMBERREF,
    Constant = crate::binds::MonoMetaTableEnum_MONO_TABLE_CONSTANT,
    CustomAttribute = crate::binds::MonoMetaTableEnum_MONO_TABLE_CUSTOMATTRIBUTE,
    FieldMarshal = crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDMARSHAL,
    DeclSceurity = crate::binds::MonoMetaTableEnum_MONO_TABLE_DECLSECURITY,
    ClassLayout = crate::binds::MonoMetaTableEnum_MONO_TABLE_CLASSLAYOUT,
    FieldLatout = crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDLAYOUT,
    StandaloneSig = crate::binds::MonoMetaTableEnum_MONO_TABLE_STANDALONESIG,
    EventMap = crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENTMAP,
    EventPointer = crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT_POINTER,
    Event = crate::binds::MonoMetaTableEnum_MONO_TABLE_EVENT,
    PropertyMap = crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTYMAP,
    PropertyPointer = crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY_POINTER,
    Property = crate::binds::MonoMetaTableEnum_MONO_TABLE_PROPERTY,
    MethodSemantics = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSEMANTICS,
    MethodImpl = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODIMPL,
    ModuleRef = crate::binds::MonoMetaTableEnum_MONO_TABLE_MODULEREF,
    TypeSpec = crate::binds::MonoMetaTableEnum_MONO_TABLE_TYPESPEC,
    ImplMap = crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPLMAP,
    FieldRVA = crate::binds::MonoMetaTableEnum_MONO_TABLE_FIELDRVA,
    Unused6 = crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED6,
    Unused7 = crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED7,
    Assembly = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLY,
    AssemblyProcessor = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYPROCESSOR,
    AssemblyOS = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYOS,
    AssmeblyRef = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREF,
    AssmeblyRefProcessor = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFPROCESSOR,
    AssmeblyRefOS = crate::binds::MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFOS,
    File = crate::binds::MonoMetaTableEnum_MONO_TABLE_FILE,
    ExportedType = crate::binds::MonoMetaTableEnum_MONO_TABLE_EXPORTEDTYPE,
    ManifestResource = crate::binds::MonoMetaTableEnum_MONO_TABLE_MANIFESTRESOURCE,
    NestedClass = crate::binds::MonoMetaTableEnum_MONO_TABLE_NESTEDCLASS,
    GenericParam = crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAM,
    MethodSpec = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODSPEC,
    GenericParamConstraint = crate::binds::MonoMetaTableEnum_MONO_TABLE_GENERICPARAMCONSTRAINT,
    Unused8 = crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED8,
    Unused9 = crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED9,
    Unused10 = crate::binds::MonoMetaTableEnum_MONO_TABLE_UNUSED10,
    Document = crate::binds::MonoMetaTableEnum_MONO_TABLE_DOCUMENT,
    MethodBody = crate::binds::MonoMetaTableEnum_MONO_TABLE_METHODBODY,
    LocalScope = crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALSCOPE,
    LocalVariable = crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALVARIABLE,
    LocalConstant = crate::binds::MonoMetaTableEnum_MONO_TABLE_LOCALCONSTANT,
    ImportScope = crate::binds::MonoMetaTableEnum_MONO_TABLE_IMPORTSCOPE,
    MachineMethod = crate::binds::MonoMetaTableEnum_MONO_TABLE_STATEMACHINEMETHOD,
}
impl MetadataTableInfo {
    /// Creates [`MetadataTableInfo`] from a [`MonoTableInfo`] pointer.
    /// # Safety
    /// *table* must be a valid [`MonoTableInfo`] pointer, and must match kind.
    #[must_use]
    pub unsafe fn from_ptr(table: *const MonoTableInfo, kind: MetadataTableKind) -> Self {
        Self { table, kind }
    }
    ///Get amount of rows in a table.
    #[must_use]
    pub fn get_table_rows(&self) -> i32 {
        unsafe { crate::binds::mono_table_info_get_rows(self.table) }
    }
    ///Gets the token at *`column`* in *`row`*
    #[must_use]
    pub fn decode_row_col(&self, row: i32, column: u32) -> MetadataToken {
        unsafe { crate::binds::mono_metadata_decode_row_col(self.table, row, column) }
    }
}
///Representation of data about assembly.
pub struct AssemblyMetadata {
    pub hash_alg: HashAlgorithm,
    pub major_version: u32,
    pub minor_version: u32,
    pub build_number: u32,
    pub rev_number: u32,
    pub flags: AssemblyFlags,
    pub public_key: u32,
    name: String,
    culture: String,
}
impl AssemblyMetadata {
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        assert!(table.kind == MetadataTableKind::Assembly);
        Self {
            hash_alg: HashAlgorithm::from_u32(table.decode_row_col(0, 0)),
            major_version: table.decode_row_col(0, 1),
            minor_version: table.decode_row_col(0, 2),
            build_number: table.decode_row_col(0, 3),
            rev_number: table.decode_row_col(0, 4),
            flags: AssemblyFlags {
                flags: table.decode_row_col(0, 5),
            },
            public_key: table.decode_row_col(0, 6),
            name: img.metadata_string_heap(table.decode_row_col(0, 7)),
            culture: img.metadata_string_heap(table.decode_row_col(0, 8)),
        }
    }
    ///Gets [`AssemblyMetadata`] from an [`Image`]
    #[must_use]
    pub fn from_image(img: Image) -> Self {
        Self::from_meta_table(&img.get_table_info(MetadataTableKind::Assembly), img)
    }
    //Returns name string.
    #[must_use]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    ///Returns culture string.
    #[must_use]
    pub fn get_culture(&self) -> String {
        self.culture.clone()
    }
}
///Representation of assembly flags. More info <a href="https://docs.microsoft.com/en-us/dotnet/api/system.reflection.assemblyflags?view=net-6.0"> here </a>
pub struct AssemblyFlags {
    pub flags: u32,
}
#[allow(non_snake_case)]
impl AssemblyFlags {
    ///Checks is `WindowsRuntime` flag is set.
    #[must_use]
    pub fn is_set_WindowsRuntime(&self) -> bool {
        (self.flags & 512) != 0
    }
    ///Checks is `Retargtable` flag is set.
    #[must_use]
    pub fn is_set_Retargtable(&self) -> bool {
        (self.flags & 256) != 0
    }
    ///Checks if `PublicKey` flag is set.
    #[must_use]
    pub fn is_set_PublicKey(&self) -> bool {
        (self.flags & 1) != 0
    }
    ///Checks if `DisableJitCompileOptimizer` flag is set.
    #[must_use]
    pub fn is_set_DisableJitCompileOptimizer(&self) -> bool {
        (self.flags & 16384) != 0
    }
    ///Checks if `EnableJitCompileTracking` flag is set.
    #[must_use]
    pub fn is_set_EnableJitCompileTracking(&self) -> bool {
        (self.flags & 32768) != 0
    }
    ///Returns the `ContentType` mask bits.
    #[must_use]
    pub fn content_type_mask(&self) -> [bool; 2] {
        [(self.flags & 2048) != 0, (self.flags & 1024) != 0]
    }
}
#[warn(non_snake_case)]
impl std::fmt::Display for AssemblyFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AssemblyFlags{{WindowsRuntime:{}, ",
            self.is_set_WindowsRuntime()
        )?;
        write!(f, "Retargtable:{}, ", self.is_set_Retargtable())?;
        write!(f, "PublicKey:{}, ", self.is_set_PublicKey())?;
        write!(
            f,
            "DisableJitCompileOptimizer:{}, ",
            self.is_set_DisableJitCompileOptimizer()
        )?;
        write!(
            f,
            "EnableJitCompileTracking:{}, ",
            self.is_set_EnableJitCompileTracking()
        )?;
        write!(f, "ContentTypeMask:{:?}}} ", self.content_type_mask())
    }
}
///Assembly hash algorithm type. More info <a href="docs.microsoft.com/en-us/dotnet/api/system.configuration.assemblies.assemblyhashalgorithm?view=net-6.0"> here </a>
#[repr(u32)]
pub enum HashAlgorithm {
    MD5 = 32771,
    None = 0,
    SHA1 = 32772,
    SHA256 = 32780,
    SHA384 = 32781,
    SHA512 = 32782,
}
impl HashAlgorithm {
    fn from_u32(u: u32) -> Self {
        match u {
            32771 => Self::MD5,
            0 => Self::None,
            32772 => Self::SHA1,
            32780 => Self::SHA256,
            32781 => Self::SHA384,
            32782 => Self::SHA512,
            _ => panic!("{u} is not a valid HashAlgorithm"),
        }
    }
}
impl std::fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::MD5 => "MD5",
            Self::None => "None",
            Self::SHA1 => "SHA1",
            Self::SHA256 => "SHA256",
            Self::SHA384 => "SHA386",
            Self::SHA512 => "SHA512",
        };
        write!(f, "{s}")
    }
}
impl std::fmt::Display for AssemblyMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssemblyMetadata{{")?;
        write!(f, "hash_alg:{}, ", self.hash_alg)?;
        write!(f, "major_version:{}, ", self.major_version)?;
        write!(f, "minor_version:{}, ", self.minor_version)?;
        write!(f, "build_number:{}, ", self.build_number)?;
        write!(f, "rev_number:{}, ", self.rev_number)?;
        write!(f, "flags:{}, ", self.flags)?;
        write!(f, "public_key:{}, ", self.public_key)?;
        write!(f, "Name:{}, ", &self.name)?;
        write!(f, "Culture:{}}}", &self.culture)
    }
}
pub struct AssemblyOSMetadata {
    platform: String,
    major_version: u32,
    minor_version: u32,
}
impl AssemblyOSMetadata {
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        assert!(table.kind == MetadataTableKind::AssemblyOS);
        Self {
            platform: img.metadata_string_heap(table.decode_row_col(0, 0)),
            major_version: table.decode_row_col(0, 1),
            minor_version: table.decode_row_col(0, 2),
        }
    }
    ///Gets [`AssemblyMetadata`]
    #[must_use]
    pub fn from_image(img: Image) -> Option<Self> {
        let table = img.get_table_info(MetadataTableKind::AssemblyOS);
        if table.get_table_rows() > 0 {
            Some(Self::from_meta_table(&table, img))
        } else {
            None
        }
    }
    //Returns platform string.
    #[must_use]
    pub fn get_platform(&self) -> String {
        self.platform.clone()
    }
}
impl std::fmt::Display for AssemblyOSMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AssemblyOSMetadata{{")?;
        write!(f, "Platform:{}, ", &self.platform)?;
        write!(f, "MajorVersion:{}, ", self.major_version)?;
        write!(f, "MinorVersion:{}}}", self.minor_version)
    }
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CallingConventionType {
    Default,
    Generic,
    VArg,
}
#[derive(Copy, Clone)]
pub struct SignatureFlags {
    flags: u8,
}
impl SignatureFlags {
    fn new(flags: u8) -> Self {
        Self { flags }
    }
    pub fn has_this(&self) -> bool {
        (self.flags & 0x20) != 0
    }
    pub fn explicit_this(&self) -> bool {
        (self.flags & 0x40) != 0
    }
    pub fn callconv_type(&self) -> CallingConventionType {
        if self.flags & 0x10 != 0 {
            CallingConventionType::Generic
        } else if self.flags & 0x5 != 0 {
            CallingConventionType::VArg
        } else {
            CallingConventionType::Default
        }
    }
}
/*Ecma International 2012, C# spec
For unsigned integers:
o If the value lies between 0 (0x00) and 127 (0x7F), inclusive, encode as a
one-byte integer (bit 7 is clear, value held in bits 6 through 0)
o If the value lies between 28 (0x80) and 214 – 1 (0x3FFF), inclusive,
encode as a 2-byte integer with bit 15 set, bit 14 clear (value held in
bits 13 through 0)
o Otherwise, encode as a 4-byte integer, with bit 31 set, bit 30 set, bit 29
clear (value held in bits 28 through 0)
 For signed integers:
o If the value lies between -26 and 26-1 inclusive:
o Represent the value as a 7-bit 2’s complement number, giving 0x40
(-26) to 0x3F (26-1);
o Rotate this value 1 bit left, giving 0x01 (-26) to 0x7E (26-1);
o Encode as a one-byte integer, bit 7 clear, rotated value in bits 6
through 0, giving 0x01 (-26) to 0x7E (26-1).
o If the value lies between -213 and 213-1 inclusive:
258 © Ecma International 2012
o Represent the value as a 14-bit 2’s complement number, giving
0x2000 (-213) to 0x1FFF (213-1);
o Rotate this value 1 bit left, giving 0x0001 (-213) to 0x3FFE (213-1);
o Encode as a two-byte integer: bit 15 set, bit 14 clear, rotated value
in bits 13 through 0, giving 0x8001 (-213) to 0xBFFE (213-1).
o If the value lies between -228 and 228-1 inclusive:
o Represent the value as a 29-bit 2’s complement representation,
giving 0x10000000 (-228) to 0xFFFFFFF (228-1);
o Rotate this value 1-bit left, giving 0x00000001 (-228) to
0x1FFFFFFE (228-1);
o Encode as a four-byte integer: bit 31 set, 30 set, bit 29 clear, rotated
value in bits 28 through 0, giving 0xC0000001 (-228) to
0xDFFFFFFE (228-1)
*/
#[test]
fn test_decode() {
    assert_eq!(0x03, blob_decode_u32(&mut &[0x03][..]));
    println!("0x03");
    assert_eq!(0x7F, blob_decode_u32(&mut &[0x7F][..]));
    println!("0x7F");
    assert_eq!(0x80, blob_decode_u32(&mut &[0x80, 0x80][..]));
    println!("0x80");
    assert_eq!(0x2E57, blob_decode_u32(&mut &[0xAE, 0x57][..]));
    println!("0x2E57");
    assert_eq!(0x3FFF, blob_decode_u32(&mut &[0xBF, 0xFF][..]));
}
fn blob_decode_u32(src: &mut &[u8]) -> u32 {
    if (*src)[0] & 0x80 == 0 {
        let res = (*src)[0];
        (*src) = &(*src)[1..];
        res as u32
    } else if (*src)[0] & 0x80 != 0 && (*src)[0] & 0x40 == 0 {
        let res = ((((*src)[0] & 0x7F) as u32) << 8) + (*src)[1] as u32;
        (*src) = &(*src)[2..];
        res as u32
    } else {
        panic!(
            "Can't decode integers bigger than 16383 from blob heap yet, {}!",
            src[0]
        );
    }
}
impl std::fmt::Debug for SignatureFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SignatureFlags{{")?;
        write!(f, "has_this:{has_this},", has_this = self.has_this())?;
        write!(
            f,
            "explicit_this:{explicit_this},",
            explicit_this = self.explicit_this()
        )?;
        write!(
            f,
            "callconv_type:{callconv_type:?}",
            callconv_type = self.callconv_type()
        )?;
        write!(f, "}}")
    }
}
#[derive(Debug, Copy, Clone)]
pub enum TypeDefOrRef {
    TypeDef(u32),
    TypeRef(u32),
    TypeSpec(u32),
}
impl TypeDefOrRef {
    pub fn is_null(&self) -> bool {
        match self {
            Self::TypeDef(index) => *index == 0,
            //TODO: check how null is represented for TypeRef and TypeSpec, if it is represented at all.
            Self::TypeRef(index) => *index == 0,
            Self::TypeSpec(_) => false,
        }
    }
    fn decode(src: &mut &[u8]) -> Option<Self> {
        assert!(src.len() > 0);
        let decoded = blob_decode_u32(src);
        Self::new(decoded)
    }
    fn new(src: u32) -> Option<Self> {
        let type_kind = src & 0b11;
        let type_index = src & !(0b11);
        match type_kind {
            0 => Some(Self::TypeDef(type_kind)),
            1 => Some(Self::TypeRef(type_kind)),
            2 => Some(Self::TypeSpec(type_kind)),
            3 => None,
            _ => panic!("Decode error: type kind must be either 0,1 or 2 but got {type_kind}."),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Signature {
    flags: SignatureFlags,
    signature: Box<[u8]>,
    params: Box<[TypeDefOrRef]>,
    ret: TypeDefOrRef,
}
#[derive(Debug)]
enum SignatureDecodeError {
    UnsuportedGeneric,
    InvalidTypedef,
}
impl Signature {
    pub fn params(&self) -> &[TypeDefOrRef] {
        &self.params
    }
    pub fn flags(&self) -> SignatureFlags {
        self.flags
    }
    pub fn ret(&self) -> TypeDefOrRef {
        self.ret
    }
    fn new(mut signature: &[u8]) -> Result<Self, SignatureDecodeError> {
        let flags = SignatureFlags::new(signature[0]);
        signature = &signature[1..];
        if flags.callconv_type() == CallingConventionType::Generic {
            //TODO:support generic paramters
            /*
            let _generics =
                i32::from_le_bytes(signature[0..std::mem::size_of::<i32>()].try_into().unwrap());
            signature = &signature[std::mem::size_of::<i32>()..];*/
            return Err(SignatureDecodeError::UnsuportedGeneric);
        }

        let param_count = blob_decode_u32(&mut signature);
        let mut params = Vec::with_capacity(param_count as usize);
        //println!("param_count:{param_count}");
        assert!(signature.len() > 1 || param_count == 0);
        for _ in 0..param_count {
            let stype =
                TypeDefOrRef::decode(&mut signature).ok_or(SignatureDecodeError::InvalidTypedef)?;
            //println!("stype:{stype:?}");
            params.push(stype);
        }

        let ret =
            TypeDefOrRef::decode(&mut signature).ok_or(SignatureDecodeError::InvalidTypedef)?;
        let signature = signature.into();
        let params = params.into();
        Ok(Self {
            signature,
            flags,
            params,
            ret,
        })
    }
}
#[derive(Debug, Clone)]
pub struct Method {
    rva: u32,
    impl_flags: u32,
    flags: u32,
    name: String,
    signature: Signature,
    paramlist: u32,
}
impl Method {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}
#[derive(Debug)]
pub struct MethodTable {
    methods: Box<[Method]>,
}
impl MethodTable {
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        assert_eq!(table.kind, MetadataTableKind::Method);
        let method_count = table.get_table_rows();
        let mut methods = Vec::with_capacity(method_count as usize);
        for index in 0..method_count {
            let rva = table.decode_row_col(index, crate::binds::MONO_METHOD_RVA);
            let impl_flags = table.decode_row_col(index, crate::binds::MONO_METHOD_IMPLFLAGS);
            let flags = table.decode_row_col(index, crate::binds::MONO_METHOD_FLAGS);
            let name = table.decode_row_col(index, crate::binds::MONO_METHOD_NAME);
            let name = img.metadata_string_heap(name);
            let signature = table.decode_row_col(index, crate::binds::MONO_METHOD_SIGNATURE);
            let signature = Signature::new(img.blob_heap(signature));
            let signature = match signature {
                Ok(signature) => signature,
                Err(err) => continue,
            };
            let paramlist = table.decode_row_col(index, crate::binds::MONO_METHOD_PARAMLIST);
            methods.push(Method {
                rva,
                impl_flags,
                flags,
                name,
                signature,
                paramlist,
            });
        }
        let methods = methods.into();
        MethodTable { methods }
    }
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }
    ///Gets [`MethodTable`]
    #[must_use]
    pub fn from_image(img: Image) -> Option<Self> {
        let table = img.get_table_info(MetadataTableKind::Method);
        if table.get_table_rows() > 0 {
            Some(Self::from_meta_table(&table, img))
        } else {
            None
        }
    }
}
#[derive(Debug)]
struct TypeFlags {
    flags: u32,
}
impl TypeFlags {
    fn new(flags: u32) -> Self {
        TypeFlags { flags }
    }
}
#[derive(Debug)]
pub struct TypeDefinition {
    flags: TypeFlags,
    name: String,
    namespace: String,
    extends: TypeDefOrRef,
    field_list: u32,
    methods: Box<[Method]>,
}
impl TypeDefinition {
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn methods(&self) -> &[Method] {
        &self.methods
    }
    pub fn extends(&self) -> TypeDefOrRef {
        self.extends
    }
}
#[derive(Debug)]
pub struct TypeReference {
    scope: u32,
    name: String,
    namespace: String,
}
impl TypeReference {
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
#[derive(Debug)]
pub struct TypeReferenceTable {
    refs: Box<[TypeReference]>,
}
impl TypeReferenceTable {
    pub fn empty() -> Self {
        Self { refs: Box::new([]) }
    }
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        let ref_count = table.get_table_rows();
        let mut refs = Vec::with_capacity(ref_count as usize);
        for index in 0..ref_count {
            let scope = table.decode_row_col(index, crate::binds::MONO_TYPEREF_SCOPE);
            let name = table.decode_row_col(index, crate::binds::MONO_TYPEREF_NAME);
            let name = img.metadata_string_heap(name);
            let namespace = table.decode_row_col(index, crate::binds::MONO_TYPEREF_NAMESPACE);
            let namespace = img.metadata_string_heap(namespace);
            let reference = TypeReference {
                scope,
                name,
                namespace,
            };
            refs.push(reference);
        }
        let refs = refs.into();
        Self { refs }
    }
    pub fn refs(&self) -> &[TypeReference] {
        &self.refs
    }
    ///Gets [`TypeReferenceTable`]
    #[must_use]
    pub fn from_image(img: Image) -> Option<Self> {
        let table = img.get_table_info(MetadataTableKind::TypeRef);
        if table.get_table_rows() > 0 {
            Some(Self::from_meta_table(&table, img))
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct TypeSpec {}
#[derive(Debug)]
pub struct TypeSpecTable {
    specs: Box<[TypeSpec]>,
}
impl TypeSpecTable {
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        let ref_count = table.get_table_rows();
        let mut specs = Vec::with_capacity(ref_count as usize);
        for index in 0..ref_count {
            let spec = TypeSpec {};
            specs.push(spec);
        }
        let specs = specs.into();
        Self { specs }
    }
    pub fn specs(&self) -> &[TypeSpec] {
        &self.specs
    }
    ///Gets [`TypeReferenceTable`]
    #[must_use]
    pub fn from_image(img: Image) -> Option<Self> {
        let table = img.get_table_info(MetadataTableKind::TypeSpec);
        if table.get_table_rows() > 0 {
            Some(Self::from_meta_table(&table, img))
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct TypeDefinitionTable {
    defs: Box<[TypeDefinition]>,
}
impl TypeDefinitionTable {
    #[must_use]
    fn from_meta_table(table: &MetadataTableInfo, img: Image) -> Self {
        assert_eq!(table.kind, MetadataTableKind::TypeDef);
        let methods = MethodTable::from_image(img).unwrap();
        let type_count = table.get_table_rows();
        let mut defs = Vec::with_capacity(type_count as usize);
        for index in 0..type_count {
            let flags = table.decode_row_col(index, crate::binds::MONO_TYPEDEF_FLAGS);
            let name = table.decode_row_col(index, crate::binds::MONO_TYPEDEF_NAME);
            let namespace = table.decode_row_col(index, crate::binds::MONO_TYPEDEF_NAMESPACE);
            let extends = table.decode_row_col(index, crate::binds::MONO_TYPEDEF_EXTENDS);
            let extends = TypeDefOrRef::new(extends).unwrap();
            let field_list = table.decode_row_col(index, crate::binds::MONO_TYPEDEF_FIELD_LIST);
            let method_list =
                table.decode_row_col(index, crate::binds::MONO_TYPEDEF_METHOD_LIST) as usize - 1;
            let method_list_end = if index < type_count - 1 {
                table.decode_row_col(index + 1, crate::binds::MONO_TYPEDEF_METHOD_LIST) as usize - 1
            } else {
                methods.methods().len()
            };
            //BUGFIX: should never normaly happen.
            if method_list > method_list_end || method_list_end > methods.methods().len() {
                continue;
            }
            let methods = methods.methods()[method_list..method_list_end]
                .to_vec()
                .into_boxed_slice();
            let name = img.metadata_string_heap(name);
            let namespace = img.metadata_string_heap(namespace);
            let flags = TypeFlags::new(flags);
            let definition = TypeDefinition {
                flags,
                name,
                namespace,
                extends,
                field_list,
                methods,
            };
            defs.push(definition);
        }
        let defs = defs.into();
        Self { defs }
    }
    pub fn defs(&self) -> &[TypeDefinition] {
        &self.defs
    }
    ///Gets [`TypeDefinitionTable`]
    #[must_use]
    pub fn from_image(img: Image) -> Option<Self> {
        let table = img.get_table_info(MetadataTableKind::TypeDef);
        if table.get_table_rows() > 0 {
            Some(Self::from_meta_table(&table, img))
        } else {
            None
        }
    }
}

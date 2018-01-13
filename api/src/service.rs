// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Operation {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Operation {}

impl Operation {
    pub fn new() -> Operation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Operation {
        static mut instance: ::protobuf::lazy::Lazy<Operation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Operation,
        };
        unsafe {
            instance.get(Operation::new)
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for Operation {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Operation {
    fn new() -> Operation {
        Operation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Operation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Operation::get_name_for_reflect,
                    Operation::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Operation::get_data_for_reflect,
                    Operation::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Operation>(
                    "Operation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Operation {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Operation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fop {
    IPC = 0,
    LOOKUP = 1,
    MKNOD = 2,
    CREATE = 3,
    OPEN = 4,
    STATFS = 5,
    OPENDIR = 6,
    READDIR = 7,
    READDIRP = 8,
    FSYNCDIR = 9,
    SYMLINK = 10,
    UNLINK = 11,
    LINK = 12,
    MKDIR = 13,
    RMDIR = 14,
    RENAME = 15,
    ENTRYLK = 16,
    FENTRYLK = 17,
    STAT = 18,
    FSTAT = 19,
    ACCESS = 20,
    READLINK = 21,
    GETXATTR = 22,
    FGETXATTR = 23,
    READV = 24,
    FLUSH = 25,
    FSYNC = 26,
    INODELK = 27,
    FINODELK = 28,
    LK = 29,
    LEASE = 30,
    FREMOVEXATTR = 31,
    REMOVEXATTR = 32,
    SETXATTR = 33,
    FSETXATTR = 34,
    TRUNCATE = 35,
    FTRUNCATE = 36,
    WRITEV = 37,
    XATTROP = 38,
    FXATTROP = 39,
    SETATTR = 40,
    FSETATTR = 41,
    FALLOCATE = 42,
    DISCARD = 43,
    ZEROFILL = 44,
}

impl ::protobuf::ProtobufEnum for Fop {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fop> {
        match value {
            0 => ::std::option::Option::Some(Fop::IPC),
            1 => ::std::option::Option::Some(Fop::LOOKUP),
            2 => ::std::option::Option::Some(Fop::MKNOD),
            3 => ::std::option::Option::Some(Fop::CREATE),
            4 => ::std::option::Option::Some(Fop::OPEN),
            5 => ::std::option::Option::Some(Fop::STATFS),
            6 => ::std::option::Option::Some(Fop::OPENDIR),
            7 => ::std::option::Option::Some(Fop::READDIR),
            8 => ::std::option::Option::Some(Fop::READDIRP),
            9 => ::std::option::Option::Some(Fop::FSYNCDIR),
            10 => ::std::option::Option::Some(Fop::SYMLINK),
            11 => ::std::option::Option::Some(Fop::UNLINK),
            12 => ::std::option::Option::Some(Fop::LINK),
            13 => ::std::option::Option::Some(Fop::MKDIR),
            14 => ::std::option::Option::Some(Fop::RMDIR),
            15 => ::std::option::Option::Some(Fop::RENAME),
            16 => ::std::option::Option::Some(Fop::ENTRYLK),
            17 => ::std::option::Option::Some(Fop::FENTRYLK),
            18 => ::std::option::Option::Some(Fop::STAT),
            19 => ::std::option::Option::Some(Fop::FSTAT),
            20 => ::std::option::Option::Some(Fop::ACCESS),
            21 => ::std::option::Option::Some(Fop::READLINK),
            22 => ::std::option::Option::Some(Fop::GETXATTR),
            23 => ::std::option::Option::Some(Fop::FGETXATTR),
            24 => ::std::option::Option::Some(Fop::READV),
            25 => ::std::option::Option::Some(Fop::FLUSH),
            26 => ::std::option::Option::Some(Fop::FSYNC),
            27 => ::std::option::Option::Some(Fop::INODELK),
            28 => ::std::option::Option::Some(Fop::FINODELK),
            29 => ::std::option::Option::Some(Fop::LK),
            30 => ::std::option::Option::Some(Fop::LEASE),
            31 => ::std::option::Option::Some(Fop::FREMOVEXATTR),
            32 => ::std::option::Option::Some(Fop::REMOVEXATTR),
            33 => ::std::option::Option::Some(Fop::SETXATTR),
            34 => ::std::option::Option::Some(Fop::FSETXATTR),
            35 => ::std::option::Option::Some(Fop::TRUNCATE),
            36 => ::std::option::Option::Some(Fop::FTRUNCATE),
            37 => ::std::option::Option::Some(Fop::WRITEV),
            38 => ::std::option::Option::Some(Fop::XATTROP),
            39 => ::std::option::Option::Some(Fop::FXATTROP),
            40 => ::std::option::Option::Some(Fop::SETATTR),
            41 => ::std::option::Option::Some(Fop::FSETATTR),
            42 => ::std::option::Option::Some(Fop::FALLOCATE),
            43 => ::std::option::Option::Some(Fop::DISCARD),
            44 => ::std::option::Option::Some(Fop::ZEROFILL),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fop] = &[
            Fop::IPC,
            Fop::LOOKUP,
            Fop::MKNOD,
            Fop::CREATE,
            Fop::OPEN,
            Fop::STATFS,
            Fop::OPENDIR,
            Fop::READDIR,
            Fop::READDIRP,
            Fop::FSYNCDIR,
            Fop::SYMLINK,
            Fop::UNLINK,
            Fop::LINK,
            Fop::MKDIR,
            Fop::RMDIR,
            Fop::RENAME,
            Fop::ENTRYLK,
            Fop::FENTRYLK,
            Fop::STAT,
            Fop::FSTAT,
            Fop::ACCESS,
            Fop::READLINK,
            Fop::GETXATTR,
            Fop::FGETXATTR,
            Fop::READV,
            Fop::FLUSH,
            Fop::FSYNC,
            Fop::INODELK,
            Fop::FINODELK,
            Fop::LK,
            Fop::LEASE,
            Fop::FREMOVEXATTR,
            Fop::REMOVEXATTR,
            Fop::SETXATTR,
            Fop::FSETXATTR,
            Fop::TRUNCATE,
            Fop::FTRUNCATE,
            Fop::WRITEV,
            Fop::XATTROP,
            Fop::FXATTROP,
            Fop::SETATTR,
            Fop::FSETATTR,
            Fop::FALLOCATE,
            Fop::DISCARD,
            Fop::ZEROFILL,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Fop>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fop", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fop {
}

impl ::protobuf::reflect::ProtobufValue for Fop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14protos/service.proto\x12\x05rusix\"'\n\tOperation\x12\x0c\n\x04nam\
    e\x18\x01\x20\x02(\t\x12\x0c\n\x04data\x18\x02\x20\x01(\x0c*\xc1\x04\n\
    \x03Fop\x12\x07\n\x03IPC\x10\0\x12\n\n\x06LOOKUP\x10\x01\x12\t\n\x05MKNO\
    D\x10\x02\x12\n\n\x06CREATE\x10\x03\x12\x08\n\x04OPEN\x10\x04\x12\n\n\
    \x06STATFS\x10\x05\x12\x0b\n\x07OPENDIR\x10\x06\x12\x0b\n\x07READDIR\x10\
    \x07\x12\x0c\n\x08READDIRP\x10\x08\x12\x0c\n\x08FSYNCDIR\x10\t\x12\x0b\n\
    \x07SYMLINK\x10\n\x12\n\n\x06UNLINK\x10\x0b\x12\x08\n\x04LINK\x10\x0c\
    \x12\t\n\x05MKDIR\x10\r\x12\t\n\x05RMDIR\x10\x0e\x12\n\n\x06RENAME\x10\
    \x0f\x12\x0b\n\x07ENTRYLK\x10\x10\x12\x0c\n\x08FENTRYLK\x10\x11\x12\x08\
    \n\x04STAT\x10\x12\x12\t\n\x05FSTAT\x10\x13\x12\n\n\x06ACCESS\x10\x14\
    \x12\x0c\n\x08READLINK\x10\x15\x12\x0c\n\x08GETXATTR\x10\x16\x12\r\n\tFG\
    ETXATTR\x10\x17\x12\t\n\x05READV\x10\x18\x12\t\n\x05FLUSH\x10\x19\x12\t\
    \n\x05FSYNC\x10\x1a\x12\x0b\n\x07INODELK\x10\x1b\x12\x0c\n\x08FINODELK\
    \x10\x1c\x12\x06\n\x02LK\x10\x1d\x12\t\n\x05LEASE\x10\x1e\x12\x10\n\x0cF\
    REMOVEXATTR\x10\x1f\x12\x0f\n\x0bREMOVEXATTR\x10\x20\x12\x0c\n\x08SETXAT\
    TR\x10!\x12\r\n\tFSETXATTR\x10\"\x12\x0c\n\x08TRUNCATE\x10#\x12\r\n\tFTR\
    UNCATE\x10$\x12\n\n\x06WRITEV\x10%\x12\x0b\n\x07XATTROP\x10&\x12\x0c\n\
    \x08FXATTROP\x10'\x12\x0b\n\x07SETATTR\x10(\x12\x0c\n\x08FSETATTR\x10)\
    \x12\r\n\tFALLOCATE\x10*\x12\x0b\n\x07DISCARD\x10+\x12\x0c\n\x08ZEROFILL\
    \x10,B\x02H\x01\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct PublicKey {
    // message fields
    key_type: ::std::option::Option<KeyType>,
    bytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PublicKey {}

impl PublicKey {
    pub fn new() -> PublicKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PublicKey {
        static mut instance: ::protobuf::lazy::Lazy<PublicKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PublicKey,
        };
        unsafe {
            instance.get(|| {
                PublicKey {
                    key_type: ::std::option::Option::None,
                    bytes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .KeyType key_type = 1;

    pub fn clear_key_type(&mut self) {
        self.key_type = ::std::option::Option::None;
    }

    pub fn has_key_type(&self) -> bool {
        self.key_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: KeyType) {
        self.key_type = ::std::option::Option::Some(v);
    }

    pub fn get_key_type<'a>(&self) -> KeyType {
        self.key_type.unwrap_or(KeyType::RSA)
    }

    // required bytes bytes = 2;

    pub fn clear_bytes(&mut self) {
        self.bytes.clear();
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.bytes.is_none() {
            self.bytes.set_default();
        };
        self.bytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_bytes(&mut self) -> ::std::vec::Vec<u8> {
        self.bytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_bytes<'a>(&'a self) -> &'a [u8] {
        match self.bytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for PublicKey {
    fn is_initialized(&self) -> bool {
        if self.key_type.is_none() {
            return false;
        };
        if self.bytes.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.key_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.bytes));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.bytes.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.bytes.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PublicKey>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PublicKey {
    fn new() -> PublicKey {
        PublicKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<PublicKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "key_type",
                    PublicKey::has_key_type,
                    PublicKey::get_key_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "bytes",
                    PublicKey::has_bytes,
                    PublicKey::get_bytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PublicKey>(
                    "PublicKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PublicKey {
    fn clear(&mut self) {
        self.clear_key_type();
        self.clear_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PublicKey {
    fn eq(&self, other: &PublicKey) -> bool {
        self.key_type == other.key_type &&
        self.bytes == other.bytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum KeyType {
    RSA = 0,
}

impl ::protobuf::ProtobufEnum for KeyType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<KeyType> {
        match value {
            0 => ::std::option::Option::Some(KeyType::RSA),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [KeyType] = &[
            KeyType::RSA,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<KeyType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("KeyType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for KeyType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0b, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x36, 0x0a,
    0x09, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x12, 0x1a, 0x0a, 0x08, 0x6b, 0x65,
    0x79, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x08, 0x2e, 0x4b,
    0x65, 0x79, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0c, 0x2a, 0x12, 0x0a, 0x07, 0x4b, 0x65, 0x79, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x07, 0x0a, 0x03, 0x52, 0x53, 0x41, 0x10, 0x00, 0x4a, 0xeb, 0x01, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x00, 0x00, 0x02, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x00, 0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x01, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x01, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x07,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x05, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x05, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05,
    0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x08, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x06, 0x1f, 0x20,
];

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

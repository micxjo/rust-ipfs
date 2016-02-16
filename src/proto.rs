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

#[derive(Clone,Default)]
pub struct Propose {
    // message fields
    rand: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    public_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    exchanges: ::protobuf::SingularField<::std::string::String>,
    ciphers: ::protobuf::SingularField<::std::string::String>,
    hashes: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Propose {}

impl Propose {
    pub fn new() -> Propose {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Propose {
        static mut instance: ::protobuf::lazy::Lazy<Propose> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Propose,
        };
        unsafe {
            instance.get(|| {
                Propose {
                    rand: ::protobuf::SingularField::none(),
                    public_key: ::protobuf::SingularField::none(),
                    exchanges: ::protobuf::SingularField::none(),
                    ciphers: ::protobuf::SingularField::none(),
                    hashes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes rand = 1;

    pub fn clear_rand(&mut self) {
        self.rand.clear();
    }

    pub fn has_rand(&self) -> bool {
        self.rand.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rand(&mut self, v: ::std::vec::Vec<u8>) {
        self.rand = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rand<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.rand.is_none() {
            self.rand.set_default();
        };
        self.rand.as_mut().unwrap()
    }

    // Take field
    pub fn take_rand(&mut self) -> ::std::vec::Vec<u8> {
        self.rand.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_rand<'a>(&'a self) -> &'a [u8] {
        match self.rand.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes public_key = 2;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.public_key.is_none() {
            self.public_key.set_default();
        };
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        self.public_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_public_key<'a>(&'a self) -> &'a [u8] {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string exchanges = 3;

    pub fn clear_exchanges(&mut self) {
        self.exchanges.clear();
    }

    pub fn has_exchanges(&self) -> bool {
        self.exchanges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_exchanges(&mut self, v: ::std::string::String) {
        self.exchanges = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_exchanges<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.exchanges.is_none() {
            self.exchanges.set_default();
        };
        self.exchanges.as_mut().unwrap()
    }

    // Take field
    pub fn take_exchanges(&mut self) -> ::std::string::String {
        self.exchanges.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_exchanges<'a>(&'a self) -> &'a str {
        match self.exchanges.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string ciphers = 4;

    pub fn clear_ciphers(&mut self) {
        self.ciphers.clear();
    }

    pub fn has_ciphers(&self) -> bool {
        self.ciphers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ciphers(&mut self, v: ::std::string::String) {
        self.ciphers = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphers<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ciphers.is_none() {
            self.ciphers.set_default();
        };
        self.ciphers.as_mut().unwrap()
    }

    // Take field
    pub fn take_ciphers(&mut self) -> ::std::string::String {
        self.ciphers.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ciphers<'a>(&'a self) -> &'a str {
        match self.ciphers.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string hashes = 5;

    pub fn clear_hashes(&mut self) {
        self.hashes.clear();
    }

    pub fn has_hashes(&self) -> bool {
        self.hashes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hashes(&mut self, v: ::std::string::String) {
        self.hashes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hashes<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.hashes.is_none() {
            self.hashes.set_default();
        };
        self.hashes.as_mut().unwrap()
    }

    // Take field
    pub fn take_hashes(&mut self) -> ::std::string::String {
        self.hashes.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hashes<'a>(&'a self) -> &'a str {
        match self.hashes.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Propose {
    fn is_initialized(&self) -> bool {
        if self.rand.is_none() {
            return false;
        };
        if self.public_key.is_none() {
            return false;
        };
        if self.exchanges.is_none() {
            return false;
        };
        if self.ciphers.is_none() {
            return false;
        };
        if self.hashes.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rand));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.public_key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.exchanges));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.ciphers));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hashes));
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
        for value in self.rand.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.public_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.exchanges.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.ciphers.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.hashes.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rand.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.public_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.exchanges.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.ciphers.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.hashes.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<Propose>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Propose {
    fn new() -> Propose {
        Propose::new()
    }

    fn descriptor_static(_: ::std::option::Option<Propose>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "rand",
                    Propose::has_rand,
                    Propose::get_rand,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "public_key",
                    Propose::has_public_key,
                    Propose::get_public_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "exchanges",
                    Propose::has_exchanges,
                    Propose::get_exchanges,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ciphers",
                    Propose::has_ciphers,
                    Propose::get_ciphers,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hashes",
                    Propose::has_hashes,
                    Propose::get_hashes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Propose>(
                    "Propose",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Propose {
    fn clear(&mut self) {
        self.clear_rand();
        self.clear_public_key();
        self.clear_exchanges();
        self.clear_ciphers();
        self.clear_hashes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Propose {
    fn eq(&self, other: &Propose) -> bool {
        self.rand == other.rand &&
        self.public_key == other.public_key &&
        self.exchanges == other.exchanges &&
        self.ciphers == other.ciphers &&
        self.hashes == other.hashes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Propose {
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
    0x02, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x5f, 0x0a, 0x07, 0x50, 0x72, 0x6f, 0x70, 0x6f, 0x73, 0x65,
    0x12, 0x0c, 0x0a, 0x04, 0x72, 0x61, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x12,
    0x0a, 0x0a, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x65, 0x78, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x73,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x68, 0x61, 0x73, 0x68, 0x65, 0x73,
    0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x2a, 0x12, 0x0a, 0x07, 0x4b, 0x65, 0x79, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x53, 0x41, 0x10, 0x00, 0x4a, 0xdc, 0x04, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x00, 0x00, 0x02,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x00, 0x05, 0x0c, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x08, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x03, 0x01, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00,
    0x07, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08, 0x11, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x05, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x19, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x05, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x06, 0x08, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x06, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x09, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x09, 0x08, 0x0f,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0a, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x17, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x0a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0b,
    0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0b, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0b, 0x11, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0b, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0b, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x02, 0x12, 0x03, 0x0c, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x0c, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x0c, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x18,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x0d, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0d, 0x18, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0d, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0e, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x0e, 0x08, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0e, 0x11, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0e, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0e, 0x21, 0x22,
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

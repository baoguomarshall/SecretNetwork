// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `secret/registration/v1beta1/msg.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct RaAuthenticate {
    // message fields
    pub sender: ::std::vec::Vec<u8>,
    pub certificate: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RaAuthenticate {
    fn default() -> &'a RaAuthenticate {
        <RaAuthenticate as ::protobuf::Message>::default_instance()
    }
}

impl RaAuthenticate {
    pub fn new() -> RaAuthenticate {
        ::std::default::Default::default()
    }

    // bytes sender = 1;


    pub fn get_sender(&self) -> &[u8] {
        &self.sender
    }
    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::vec::Vec<u8>) {
        self.sender = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.sender
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.sender, ::std::vec::Vec::new())
    }

    // bytes certificate = 2;


    pub fn get_certificate(&self) -> &[u8] {
        &self.certificate
    }
    pub fn clear_certificate(&mut self) {
        self.certificate.clear();
    }

    // Param is passed by value, moved
    pub fn set_certificate(&mut self, v: ::std::vec::Vec<u8>) {
        self.certificate = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_certificate(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.certificate
    }

    // Take field
    pub fn take_certificate(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.certificate, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for RaAuthenticate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.sender)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.certificate)?;
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
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.sender);
        }
        if !self.certificate.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.certificate);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.sender.is_empty() {
            os.write_bytes(1, &self.sender)?;
        }
        if !self.certificate.is_empty() {
            os.write_bytes(2, &self.certificate)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RaAuthenticate {
        RaAuthenticate::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "sender",
                |m: &RaAuthenticate| { &m.sender },
                |m: &mut RaAuthenticate| { &mut m.sender },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "certificate",
                |m: &RaAuthenticate| { &m.certificate },
                |m: &mut RaAuthenticate| { &mut m.certificate },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RaAuthenticate>(
                "RaAuthenticate",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RaAuthenticate {
        static instance: ::protobuf::rt::LazyV2<RaAuthenticate> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RaAuthenticate::new)
    }
}

impl ::protobuf::Clear for RaAuthenticate {
    fn clear(&mut self) {
        self.sender.clear();
        self.certificate.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaAuthenticate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaAuthenticate {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MasterKey {
    // message fields
    pub bytes: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a MasterKey {
    fn default() -> &'a MasterKey {
        <MasterKey as ::protobuf::Message>::default_instance()
    }
}

impl MasterKey {
    pub fn new() -> MasterKey {
        ::std::default::Default::default()
    }

    // bytes bytes = 1;


    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn clear_bytes(&mut self) {
        self.bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.bytes = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bytes(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.bytes
    }

    // Take field
    pub fn take_bytes(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.bytes, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for MasterKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.bytes)?;
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
        if !self.bytes.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.bytes);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.bytes.is_empty() {
            os.write_bytes(1, &self.bytes)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> MasterKey {
        MasterKey::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "bytes",
                |m: &MasterKey| { &m.bytes },
                |m: &mut MasterKey| { &mut m.bytes },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<MasterKey>(
                "MasterKey",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static MasterKey {
        static instance: ::protobuf::rt::LazyV2<MasterKey> = ::protobuf::rt::LazyV2::INIT;
        instance.get(MasterKey::new)
    }
}

impl ::protobuf::Clear for MasterKey {
    fn clear(&mut self) {
        self.bytes.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MasterKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MasterKey {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Key {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Key {
    fn default() -> &'a Key {
        <Key as ::protobuf::Message>::default_instance()
    }
}

impl Key {
    pub fn new() -> Key {
        ::std::default::Default::default()
    }

    // bytes key = 1;


    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Key {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Key {
        Key::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "key",
                |m: &Key| { &m.key },
                |m: &mut Key| { &mut m.key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Key>(
                "Key",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Key {
        static instance: ::protobuf::rt::LazyV2<Key> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Key::new)
    }
}

impl ::protobuf::Clear for Key {
    fn clear(&mut self) {
        self.key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Key {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Key {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n%secret/registration/v1beta1/msg.proto\x12\x1bsecret.registration.v1be\
    ta1\x1a\x14gogoproto/gogo.proto\"\xde\x01\n\x0eRaAuthenticate\x12I\n\x06\
    sender\x18\x01\x20\x01(\x0cR\x06senderB1\xfa\xde\x1f-github.com/cosmos/c\
    osmos-sdk/types.AccAddress\x12\x80\x01\n\x0bcertificate\x18\x02\x20\x01(\
    \x0cR\x0bcertificateB^\xfa\xde\x1fOgithub.com/scrtlabs/SecretNetwork/x/r\
    egistration/remote_attestation.Certificate\xea\xde\x1f\x07ra_cert\"!\n\t\
    MasterKey\x12\x14\n\x05bytes\x18\x01\x20\x01(\x0cR\x05bytes\"\x20\n\x03K\
    ey\x12\x19\n\x03key\x18\x01\x20\x01(\x0cR\x03keyB\x07\xea\xde\x1f\x03key\
    BIZ?github.com/scrtlabs/SecretNetwork/x/registration/internal/types\xc8\
    \xe1\x1e\0\xa8\xe2\x1e\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
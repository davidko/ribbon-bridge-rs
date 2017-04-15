// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct VersionTriplet {
    // message fields
    major: ::std::option::Option<u32>,
    minor: ::std::option::Option<u32>,
    patch: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VersionTriplet {}

impl VersionTriplet {
    pub fn new() -> VersionTriplet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionTriplet {
        static mut instance: ::protobuf::lazy::Lazy<VersionTriplet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionTriplet,
        };
        unsafe {
            instance.get(|| {
                VersionTriplet {
                    major: ::std::option::Option::None,
                    minor: ::std::option::Option::None,
                    patch: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 major = 1;

    pub fn clear_major(&mut self) {
        self.major = ::std::option::Option::None;
    }

    pub fn has_major(&self) -> bool {
        self.major.is_some()
    }

    // Param is passed by value, moved
    pub fn set_major(&mut self, v: u32) {
        self.major = ::std::option::Option::Some(v);
    }

    pub fn get_major(&self) -> u32 {
        self.major.unwrap_or(0)
    }

    // required uint32 minor = 2;

    pub fn clear_minor(&mut self) {
        self.minor = ::std::option::Option::None;
    }

    pub fn has_minor(&self) -> bool {
        self.minor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minor(&mut self, v: u32) {
        self.minor = ::std::option::Option::Some(v);
    }

    pub fn get_minor(&self) -> u32 {
        self.minor.unwrap_or(0)
    }

    // required uint32 patch = 3;

    pub fn clear_patch(&mut self) {
        self.patch = ::std::option::Option::None;
    }

    pub fn has_patch(&self) -> bool {
        self.patch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_patch(&mut self, v: u32) {
        self.patch = ::std::option::Option::Some(v);
    }

    pub fn get_patch(&self) -> u32 {
        self.patch.unwrap_or(0)
    }
}

impl ::protobuf::Message for VersionTriplet {
    fn is_initialized(&self) -> bool {
        if self.major.is_none() {
            return false;
        };
        if self.minor.is_none() {
            return false;
        };
        if self.patch.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.major = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.minor = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.patch = ::std::option::Option::Some(tmp);
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
        for value in &self.major {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.minor {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.patch {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.major {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.minor {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.patch {
            try!(os.write_uint32(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VersionTriplet>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VersionTriplet {
    fn new() -> VersionTriplet {
        VersionTriplet::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionTriplet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "major",
                    VersionTriplet::has_major,
                    VersionTriplet::get_major,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "minor",
                    VersionTriplet::has_minor,
                    VersionTriplet::get_minor,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "patch",
                    VersionTriplet::has_patch,
                    VersionTriplet::get_patch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionTriplet>(
                    "VersionTriplet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionTriplet {
    fn clear(&mut self) {
        self.clear_major();
        self.clear_minor();
        self.clear_patch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VersionTriplet {
    fn eq(&self, other: &VersionTriplet) -> bool {
        self.major == other.major &&
        self.minor == other.minor &&
        self.patch == other.patch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VersionTriplet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Versions {
    // message fields
    rpc: ::protobuf::SingularPtrField<VersionTriplet>,
    interface: ::protobuf::SingularPtrField<VersionTriplet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Versions {}

impl Versions {
    pub fn new() -> Versions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Versions {
        static mut instance: ::protobuf::lazy::Lazy<Versions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Versions,
        };
        unsafe {
            instance.get(|| {
                Versions {
                    rpc: ::protobuf::SingularPtrField::none(),
                    interface: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .barobo.rpc.VersionTriplet rpc = 2;

    pub fn clear_rpc(&mut self) {
        self.rpc.clear();
    }

    pub fn has_rpc(&self) -> bool {
        self.rpc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpc(&mut self, v: VersionTriplet) {
        self.rpc = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpc(&mut self) -> &mut VersionTriplet {
        if self.rpc.is_none() {
            self.rpc.set_default();
        };
        self.rpc.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpc(&mut self) -> VersionTriplet {
        self.rpc.take().unwrap_or_else(|| VersionTriplet::new())
    }

    pub fn get_rpc(&self) -> &VersionTriplet {
        self.rpc.as_ref().unwrap_or_else(|| VersionTriplet::default_instance())
    }

    // required .barobo.rpc.VersionTriplet interface = 3;

    pub fn clear_interface(&mut self) {
        self.interface.clear();
    }

    pub fn has_interface(&self) -> bool {
        self.interface.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interface(&mut self, v: VersionTriplet) {
        self.interface = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interface(&mut self) -> &mut VersionTriplet {
        if self.interface.is_none() {
            self.interface.set_default();
        };
        self.interface.as_mut().unwrap()
    }

    // Take field
    pub fn take_interface(&mut self) -> VersionTriplet {
        self.interface.take().unwrap_or_else(|| VersionTriplet::new())
    }

    pub fn get_interface(&self) -> &VersionTriplet {
        self.interface.as_ref().unwrap_or_else(|| VersionTriplet::default_instance())
    }
}

impl ::protobuf::Message for Versions {
    fn is_initialized(&self) -> bool {
        if self.rpc.is_none() {
            return false;
        };
        if self.interface.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rpc));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.interface));
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
        for value in &self.rpc {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.interface {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rpc.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.interface.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Versions>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Versions {
    fn new() -> Versions {
        Versions::new()
    }

    fn descriptor_static(_: ::std::option::Option<Versions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "rpc",
                    Versions::has_rpc,
                    Versions::get_rpc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "interface",
                    Versions::has_interface,
                    Versions::get_interface,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Versions>(
                    "Versions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Versions {
    fn clear(&mut self) {
        self.clear_rpc();
        self.clear_interface();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Versions {
    fn eq(&self, other: &Versions) -> bool {
        self.rpc == other.rpc &&
        self.interface == other.interface &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Versions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    field_type: ::std::option::Option<Request_Type>,
    fire: ::protobuf::SingularPtrField<Request_Fire>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(|| {
                Request {
                    field_type: ::std::option::Option::None,
                    fire: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .barobo.rpc.Request.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Request_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Request_Type {
        self.field_type.unwrap_or(Request_Type::CONNECT)
    }

    // optional .barobo.rpc.Request.Fire fire = 3;

    pub fn clear_fire(&mut self) {
        self.fire.clear();
    }

    pub fn has_fire(&self) -> bool {
        self.fire.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fire(&mut self, v: Request_Fire) {
        self.fire = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fire(&mut self) -> &mut Request_Fire {
        if self.fire.is_none() {
            self.fire.set_default();
        };
        self.fire.as_mut().unwrap()
    }

    // Take field
    pub fn take_fire(&mut self) -> Request_Fire {
        self.fire.take().unwrap_or_else(|| Request_Fire::new())
    }

    pub fn get_fire(&self) -> &Request_Fire {
        self.fire.as_ref().unwrap_or_else(|| Request_Fire::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fire));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.fire {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.fire.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Request::has_field_type,
                    Request::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fire",
                    Request::has_fire,
                    Request::get_fire,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_fire();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.field_type == other.field_type &&
        self.fire == other.fire &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request_Fire {
    // message fields
    id: ::std::option::Option<u32>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request_Fire {}

impl Request_Fire {
    pub fn new() -> Request_Fire {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request_Fire {
        static mut instance: ::protobuf::lazy::Lazy<Request_Fire> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request_Fire,
        };
        unsafe {
            instance.get(|| {
                Request_Fire {
                    id: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Request_Fire {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.payload.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.payload {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request_Fire>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Request_Fire {
    fn new() -> Request_Fire {
        Request_Fire::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request_Fire>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Request_Fire::has_id,
                    Request_Fire::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    Request_Fire::has_payload,
                    Request_Fire::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request_Fire>(
                    "Request_Fire",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request_Fire {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request_Fire {
    fn eq(&self, other: &Request_Fire) -> bool {
        self.id == other.id &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request_Fire {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Request_Type {
    CONNECT = 0,
    DISCONNECT = 1,
    FIRE = 2,
}

impl ::protobuf::ProtobufEnum for Request_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Request_Type> {
        match value {
            0 => ::std::option::Option::Some(Request_Type::CONNECT),
            1 => ::std::option::Option::Some(Request_Type::DISCONNECT),
            2 => ::std::option::Option::Some(Request_Type::FIRE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Request_Type] = &[
            Request_Type::CONNECT,
            Request_Type::DISCONNECT,
            Request_Type::FIRE,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Request_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Request_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Request_Type {
}

#[derive(Clone,Default)]
pub struct ClientMessage {
    // message fields
    id: ::std::option::Option<u32>,
    request: ::protobuf::SingularPtrField<Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClientMessage {}

impl ClientMessage {
    pub fn new() -> ClientMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientMessage {
        static mut instance: ::protobuf::lazy::Lazy<ClientMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientMessage,
        };
        unsafe {
            instance.get(|| {
                ClientMessage {
                    id: ::std::option::Option::None,
                    request: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required .barobo.rpc.Request request = 2;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: Request) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut Request {
        if self.request.is_none() {
            self.request.set_default();
        };
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> Request {
        self.request.take().unwrap_or_else(|| Request::new())
    }

    pub fn get_request(&self) -> &Request {
        self.request.as_ref().unwrap_or_else(|| Request::default_instance())
    }
}

impl ::protobuf::Message for ClientMessage {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.request.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.request {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.request.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ClientMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientMessage {
    fn new() -> ClientMessage {
        ClientMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    ClientMessage::has_id,
                    ClientMessage::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "request",
                    ClientMessage::has_request,
                    ClientMessage::get_request,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientMessage>(
                    "ClientMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_request();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientMessage {
    fn eq(&self, other: &ClientMessage) -> bool {
        self.id == other.id &&
        self.request == other.request &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Reply {
    // message fields
    field_type: ::std::option::Option<Reply_Type>,
    versions: ::protobuf::SingularPtrField<Versions>,
    status: ::protobuf::SingularPtrField<Reply_Status>,
    result: ::protobuf::SingularPtrField<Reply_Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Reply {}

impl Reply {
    pub fn new() -> Reply {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Reply {
        static mut instance: ::protobuf::lazy::Lazy<Reply> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Reply,
        };
        unsafe {
            instance.get(|| {
                Reply {
                    field_type: ::std::option::Option::None,
                    versions: ::protobuf::SingularPtrField::none(),
                    status: ::protobuf::SingularPtrField::none(),
                    result: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .barobo.rpc.Reply.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Reply_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Reply_Type {
        self.field_type.unwrap_or(Reply_Type::VERSIONS)
    }

    // optional .barobo.rpc.Versions versions = 3;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    pub fn has_versions(&self) -> bool {
        self.versions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: Versions) {
        self.versions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_versions(&mut self) -> &mut Versions {
        if self.versions.is_none() {
            self.versions.set_default();
        };
        self.versions.as_mut().unwrap()
    }

    // Take field
    pub fn take_versions(&mut self) -> Versions {
        self.versions.take().unwrap_or_else(|| Versions::new())
    }

    pub fn get_versions(&self) -> &Versions {
        self.versions.as_ref().unwrap_or_else(|| Versions::default_instance())
    }

    // optional .barobo.rpc.Reply.Status status = 4;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Reply_Status) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut Reply_Status {
        if self.status.is_none() {
            self.status.set_default();
        };
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> Reply_Status {
        self.status.take().unwrap_or_else(|| Reply_Status::new())
    }

    pub fn get_status(&self) -> &Reply_Status {
        self.status.as_ref().unwrap_or_else(|| Reply_Status::default_instance())
    }

    // optional .barobo.rpc.Reply.Result result = 5;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Reply_Result) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result(&mut self) -> &mut Reply_Result {
        if self.result.is_none() {
            self.result.set_default();
        };
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> Reply_Result {
        self.result.take().unwrap_or_else(|| Reply_Result::new())
    }

    pub fn get_result(&self) -> &Reply_Result {
        self.result.as_ref().unwrap_or_else(|| Reply_Result::default_instance())
    }
}

impl ::protobuf::Message for Reply {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.versions));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.versions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.status {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.result {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.versions.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.status.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.result.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Reply>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Reply {
    fn new() -> Reply {
        Reply::new()
    }

    fn descriptor_static(_: ::std::option::Option<Reply>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Reply::has_field_type,
                    Reply::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "versions",
                    Reply::has_versions,
                    Reply::get_versions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "status",
                    Reply::has_status,
                    Reply::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "result",
                    Reply::has_result,
                    Reply::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Reply>(
                    "Reply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Reply {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_versions();
        self.clear_status();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Reply {
    fn eq(&self, other: &Reply) -> bool {
        self.field_type == other.field_type &&
        self.versions == other.versions &&
        self.status == other.status &&
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Reply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Reply_Status {
    // message fields
    value: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Reply_Status {}

impl Reply_Status {
    pub fn new() -> Reply_Status {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Reply_Status {
        static mut instance: ::protobuf::lazy::Lazy<Reply_Status> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Reply_Status,
        };
        unsafe {
            instance.get(|| {
                Reply_Status {
                    value: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .barobo.rpc.Status value = 1;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: Status) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> Status {
        self.value.unwrap_or(Status::OK)
    }
}

impl ::protobuf::Message for Reply_Status {
    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                    self.value = ::std::option::Option::Some(tmp);
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
        for value in &self.value {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value {
            try!(os.write_enum(1, v.value()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Reply_Status>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Reply_Status {
    fn new() -> Reply_Status {
        Reply_Status::new()
    }

    fn descriptor_static(_: ::std::option::Option<Reply_Status>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "value",
                    Reply_Status::has_value,
                    Reply_Status::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Reply_Status>(
                    "Reply_Status",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Reply_Status {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Reply_Status {
    fn eq(&self, other: &Reply_Status) -> bool {
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Reply_Status {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Reply_Result {
    // message fields
    id: ::std::option::Option<u32>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Reply_Result {}

impl Reply_Result {
    pub fn new() -> Reply_Result {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Reply_Result {
        static mut instance: ::protobuf::lazy::Lazy<Reply_Result> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Reply_Result,
        };
        unsafe {
            instance.get(|| {
                Reply_Result {
                    id: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Reply_Result {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.payload.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.payload {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Reply_Result>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Reply_Result {
    fn new() -> Reply_Result {
        Reply_Result::new()
    }

    fn descriptor_static(_: ::std::option::Option<Reply_Result>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Reply_Result::has_id,
                    Reply_Result::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    Reply_Result::has_payload,
                    Reply_Result::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Reply_Result>(
                    "Reply_Result",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Reply_Result {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Reply_Result {
    fn eq(&self, other: &Reply_Result) -> bool {
        self.id == other.id &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Reply_Result {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Reply_Type {
    VERSIONS = 0,
    STATUS = 2,
    RESULT = 3,
}

impl ::protobuf::ProtobufEnum for Reply_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Reply_Type> {
        match value {
            0 => ::std::option::Option::Some(Reply_Type::VERSIONS),
            2 => ::std::option::Option::Some(Reply_Type::STATUS),
            3 => ::std::option::Option::Some(Reply_Type::RESULT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Reply_Type] = &[
            Reply_Type::VERSIONS,
            Reply_Type::STATUS,
            Reply_Type::RESULT,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Reply_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Reply_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Reply_Type {
}

#[derive(Clone,Default)]
pub struct Broadcast {
    // message fields
    id: ::std::option::Option<u32>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Broadcast {}

impl Broadcast {
    pub fn new() -> Broadcast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Broadcast {
        static mut instance: ::protobuf::lazy::Lazy<Broadcast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Broadcast,
        };
        unsafe {
            instance.get(|| {
                Broadcast {
                    id: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u32 {
        self.id.unwrap_or(0)
    }

    // required bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload(&self) -> &[u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Broadcast {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.payload.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.payload));
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
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.payload {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Broadcast>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Broadcast {
    fn new() -> Broadcast {
        Broadcast::new()
    }

    fn descriptor_static(_: ::std::option::Option<Broadcast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "id",
                    Broadcast::has_id,
                    Broadcast::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    Broadcast::has_payload,
                    Broadcast::get_payload,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Broadcast>(
                    "Broadcast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Broadcast {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_payload();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Broadcast {
    fn eq(&self, other: &Broadcast) -> bool {
        self.id == other.id &&
        self.payload == other.payload &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Broadcast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerMessage {
    // message fields
    field_type: ::std::option::Option<ServerMessage_Type>,
    reply: ::protobuf::SingularPtrField<Reply>,
    inReplyTo: ::std::option::Option<u32>,
    broadcast: ::protobuf::SingularPtrField<Broadcast>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerMessage {}

impl ServerMessage {
    pub fn new() -> ServerMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerMessage {
        static mut instance: ::protobuf::lazy::Lazy<ServerMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerMessage,
        };
        unsafe {
            instance.get(|| {
                ServerMessage {
                    field_type: ::std::option::Option::None,
                    reply: ::protobuf::SingularPtrField::none(),
                    inReplyTo: ::std::option::Option::None,
                    broadcast: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .barobo.rpc.ServerMessage.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ServerMessage_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ServerMessage_Type {
        self.field_type.unwrap_or(ServerMessage_Type::REPLY)
    }

    // optional .barobo.rpc.Reply reply = 2;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    pub fn has_reply(&self) -> bool {
        self.reply.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: Reply) {
        self.reply = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reply(&mut self) -> &mut Reply {
        if self.reply.is_none() {
            self.reply.set_default();
        };
        self.reply.as_mut().unwrap()
    }

    // Take field
    pub fn take_reply(&mut self) -> Reply {
        self.reply.take().unwrap_or_else(|| Reply::new())
    }

    pub fn get_reply(&self) -> &Reply {
        self.reply.as_ref().unwrap_or_else(|| Reply::default_instance())
    }

    // optional uint32 inReplyTo = 3;

    pub fn clear_inReplyTo(&mut self) {
        self.inReplyTo = ::std::option::Option::None;
    }

    pub fn has_inReplyTo(&self) -> bool {
        self.inReplyTo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inReplyTo(&mut self, v: u32) {
        self.inReplyTo = ::std::option::Option::Some(v);
    }

    pub fn get_inReplyTo(&self) -> u32 {
        self.inReplyTo.unwrap_or(0)
    }

    // optional .barobo.rpc.Broadcast broadcast = 4;

    pub fn clear_broadcast(&mut self) {
        self.broadcast.clear();
    }

    pub fn has_broadcast(&self) -> bool {
        self.broadcast.is_some()
    }

    // Param is passed by value, moved
    pub fn set_broadcast(&mut self, v: Broadcast) {
        self.broadcast = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_broadcast(&mut self) -> &mut Broadcast {
        if self.broadcast.is_none() {
            self.broadcast.set_default();
        };
        self.broadcast.as_mut().unwrap()
    }

    // Take field
    pub fn take_broadcast(&mut self) -> Broadcast {
        self.broadcast.take().unwrap_or_else(|| Broadcast::new())
    }

    pub fn get_broadcast(&self) -> &Broadcast {
        self.broadcast.as_ref().unwrap_or_else(|| Broadcast::default_instance())
    }
}

impl ::protobuf::Message for ServerMessage {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reply));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.inReplyTo = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.broadcast));
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
        for value in &self.field_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.reply {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.inReplyTo {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.broadcast {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.reply.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.inReplyTo {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.broadcast.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ServerMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerMessage {
    fn new() -> ServerMessage {
        ServerMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    ServerMessage::has_field_type,
                    ServerMessage::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "reply",
                    ServerMessage::has_reply,
                    ServerMessage::get_reply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "inReplyTo",
                    ServerMessage::has_inReplyTo,
                    ServerMessage::get_inReplyTo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "broadcast",
                    ServerMessage::has_broadcast,
                    ServerMessage::get_broadcast,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServerMessage>(
                    "ServerMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerMessage {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_reply();
        self.clear_inReplyTo();
        self.clear_broadcast();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerMessage {
    fn eq(&self, other: &ServerMessage) -> bool {
        self.field_type == other.field_type &&
        self.reply == other.reply &&
        self.inReplyTo == other.inReplyTo &&
        self.broadcast == other.broadcast &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ServerMessage_Type {
    REPLY = 0,
    BROADCAST = 1,
}

impl ::protobuf::ProtobufEnum for ServerMessage_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ServerMessage_Type> {
        match value {
            0 => ::std::option::Option::Some(ServerMessage_Type::REPLY),
            1 => ::std::option::Option::Some(ServerMessage_Type::BROADCAST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ServerMessage_Type] = &[
            ServerMessage_Type::REPLY,
            ServerMessage_Type::BROADCAST,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ServerMessage_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ServerMessage_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ServerMessage_Type {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    OK = 0,
    DECODING_FAILURE = 1,
    ENCODING_FAILURE = 2,
    PROTOCOL_ERROR = 3,
    INTERFACE_ERROR = 4,
    NOT_CONNECTED = 5,
    CONNECTION_REFUSED = 6,
    TIMED_OUT = 7,
    VERSION_MISMATCH = 8,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::OK),
            1 => ::std::option::Option::Some(Status::DECODING_FAILURE),
            2 => ::std::option::Option::Some(Status::ENCODING_FAILURE),
            3 => ::std::option::Option::Some(Status::PROTOCOL_ERROR),
            4 => ::std::option::Option::Some(Status::INTERFACE_ERROR),
            5 => ::std::option::Option::Some(Status::NOT_CONNECTED),
            6 => ::std::option::Option::Some(Status::CONNECTION_REFUSED),
            7 => ::std::option::Option::Some(Status::TIMED_OUT),
            8 => ::std::option::Option::Some(Status::VERSION_MISMATCH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::OK,
            Status::DECODING_FAILURE,
            Status::ENCODING_FAILURE,
            Status::PROTOCOL_ERROR,
            Status::INTERFACE_ERROR,
            Status::NOT_CONNECTED,
            Status::CONNECTION_REFUSED,
            Status::TIMED_OUT,
            Status::VERSION_MISMATCH,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x72, 0x70, 0x63, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x62, 0x61, 0x72,
    0x6f, 0x62, 0x6f, 0x2e, 0x72, 0x70, 0x63, 0x1a, 0x0c, 0x6e, 0x61, 0x6e, 0x6f, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x52, 0x0a, 0x0e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x54, 0x72, 0x69, 0x70, 0x6c, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x6d, 0x61, 0x6a, 0x6f, 0x72,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x05, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x12, 0x14, 0x0a,
    0x05, 0x6d, 0x69, 0x6e, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x05, 0x6d, 0x69,
    0x6e, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x0d, 0x52, 0x05, 0x70, 0x61, 0x74, 0x63, 0x68, 0x22, 0x72, 0x0a, 0x08, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x2c, 0x0a, 0x03, 0x72, 0x70, 0x63, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e, 0x72, 0x70, 0x63, 0x2e,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x72, 0x69, 0x70, 0x6c, 0x65, 0x74, 0x52, 0x03,
    0x72, 0x70, 0x63, 0x12, 0x38, 0x0a, 0x09, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e,
    0x72, 0x70, 0x63, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x54, 0x72, 0x69, 0x70, 0x6c,
    0x65, 0x74, 0x52, 0x09, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x22, 0xce, 0x01,
    0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2c, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f,
    0x2e, 0x72, 0x70, 0x63, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x2c, 0x0a, 0x04, 0x66, 0x69, 0x72, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e, 0x72,
    0x70, 0x63, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x46, 0x69, 0x72, 0x65, 0x52,
    0x04, 0x66, 0x69, 0x72, 0x65, 0x1a, 0x38, 0x0a, 0x04, 0x46, 0x69, 0x72, 0x65, 0x12, 0x0e, 0x0a,
    0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a,
    0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x06,
    0x92, 0x3f, 0x03, 0x08, 0x80, 0x01, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22,
    0x2d, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x4f, 0x4e, 0x4e, 0x45,
    0x43, 0x54, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e, 0x45,
    0x43, 0x54, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x49, 0x52, 0x45, 0x10, 0x02, 0x22, 0x4e,
    0x0a, 0x0d, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x2d, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x13, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0xe7,
    0x02, 0x0a, 0x05, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x2a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e,
    0x72, 0x70, 0x63, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x12, 0x30, 0x0a, 0x08, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e,
    0x72, 0x70, 0x63, 0x2e, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x08, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x30, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e,
    0x72, 0x70, 0x63, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x30, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62,
    0x6f, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x2e, 0x52, 0x65, 0x73, 0x75,
    0x6c, 0x74, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x1a, 0x32, 0x0a, 0x06, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x12, 0x28, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e, 0x72, 0x70, 0x63,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x1a, 0x3a,
    0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c,
    0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x06, 0x92, 0x3f, 0x03, 0x08, 0x80,
    0x01, 0x52, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0x2c, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x53, 0x10, 0x00,
    0x12, 0x0a, 0x0a, 0x06, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x10, 0x02, 0x12, 0x0a, 0x0a, 0x06,
    0x52, 0x45, 0x53, 0x55, 0x4c, 0x54, 0x10, 0x03, 0x22, 0x3d, 0x0a, 0x09, 0x42, 0x72, 0x6f, 0x61,
    0x64, 0x63, 0x61, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0d, 0x52, 0x02, 0x69, 0x64, 0x12, 0x20, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x42, 0x06, 0x92, 0x3f, 0x03, 0x08, 0x80, 0x01, 0x52, 0x07,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x22, 0xe1, 0x01, 0x0a, 0x0d, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x32, 0x0a, 0x04, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f,
    0x2e, 0x72, 0x70, 0x63, 0x2e, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x27, 0x0a,
    0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x62,
    0x61, 0x72, 0x6f, 0x62, 0x6f, 0x2e, 0x72, 0x70, 0x63, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x79, 0x52,
    0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x12, 0x1c, 0x0a, 0x09, 0x69, 0x6e, 0x52, 0x65, 0x70, 0x6c,
    0x79, 0x54, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x09, 0x69, 0x6e, 0x52, 0x65, 0x70,
    0x6c, 0x79, 0x54, 0x6f, 0x12, 0x33, 0x0a, 0x09, 0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73,
    0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x62, 0x61, 0x72, 0x6f, 0x62, 0x6f,
    0x2e, 0x72, 0x70, 0x63, 0x2e, 0x42, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x52, 0x09,
    0x62, 0x72, 0x6f, 0x61, 0x64, 0x63, 0x61, 0x73, 0x74, 0x22, 0x20, 0x0a, 0x04, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x09, 0x0a, 0x05, 0x52, 0x45, 0x50, 0x4c, 0x59, 0x10, 0x00, 0x12, 0x0d, 0x0a, 0x09,
    0x42, 0x52, 0x4f, 0x41, 0x44, 0x43, 0x41, 0x53, 0x54, 0x10, 0x01, 0x2a, 0xb5, 0x01, 0x0a, 0x06,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x4b, 0x10, 0x00, 0x12, 0x14,
    0x0a, 0x10, 0x44, 0x45, 0x43, 0x4f, 0x44, 0x49, 0x4e, 0x47, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x55,
    0x52, 0x45, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x4e, 0x43, 0x4f, 0x44, 0x49, 0x4e, 0x47,
    0x5f, 0x46, 0x41, 0x49, 0x4c, 0x55, 0x52, 0x45, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x50, 0x52,
    0x4f, 0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x03, 0x12, 0x13,
    0x0a, 0x0f, 0x49, 0x4e, 0x54, 0x45, 0x52, 0x46, 0x41, 0x43, 0x45, 0x5f, 0x45, 0x52, 0x52, 0x4f,
    0x52, 0x10, 0x04, 0x12, 0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x54, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45,
    0x43, 0x54, 0x45, 0x44, 0x10, 0x05, 0x12, 0x16, 0x0a, 0x12, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43,
    0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x52, 0x45, 0x46, 0x55, 0x53, 0x45, 0x44, 0x10, 0x06, 0x12, 0x0d,
    0x0a, 0x09, 0x54, 0x49, 0x4d, 0x45, 0x44, 0x5f, 0x4f, 0x55, 0x54, 0x10, 0x07, 0x12, 0x14, 0x0a,
    0x10, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x4d, 0x49, 0x53, 0x4d, 0x41, 0x54, 0x43,
    0x48, 0x10, 0x08, 0x4a, 0xd6, 0x1c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x5d, 0x01, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x00, 0x07, 0x15, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x01, 0x08, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x03, 0x00, 0x11, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x03, 0x05, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x04, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x04, 0x09, 0x0a, 0x0a, 0x1e, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x06, 0x04,
    0x19, 0x1a, 0x11, 0x20, 0x6e, 0x61, 0x6e, 0x6f, 0x70, 0x62, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75,
    0x72, 0x65, 0x73, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06,
    0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x06, 0x17, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x04, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x07, 0x17, 0x18, 0x0a, 0x28, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x09, 0x04, 0x17, 0x1a, 0x1b, 0x20, 0x72, 0x69, 0x62, 0x62, 0x6f, 0x6e, 0x2d,
    0x62, 0x72, 0x69, 0x64, 0x67, 0x65, 0x20, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x15, 0x16, 0x0a,
    0x29, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x18, 0x1a, 0x1c, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x66, 0x61, 0x63, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x6d, 0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x0b, 0x16, 0x17, 0x0a, 0x34, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0d, 0x04, 0x16, 0x1a, 0x27, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x27, 0x73, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x0d, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x0e, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x0e, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x0e, 0x19,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x0f, 0x04, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x0f, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x08, 0x12, 0x03, 0x10, 0x04, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01,
    0x12, 0x03, 0x10, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03,
    0x10, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x13, 0x00, 0x17, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x13, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x14, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14,
    0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x15, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x15, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16,
    0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x19, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x19,
    0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x04, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1a, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x1c, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1a, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x1b, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1b,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x1b, 0x0d, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x1c, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1b, 0x28, 0x29, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x02, 0x12, 0x04, 0x1e, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x1e, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04, 0x00, 0x12, 0x04, 0x1f, 0x04,
    0x23, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1f, 0x09, 0x0d,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x20, 0x08, 0x14, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0f, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x20, 0x12, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x17, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x08, 0x12, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x21, 0x15, 0x16, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x22, 0x08, 0x11, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x08, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x22, 0x0f, 0x10, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x02, 0x03, 0x00, 0x12, 0x04, 0x25, 0x04, 0x28, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x03, 0x00, 0x01, 0x12, 0x03, 0x25, 0x0c, 0x10, 0x0a, 0x1d, 0x0a, 0x06, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x26, 0x08, 0x1f, 0x22, 0x0e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x26, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x18, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x27, 0x08, 0x3d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x27, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x27, 0x11, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x27, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x27, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x08, 0x12, 0x03, 0x27, 0x23, 0x3c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x03, 0x00, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x27, 0x24, 0x3b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02,
    0x03, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x27, 0x24, 0x35, 0x0a, 0x13,
    0x0a, 0x0c, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x27, 0x24, 0x2c, 0x0a, 0x14, 0x0a, 0x0d, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x25, 0x2b, 0x0a, 0x13, 0x0a, 0x0c, 0x04, 0x02, 0x03,
    0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x01, 0x12, 0x03, 0x27, 0x2d, 0x35, 0x0a, 0x14,
    0x0a, 0x0d, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x27, 0x2d, 0x35, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x03, 0x00, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x04, 0x12, 0x03, 0x27, 0x38, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x2a, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2a, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x2b, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x2b, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b,
    0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2e, 0x00, 0x31, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x15, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x1b, 0x22, 0x0c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2f, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x14, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x30, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x30, 0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x30, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x30,
    0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x33, 0x00, 0x4c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x33, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04,
    0x04, 0x00, 0x12, 0x04, 0x34, 0x04, 0x38, 0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x34, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x35, 0x08, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x35, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x35, 0x13, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x36, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x36, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x36, 0x11, 0x12, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x37, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x37, 0x08, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03,
    0x37, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04, 0x3a, 0x04, 0x3c,
    0x05, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x0c, 0x12, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3b, 0x08, 0x2d, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b, 0x08, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3b, 0x11, 0x22, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x23, 0x28, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x2b, 0x2c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x04, 0x03, 0x01, 0x12, 0x04, 0x3e, 0x04, 0x45, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x03, 0x01, 0x01, 0x12, 0x03, 0x3e, 0x0c, 0x12, 0x0a, 0x3b, 0x0a, 0x06, 0x04, 0x04,
    0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x08, 0x1f, 0x22, 0x2c, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x64, 0x2e, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x50, 0x72, 0x6f, 0x78, 0x79, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x3f, 0x08, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x3f, 0x11, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x3f, 0x18, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x3f, 0x1d, 0x1e, 0x0a, 0xb4, 0x01, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x44, 0x08, 0x3d, 0x1a, 0xa4, 0x01, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x69,
    0x67, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x62, 0x79,
    0x20, 0x6b, 0x65, 0x65, 0x70, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x70, 0x20, 0x6f,
    0x66, 0x0a, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x49, 0x44, 0x73, 0x2c, 0x20,
    0x62, 0x75, 0x74, 0x20, 0x64, 0x75, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x69, 0x74, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x6d, 0x61, 0x6b, 0x65, 0x73, 0x0a, 0x20, 0x69,
    0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x75,
    0x63, 0x68, 0x20, 0x65, 0x61, 0x73, 0x69, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x3a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x73, 0x69, 0x6f, 0x20, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x6f, 0x65, 0x73,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x08, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x44, 0x11, 0x16, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x17, 0x1e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x21, 0x22, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08, 0x12, 0x03, 0x44, 0x23, 0x3c, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x44, 0x24,
    0x3b, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x44, 0x24, 0x35, 0x0a, 0x13, 0x0a, 0x0c, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x44, 0x24, 0x2c, 0x0a, 0x14, 0x0a, 0x0d, 0x04, 0x04,
    0x03, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x25, 0x2b,
    0x0a, 0x13, 0x0a, 0x0c, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x44, 0x2d, 0x35, 0x0a, 0x14, 0x0a, 0x0d, 0x04, 0x04, 0x03, 0x01, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x2d, 0x35, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x04, 0x03, 0x01, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x04, 0x12, 0x03, 0x44, 0x38, 0x3b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x48, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x48, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x48, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x48, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x49, 0x04,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x49, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x49, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x49, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x02, 0x12, 0x03, 0x4a, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4a,
    0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x14, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x1d, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x4b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x4b, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4e, 0x00, 0x51, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4e, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f,
    0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x50, 0x04, 0x39, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x50, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x50, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x50, 0x1f, 0x38, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x50, 0x20, 0x37, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x50, 0x20, 0x31, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x50, 0x20, 0x28, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x21, 0x27, 0x0a, 0x11, 0x0a,
    0x0a, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x01, 0x12, 0x03, 0x50, 0x29, 0x31,
    0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x50, 0x29, 0x31, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x05, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x04, 0x12, 0x03, 0x50, 0x34, 0x37, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x53, 0x00,
    0x5d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x53, 0x08, 0x15, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x06, 0x04, 0x00, 0x12, 0x04, 0x54, 0x04, 0x57, 0x05, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x04, 0x00, 0x01, 0x12, 0x03, 0x54, 0x09, 0x0d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x55, 0x08, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x55, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x55, 0x10, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x06, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x56, 0x08, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x56, 0x08, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x03, 0x56, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00,
    0x12, 0x03, 0x59, 0x04, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x59, 0x0d,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x12, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x59, 0x19, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x5a, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x5a, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5a,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x04, 0x22, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x5b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x5b, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12,
    0x03, 0x5c, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x5c,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x06, 0x12, 0x03, 0x5c, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x5c, 0x17, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x5c, 0x23, 0x24,
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

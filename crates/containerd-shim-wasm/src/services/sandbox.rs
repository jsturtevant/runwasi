// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by pure
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `sandbox.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.CreateRequest)
pub struct CreateRequest {
    // message fields
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.CreateRequest.namespace)
    pub namespace: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.CreateRequest.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.CreateRequest.ttrpc_address)
    pub ttrpc_address: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.CreateRequest.working_directory)
    pub working_directory: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.CreateRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateRequest {
    fn default() -> &'a CreateRequest {
        <CreateRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateRequest {
    pub fn new() -> CreateRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "namespace",
            |m: &CreateRequest| { &m.namespace },
            |m: &mut CreateRequest| { &mut m.namespace },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &CreateRequest| { &m.id },
            |m: &mut CreateRequest| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ttrpc_address",
            |m: &CreateRequest| { &m.ttrpc_address },
            |m: &mut CreateRequest| { &mut m.ttrpc_address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "working_directory",
            |m: &CreateRequest| { &m.working_directory },
            |m: &mut CreateRequest| { &mut m.working_directory },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateRequest>(
            "CreateRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateRequest {
    const NAME: &'static str = "CreateRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.namespace = is.read_string()?;
                },
                18 => {
                    self.id = is.read_string()?;
                },
                26 => {
                    self.ttrpc_address = is.read_string()?;
                },
                34 => {
                    self.working_directory = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        if !self.ttrpc_address.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.ttrpc_address);
        }
        if !self.working_directory.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.working_directory);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        }
        if !self.ttrpc_address.is_empty() {
            os.write_string(3, &self.ttrpc_address)?;
        }
        if !self.working_directory.is_empty() {
            os.write_string(4, &self.working_directory)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CreateRequest {
        CreateRequest::new()
    }

    fn clear(&mut self) {
        self.namespace.clear();
        self.id.clear();
        self.ttrpc_address.clear();
        self.working_directory.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateRequest {
        static instance: CreateRequest = CreateRequest {
            namespace: ::std::string::String::new(),
            id: ::std::string::String::new(),
            ttrpc_address: ::std::string::String::new(),
            working_directory: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.CreateResponse)
pub struct CreateResponse {
    // message fields
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.CreateResponse.socket_path)
    pub socket_path: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.CreateResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CreateResponse {
    fn default() -> &'a CreateResponse {
        <CreateResponse as ::protobuf::Message>::default_instance()
    }
}

impl CreateResponse {
    pub fn new() -> CreateResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "socket_path",
            |m: &CreateResponse| { &m.socket_path },
            |m: &mut CreateResponse| { &mut m.socket_path },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CreateResponse>(
            "CreateResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CreateResponse {
    const NAME: &'static str = "CreateResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.socket_path = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.socket_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.socket_path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.socket_path.is_empty() {
            os.write_string(1, &self.socket_path)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CreateResponse {
        CreateResponse::new()
    }

    fn clear(&mut self) {
        self.socket_path.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CreateResponse {
        static instance: CreateResponse = CreateResponse {
            socket_path: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CreateResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CreateResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CreateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.ConnectRequest)
pub struct ConnectRequest {
    // message fields
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.ConnectRequest.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.ConnectRequest.ttrpc_address)
    pub ttrpc_address: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.ConnectRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ConnectRequest {
    fn default() -> &'a ConnectRequest {
        <ConnectRequest as ::protobuf::Message>::default_instance()
    }
}

impl ConnectRequest {
    pub fn new() -> ConnectRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &ConnectRequest| { &m.id },
            |m: &mut ConnectRequest| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ttrpc_address",
            |m: &ConnectRequest| { &m.ttrpc_address },
            |m: &mut ConnectRequest| { &mut m.ttrpc_address },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ConnectRequest>(
            "ConnectRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ConnectRequest {
    const NAME: &'static str = "ConnectRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.id = is.read_string()?;
                },
                18 => {
                    self.ttrpc_address = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.ttrpc_address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.ttrpc_address);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.ttrpc_address.is_empty() {
            os.write_string(2, &self.ttrpc_address)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ConnectRequest {
        ConnectRequest::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.ttrpc_address.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ConnectRequest {
        static instance: ConnectRequest = ConnectRequest {
            id: ::std::string::String::new(),
            ttrpc_address: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ConnectRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ConnectRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ConnectRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.ConnectResponse)
pub struct ConnectResponse {
    // message fields
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.ConnectResponse.socket_path)
    pub socket_path: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.ConnectResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ConnectResponse {
    fn default() -> &'a ConnectResponse {
        <ConnectResponse as ::protobuf::Message>::default_instance()
    }
}

impl ConnectResponse {
    pub fn new() -> ConnectResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "socket_path",
            |m: &ConnectResponse| { &m.socket_path },
            |m: &mut ConnectResponse| { &mut m.socket_path },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ConnectResponse>(
            "ConnectResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ConnectResponse {
    const NAME: &'static str = "ConnectResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.socket_path = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.socket_path.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.socket_path);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.socket_path.is_empty() {
            os.write_string(1, &self.socket_path)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ConnectResponse {
        ConnectResponse::new()
    }

    fn clear(&mut self) {
        self.socket_path.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ConnectResponse {
        static instance: ConnectResponse = ConnectResponse {
            socket_path: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ConnectResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ConnectResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ConnectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConnectResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.DeleteRequest)
pub struct DeleteRequest {
    // message fields
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.DeleteRequest.namespace)
    pub namespace: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.DeleteRequest.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:runwasi.services.sandbox.v1.DeleteRequest.ttrpc_address)
    pub ttrpc_address: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.DeleteRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DeleteRequest {
    fn default() -> &'a DeleteRequest {
        <DeleteRequest as ::protobuf::Message>::default_instance()
    }
}

impl DeleteRequest {
    pub fn new() -> DeleteRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "namespace",
            |m: &DeleteRequest| { &m.namespace },
            |m: &mut DeleteRequest| { &mut m.namespace },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &DeleteRequest| { &m.id },
            |m: &mut DeleteRequest| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ttrpc_address",
            |m: &DeleteRequest| { &m.ttrpc_address },
            |m: &mut DeleteRequest| { &mut m.ttrpc_address },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DeleteRequest>(
            "DeleteRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DeleteRequest {
    const NAME: &'static str = "DeleteRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.namespace = is.read_string()?;
                },
                18 => {
                    self.id = is.read_string()?;
                },
                26 => {
                    self.ttrpc_address = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.namespace.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.namespace);
        }
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.id);
        }
        if !self.ttrpc_address.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.ttrpc_address);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.namespace.is_empty() {
            os.write_string(1, &self.namespace)?;
        }
        if !self.id.is_empty() {
            os.write_string(2, &self.id)?;
        }
        if !self.ttrpc_address.is_empty() {
            os.write_string(3, &self.ttrpc_address)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DeleteRequest {
        DeleteRequest::new()
    }

    fn clear(&mut self) {
        self.namespace.clear();
        self.id.clear();
        self.ttrpc_address.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DeleteRequest {
        static instance: DeleteRequest = DeleteRequest {
            namespace: ::std::string::String::new(),
            id: ::std::string::String::new(),
            ttrpc_address: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DeleteRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DeleteRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:runwasi.services.sandbox.v1.DeleteResponse)
pub struct DeleteResponse {
    // special fields
    // @@protoc_insertion_point(special_field:runwasi.services.sandbox.v1.DeleteResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DeleteResponse {
    fn default() -> &'a DeleteResponse {
        <DeleteResponse as ::protobuf::Message>::default_instance()
    }
}

impl DeleteResponse {
    pub fn new() -> DeleteResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(0);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DeleteResponse>(
            "DeleteResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DeleteResponse {
    const NAME: &'static str = "DeleteResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> DeleteResponse {
        DeleteResponse::new()
    }

    fn clear(&mut self) {
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DeleteResponse {
        static instance: DeleteResponse = DeleteResponse {
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DeleteResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DeleteResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rsandbox.proto\x12\x1brunwasi.services.sandbox.v1\"\x8f\x01\n\rCreate\
    Request\x12\x1c\n\tnamespace\x18\x01\x20\x01(\tR\tnamespace\x12\x0e\n\
    \x02id\x18\x02\x20\x01(\tR\x02id\x12#\n\rttrpc_address\x18\x03\x20\x01(\
    \tR\x0cttrpcAddress\x12+\n\x11working_directory\x18\x04\x20\x01(\tR\x10w\
    orkingDirectory\"1\n\x0eCreateResponse\x12\x1f\n\x0bsocket_path\x18\x01\
    \x20\x01(\tR\nsocketPath\"E\n\x0eConnectRequest\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\tR\x02id\x12#\n\rttrpc_address\x18\x02\x20\x01(\tR\x0cttrpcAdd\
    ress\"2\n\x0fConnectResponse\x12\x1f\n\x0bsocket_path\x18\x01\x20\x01(\t\
    R\nsocketPath\"b\n\rDeleteRequest\x12\x1c\n\tnamespace\x18\x01\x20\x01(\
    \tR\tnamespace\x12\x0e\n\x02id\x18\x02\x20\x01(\tR\x02id\x12#\n\rttrpc_a\
    ddress\x18\x03\x20\x01(\tR\x0cttrpcAddress\"\x10\n\x0eDeleteResponse2\
    \xb5\x02\n\x07Manager\x12a\n\x06Create\x12*.runwasi.services.sandbox.v1.\
    CreateRequest\x1a+.runwasi.services.sandbox.v1.CreateResponse\x12d\n\x07\
    Connect\x12+.runwasi.services.sandbox.v1.ConnectRequest\x1a,.runwasi.ser\
    vices.sandbox.v1.ConnectResponse\x12a\n\x06Delete\x12*.runwasi.services.\
    sandbox.v1.DeleteRequest\x1a+.runwasi.services.sandbox.v1.DeleteResponse\
    B=Z;github.com/cpuguy83/runwasi/api/services/sandbox/v1;sandboxb\x06prot\
    o3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(6);
            messages.push(CreateRequest::generated_message_descriptor_data());
            messages.push(CreateResponse::generated_message_descriptor_data());
            messages.push(ConnectRequest::generated_message_descriptor_data());
            messages.push(ConnectResponse::generated_message_descriptor_data());
            messages.push(DeleteRequest::generated_message_descriptor_data());
            messages.push(DeleteResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}

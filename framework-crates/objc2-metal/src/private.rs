//! Private functionality.
//!
//! The credit for finding these belong to the [metal-rs] project.
//!
//! [metal-rs]: https://github.com/gfx-rs/metal-rs
#![allow(clippy::missing_safety_doc)]
#![allow(unused_imports)]
use std::ffi::c_void;

use crate::*;

use objc2::rc::{Allocated, Id};
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{extern_methods, msg_send_id, Message};

pub unsafe trait MTLDevicePrivate: Message {
    unsafe fn vendor_name(&self) -> Id<objc2_foundation::NSString> {
        unsafe { msg_send_id![self, vendorName] }
    }

    unsafe fn family_name(&self) -> Id<objc2_foundation::NSString> {
        unsafe { msg_send_id![self, familyName] }
    }
}

#[cfg(feature = "MTLDevice")]
unsafe impl<P: MTLDevice + Message> MTLDevicePrivate for P {}

extern_methods!(
    #[cfg(feature = "MTLRenderPipeline")]
    unsafe impl MTLRenderPipelineReflection {
        #[cfg(feature = "MTLDevice")]
        #[method_id(initWithVertexData:fragmentData:serializedVertexDescriptor:device:options:flags:)]
        pub unsafe fn init_with_vertex_data(
            this: Allocated<Self>,
            vertex_data: *mut c_void,
            fragment_data: *mut c_void,
            vertex_desc: *mut c_void,
            device: &ProtocolObject<dyn MTLDevice>,
            options: u64,
            flags: u64,
        ) -> Option<Id<Self>>;

        #[method_id(newSerializedVertexDataWithFlags:error:_)]
        pub unsafe fn new_serialized_vertex_data_with_flags_error(
            &self,
            flags: u64,
        ) -> Result<Id<AnyObject>, Id<objc2_foundation::NSError>>;

        #[method(serializeFragmentData)]
        pub unsafe fn serialize_fragment_data(&self) -> *mut c_void;
    }
);

extern_methods!(
    #[cfg(feature = "MTLSamplerDescriptor")]
    unsafe impl MTLSamplerDescriptor {
        #[method(setLodBias:)]
        pub unsafe fn set_lod_bias(&self, bias: f32);
    }
);

extern_methods!(
    #[cfg(feature = "MTLVertexDescriptor")]
    unsafe impl MTLVertexDescriptor {
        #[method_id(newSerializedDescriptor)]
        pub unsafe fn new_serialized_descriptor(&self) -> Option<Id<AnyObject>>;
    }
);

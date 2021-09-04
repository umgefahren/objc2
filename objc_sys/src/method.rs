use std::os::raw::{c_char, c_uint};

use crate::{objc_selector, OpaqueData, IMP};

/// A type that represents a method in a class definition.
#[repr(C)]
pub struct objc_method {
    _priv: [u8; 0],
    _p: OpaqueData,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct objc_method_description {
    pub name: *const objc_selector,
    pub types: *mut c_char,
}

extern "C" {
    pub fn method_copyArgumentType(m: *mut objc_method, index: c_uint) -> *mut c_char;
    pub fn method_copyReturnType(m: *mut objc_method) -> *mut c_char;
    pub fn method_exchangeImplementations(m1: *mut objc_method, m2: *mut objc_method);
    pub fn method_getArgumentType(
        m: *mut objc_method,
        index: c_uint,
        dst: *mut c_char,
        dst_len: usize,
    );
    pub fn method_getDescription(m: *mut objc_method) -> *mut objc_method_description;
    pub fn method_getImplementation(m: *mut objc_method) -> IMP;
    pub fn method_getName(m: *mut objc_method) -> *const objc_selector;
    pub fn method_getNumberOfArguments(m: *mut objc_method) -> c_uint;
    pub fn method_getReturnType(m: *mut objc_method, dst: *mut c_char, dst_len: usize);
    pub fn method_getTypeEncoding(m: *mut objc_method) -> *const c_char;
    pub fn method_setImplementation(m: *mut objc_method, imp: IMP) -> IMP;
}

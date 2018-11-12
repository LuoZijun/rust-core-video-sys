#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, improper_ctypes)]
#![cfg(any(target_os = "macos", target_os = "ios"))]

extern crate libc;
extern crate core_foundation_sys;
extern crate coremedia_sys;


use libc::{c_void, size_t};
use coremedia_sys::OSType;
use core_foundation_sys::base::{Boolean, CFTypeID, CFTypeRef};


pub type CVReturn = i32;
pub type CVPixelBufferLockFlags = u64;
pub type CVBufferRef = *mut __CVBuffer;
pub type CVImageBufferRef = CVBufferRef;
pub type CVPixelBufferRef = CVImageBufferRef;
pub type CVOptionFlags = u64;

pub const kCVPixelBufferLock_ReadOnly: CVPixelBufferLockFlags = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVBuffer;

pub type CVPixelBufferPoolRef = CFTypeRef;


#[link(name="CoreVideo", kind="framework")]
extern {
    pub fn CVBufferGetTypeID() -> CFTypeID;
    pub fn CVPixelBufferLockBaseAddress(pixelBuffer: CVPixelBufferRef,
                                        lockFlags: CVOptionFlags)
                                        -> CVReturn;
    pub fn CVPixelBufferUnlockBaseAddress(pixelBuffer: CVPixelBufferRef,
                                          unlockFlags: CVOptionFlags)
                                          -> CVReturn;
    pub fn CVPixelBufferGetBaseAddressOfPlane(pixelBuffer: CVPixelBufferRef,
                                              planeIndex: size_t)
                                              -> *mut c_void;
    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;
    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;
    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> size_t;
    pub fn CVPixelBufferGetBytesPerRowOfPlane(pixelBuffer: CVPixelBufferRef,
                                              planeIndex: size_t)
                                              -> size_t;
    pub fn CVPixelBufferGetWidthOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t)
                                        -> size_t;
    pub fn CVPixelBufferGetHeightOfPlane(pixelBuffer: CVPixelBufferRef, planeIndex: size_t)
                                         -> size_t;
}
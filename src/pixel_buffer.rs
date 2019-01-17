use core_foundation_sys::{
    base::{Boolean, CFAllocatorRef, CFTypeID},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use libc::{c_void, size_t};

use crate::{base::CVOptionFlags, image_buffer::CVImageBufferRef, r#return::CVReturn, OSType};

pub type CVPixelBufferLockFlags = u64;
pub type CVPixelBufferRef = CVImageBufferRef;

pub const kCVPixelBufferLock_ReadOnly: CVPixelBufferLockFlags = 1;

extern "C" {
    pub static kCVPixelBufferIOSurfacePropertiesKey: CFStringRef;
    pub static kCVPixelBufferMetalCompatibilityKey: CFStringRef;

    pub fn CVBufferGetTypeID() -> CFTypeID;

    pub fn CVPixelBufferCreate(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        pixelFormatType: OSType,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;

    pub fn CVPixelBufferLockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        lockFlags: CVOptionFlags,
    ) -> CVReturn;

    pub fn CVPixelBufferUnlockBaseAddress(
        pixelBuffer: CVPixelBufferRef,
        unlockFlags: CVOptionFlags,
    ) -> CVReturn;

    pub fn CVPixelBufferGetBaseAddressOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> *mut c_void;

    pub fn CVPixelBufferIsPlanar(pixelBuffer: CVPixelBufferRef) -> Boolean;

    pub fn CVPixelBufferGetPixelFormatType(pixelBuffer: CVPixelBufferRef) -> OSType;

    pub fn CVPixelBufferGetPlaneCount(pixelBuffer: CVPixelBufferRef) -> size_t;

    pub fn CVPixelBufferGetBytesPerRowOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;

    pub fn CVPixelBufferGetWidthOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;

    pub fn CVPixelBufferGetHeightOfPlane(
        pixelBuffer: CVPixelBufferRef,
        planeIndex: size_t,
    ) -> size_t;
}

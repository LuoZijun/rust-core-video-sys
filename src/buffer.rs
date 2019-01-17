#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __CVBuffer;

pub type CVBufferRef = *mut __CVBuffer;

extern "C" {
    pub fn CVBufferRelease(buffer: CVBufferRef);
}

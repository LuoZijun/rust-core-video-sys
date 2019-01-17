#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    improper_ctypes
)]

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}

extern crate core_foundation_sys;
extern crate libc;
extern crate metal;
extern crate objc;

use libc::{c_int, c_uint};

pub(crate) type OSType = u32;
pub(crate) type GLenum = c_uint;
pub(crate) type GLsizei = c_int;
pub(crate) type GLint = c_int;
pub(crate) type GLuint = c_uint;

pub mod base;
pub mod buffer;
pub mod image_buffer;
pub mod metal_texture;
pub mod metal_texture_cache;
pub mod open_gl_es_texture;
pub mod open_gl_es_texture_cache;
pub mod pixel_buffer;
pub mod pixel_buffer_pool;
pub mod r#return;

use crate::image_buffer::CVImageBufferRef;

pub type CVMetalTextureRef = CVImageBufferRef;

extern "C" {
    pub fn CVMetalTextureGetTexture(image: CVMetalTextureRef) -> metal::Texture;
}

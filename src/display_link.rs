use crate::core_foundation_sys::base::CFTypeID;

extern "C" {
    pub fn CVDisplayLinkGetTypeID() -> CFTypeID;
}
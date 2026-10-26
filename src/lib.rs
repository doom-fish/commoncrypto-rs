#![cfg(target_vendor = "apple")]
#![doc = include_str!("../README.md")]

use std::io;

pub mod ffi {
    #![allow(non_camel_case_types, non_snake_case)]

    use core::ffi::{c_int, c_void};

    #[repr(C)]
    #[derive(Debug, Default)]
    pub struct CC_SHA256_CTX {
        pub count: [u32; 2],
        pub hash: [u32; 8],
        pub wbuf: [u32; 16],
    }

    #[link(name = "System")]
    unsafe extern "C" {
        pub fn CC_SHA256_Init(context: *mut CC_SHA256_CTX) -> c_int;
        pub fn CC_SHA256_Update(
            context: *mut CC_SHA256_CTX,
            data: *const c_void,
            length: u32,
        ) -> c_int;
        pub fn CC_SHA256_Final(digest: *mut u8, context: *mut CC_SHA256_CTX) -> c_int;
    }
}

#[derive(Debug)]
pub struct Sha256(ffi::CC_SHA256_CTX);

impl Sha256 {
    pub fn new() -> io::Result<Self> {
        let mut context = ffi::CC_SHA256_CTX::default();
        if unsafe { ffi::CC_SHA256_Init(&raw mut context) } != 1 {
            return Err(io::Error::other("CC_SHA256_Init failed"));
        }
        Ok(Self(context))
    }

    pub fn update(&mut self, bytes: &[u8]) -> io::Result<()> {
        for chunk in bytes.chunks(u32::MAX as usize) {
            let length = u32::try_from(chunk.len()).expect("a SHA-256 chunk fits in u32");
            if unsafe { ffi::CC_SHA256_Update(&raw mut self.0, chunk.as_ptr().cast(), length) } != 1
            {
                return Err(io::Error::other("CC_SHA256_Update failed"));
            }
        }
        Ok(())
    }

    pub fn finalize(mut self) -> io::Result<[u8; 32]> {
        let mut digest = [0; 32];
        if unsafe { ffi::CC_SHA256_Final(digest.as_mut_ptr(), &raw mut self.0) } != 1 {
            return Err(io::Error::other("CC_SHA256_Final failed"));
        }
        Ok(digest)
    }
}

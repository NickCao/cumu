#![allow(non_snake_case)]
use cuda_sys::*;
use std::os::raw::{c_int, c_uint};

#[no_mangle]
pub unsafe extern "C" fn cuInit(_flags: c_uint) -> CUresult {
    CUresult::CUDA_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> CUresult {
    *version = 1000 * 12 + 10 * 0;
    CUresult::CUDA_SUCCESS
}

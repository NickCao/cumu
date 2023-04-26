#![allow(non_snake_case)]
use cuda_sys::*;
use std::os::raw::c_uint;

#[no_mangle]
pub unsafe extern "C" fn cuInit(_flags: c_uint) -> CUresult {
    CUresult::CUDA_SUCCESS
}

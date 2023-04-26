#![allow(non_snake_case)]
use cuda_sys::*;
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_uint, c_void},
};

#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress_v2(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    cudaVersion: c_int,
    flags: cuuint64_t,
    status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    let symbol = CStr::from_ptr(symbol).to_str().unwrap();
    eprintln!(
        "cuGetProcAddress_v2(symbol: {}, version: {}, flags: {})",
        &symbol, cudaVersion, flags,
    );
    *pfn = match symbol {
        "cuInit" => cuInit as _,
        "cuGetProcAddress" => cuGetProcAddress_v2 as _,
        "cuDriverGetVersion" => cuDriverGetVersion as _,
        _ => stub as _,
    };
    if !status.is_null() {
        *status = CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS;
    }
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuInit(_flags: c_uint) -> CUresult {
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> CUresult {
    *version = 1000 * 12 + 10 * 0;
    CUresult::CUDA_SUCCESS
}

#[no_mangle]
pub unsafe extern "C" fn stub() -> CUresult {
    eprintln!("stub");
    CUresult::CUDA_ERROR_UNKNOWN
}

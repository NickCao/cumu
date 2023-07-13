#![allow(non_snake_case)]
use cuda_sys::*;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

lazy_static::lazy_static! {
    static ref LIBCUDA: libloading::Library = unsafe {
        libloading::Library::new("/run/opengl-driver/lib/libcuda.so.1").unwrap()
    };
}

#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress_v2(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    cudaVersion: c_int,
    flags: cuuint64_t,
    status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *const c_char,
            *mut *mut c_void,
            c_int,
            cuuint64_t,
            *mut CUdriverProcAddressQueryResult,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGetProcAddress_v2").unwrap();
    eprintln!("looking for symbol {:?}", CStr::from_ptr(symbol));
    func(symbol, pfn, cudaVersion, flags, status)
}

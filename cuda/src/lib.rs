#![allow(non_snake_case)]
use cuda_sys::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref LIBCUDA: libloading::Library = unsafe {
        libloading::Library::new(std::env::var("LIBCUDA").unwrap_or("/usr/lib/libcuda.so.1".to_string())).unwrap()
    };
    static ref TABEL: Mutex<HashMap<(CString, c_int, cuuint64_t), usize>> = Default::default();
}

#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress_v2(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    cudaVersion: c_int,
    flags: cuuint64_t,
    status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    let lookup: libloading::Symbol<
        unsafe extern "C" fn(
            *const c_char,
            *mut *mut c_void,
            c_int,
            cuuint64_t,
            *mut CUdriverProcAddressQueryResult,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGetProcAddress_v2").unwrap();

    let res = lookup(symbol, pfn, cudaVersion, flags, status);

    let symbol = CStr::from_ptr(symbol);

    eprintln!(
        "cuGetProcAddress_v2({:?}, {:?}, {}, {}, {:?}) -> {:?}",
        symbol,
        pfn.as_ref(),
        cudaVersion,
        flags,
        status.as_ref(),
        res
    );

    TABEL
        .lock()
        .unwrap()
        .insert((symbol.into(), cudaVersion, flags), *pfn as _);

    match (symbol.to_str().unwrap(), cudaVersion, flags) {
        ("cuInit", 2000, 0) => {
            *pfn = cuInit as _;
        }
        ("cuGetProcAddress", _, 0) => {
            *pfn = cuGetProcAddress_v2 as _;
        }
        ("cuGetExportTable", 3000, 0) => {
            *pfn = cuGetExportTable as _;
        }
        _ => (),
    }

    res
}

pub unsafe extern "C" fn cuInit(flags: c_uint) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(c_uint) -> CUresult> =
        LIBCUDA.get(b"cuInit").unwrap();

    let result = func(flags);
    eprintln!("cuInit(flags: {}) -> {:?}", flags, result);
    result
}

pub unsafe extern "C" fn cuGetExportTable(
    ppExportTable: *mut *const c_void,
    pExportTableId: *const CUuuid,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut *const c_void, *const CUuuid) -> CUresult,
    > = LIBCUDA.get(b"cuGetExportTable").unwrap();

    let result = func(ppExportTable, pExportTableId);
    eprintln!(
        "cuGetExportTable(ppExportTable: {:?}, pExportTableId: {:?}) -> {:?}",
        ppExportTable.as_ref(),
        pExportTableId.as_ref(),
        result
    );
    result
}

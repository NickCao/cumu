#![allow(non_snake_case)]
use libc::size_t;
use cuda_sys::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
<<<<<<< HEAD
use std::os::raw::{c_char, c_int, c_uint, c_void};
=======
use std::os::raw::{c_char, c_int, c_uint, c_void, c_ulonglong};
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
use std::sync::Mutex;


lazy_static::lazy_static! {
    static ref LIBCUDA: libloading::Library = unsafe {
        libloading::Library::new(std::env::var("LIBCUDA").unwrap_or("/usr/lib/wsl/lib/libcuda.so".to_string())).unwrap()
        // libloading::Library::new(std::env::var("LIBCUDA").unwrap_or("/home/conqueror712/CUDA-Simulator/target/release/libcuda.so".to_string())).unwrap()
    };
    static ref TABEL: Mutex<HashMap<(CString, c_int, cuuint64_t), usize>> = Default::default();
}

pub type CSizeT = usize;
pub type CSsizeT = isize;

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

    TABEL
        .lock()
        .unwrap()
        .insert((symbol.into(), cudaVersion, flags), *pfn as _);

    match (symbol.to_str().unwrap(), cudaVersion, flags) {
        ("cuInit", 2000, 0) => {
            *pfn = cuInit as _;
        }
        ("cuDeviceGetCount", 2000, 0) => {
            *pfn = cuDeviceGetCount as _;
        }
        ("cuDeviceGet", 2000, 0) => {
            *pfn = cuDeviceGet as _;
        }
        ("cuDeviceGetName", 2000, 0) => {
            *pfn = cuDeviceGetName as _;
        }
        ("cuDeviceGetAttribute", 2000, 0) => {
            *pfn = cuDeviceGetAttribute as _;
        }
        ("cuDeviceTotalMem", 3020, 0) => {
            *pfn = cuDeviceTotalMem_v2 as _;
        }
        ("cuGetProcAddress", _, 0) => {
            *pfn = cuGetProcAddress_v2 as _;
        }
        ("cuGetExportTable", 3000, 0) => {
            *pfn = cuGetExportTable as _;
        }
<<<<<<< HEAD
        ("cuDriverGetVersion", 2020, 0) => {
            *pfn = cuDriverGetVersion as _;
        }
        ("cuGraphRemoveDependencies", 10000, 0) => {
            *pfn = cuGraphRemoveDependencies as _;
        }
        ("cuGraphUpload", 11010, 0) => {
            *pfn = cuGraphUpload as _;
        }
        ("cuGraphUpload", 11010, 2) => {    // v2
            *pfn = cuGraphUpload as _;
        }
        ("cuGraphLaunch", 10000, 0) => {
            *pfn = cuGraphLaunch as _;
        }
        ("cuGraphLaunch", 10000, 2) => {    // v2
            *pfn = cuGraphLaunch as _;
        }
        ("cuGraphInstantiate", 10000, 0) => {
            *pfn = cuGraphInstantiate as _;
        }
        ("cuGraphInstantiate", 11000, 0) => {   // v2
            *pfn = cuGraphInstantiate as _;
        }
        ("cuGraphInstantiateWithFlags", 11040, 0) => {
            *pfn = cuGraphInstantiateWithFlags as _;
        }
        ("cuStreamIsCapturing", 10000, 0) => {
            *pfn = cuStreamIsCapturing as _;
        }
        ("cuStreamIsCapturing", 10000, 2) => {  // v2
            *pfn = cuStreamIsCapturing as _;
        }
        ("cuGraphExecDestroy", 10000, 0) => {
            *pfn = cuGraphExecDestroy as _;
        }
        ("cuGraphDestroy", 10000, 0) => {
            *pfn = cuGraphDestroy as _;
        }
        ("cuStreamEndCapture", 10000, 0) => {
            *pfn = cuStreamEndCapture as _;
        }
        ("cuStreamEndCapture", 10000, 2) => {   // v2
            *pfn = cuStreamEndCapture as _;
        }
        ("cuStreamBeginCapture", 10000, 0) => {
            *pfn = cuStreamBeginCapture as _;
        }
        ("cuStreamBeginCapture", 10000, 2) => { // v2
            *pfn = cuStreamBeginCapture as _;
        }
        ("cuStreamBeginCapture", 10010, 0) => { // v3
            *pfn = cuStreamBeginCapture as _;
        }
        ("cuStreamBeginCapture", 10010, 2) => { // v4
            *pfn = cuStreamBeginCapture as _;
        }
        ("cuStreamGetCaptureInfo", 10010, 0) => {
            *pfn = cuStreamGetCaptureInfo as _;
        }
        ("cuStreamGetCaptureInfo", 10010, 2) => {   // v2
            *pfn = cuStreamGetCaptureInfo as _;
        }
        ("cuStreamGetCaptureInfo", 11030, 0) => {   // v3
            *pfn = cuStreamGetCaptureInfo as _;
        }
        ("cuStreamGetCaptureInfo", 11030, 2) => {   // v4
            *pfn = cuStreamGetCaptureInfo as _;
        }
        ("cuStreamUpdateCaptureDependencies", 11030, 0) => {
            *pfn = cuStreamUpdateCaptureDependencies as _;
        }
        ("cuStreamUpdateCaptureDependencies", 11030, 2) => {    // v2
            *pfn = cuStreamUpdateCaptureDependencies as _;
        }
        ("cuGraphExecKernelNodeSetParams", 12000, 0) => {
            *pfn = cuGraphExecKernelNodeSetParams as _;
        }
        ("cuGraphExecMemsetNodeSetParams", 10020, 0) => {
            *pfn = cuGraphExecMemsetNodeSetParams as _;
=======
        ("cudaGetLastError", 2000, 0) => {
            *pfn = cudaGetLastError as _;
        }
        ("cudaGetErrorString", 2000, 0) => {
            *pfn = cudaGetErrorString as _;
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
        }
        _ => {
            eprintln!(
                "cuGetProcAddress_v2({:?}, {:?}, {}, {}, {:?}) -> {:?}",
                symbol,
                pfn.as_ref(),
                cudaVersion,
                flags,
                status.as_ref(),
                res
            );
            // eprintln!(
            //     ">>> cuGetProcAddress_v2({:?}, {:?}, {}, {}, {:?}) -> {:?}",
            //     symbol,
            //     pfn.as_ref(),
            //     cudaVersion,
            //     flags,
            //     status.as_ref(),
            //     res
            // );
        }
    }

    res
}

pub unsafe extern "C" fn cuInit(flags: c_uint) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(c_uint) -> CUresult> =
        LIBCUDA.get(b"cuInit").unwrap();

    let result = func(flags);
    let actual_flags = 0;
    eprintln!(
        "cuInit(flags: {}) -> {:?}",
        flags,
        result
    );
    eprintln!(
        ">>> cuInit(flags: {}) -> {:?}",
        actual_flags,
        CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuDeviceGet(device: *mut CUdevice, ordinal: c_int) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*mut CUdevice, c_int) -> CUresult> =
        LIBCUDA.get(b"cuDeviceGet").unwrap();

    let result = func(device, ordinal);
    let actual_device: CUdevice = 0;
    let actual_ordinal = 0; // 要获取的设备的索引，从0开始
    eprintln!(
        "cuDeviceGet(device: {:?}, ordinal: {}) -> {:?}",
        device.as_ref(),
        ordinal,
        result
    );
    eprintln!(
        ">>> cuDeviceGet(device: {:?}, ordinal: {}) -> {:?}",
        actual_device,
        actual_ordinal,
        cudaError_enum::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuDeviceTotalMem_v2(_bytes: *mut usize, _dev: CUdevice) -> cudaError_enum {
    if _bytes.is_null() {
        return cudaError_enum::CUDA_ERROR_INVALID_VALUE;
    }
    *_bytes = 8 * 1024 * 1024 * 1024;
    eprintln!(
        ">>> cuDeviceTotalMem_v2(bytes: {}) -> {:?}",
        *_bytes,
        cudaError_enum::CUDA_SUCCESS
    );
    cudaError_enum::CUDA_SUCCESS
}

pub unsafe extern "C" fn cuDeviceGetAttribute(
    pi: *mut c_int,
    attrib: CUdevice_attribute,
    dev: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int, CUdevice_attribute, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceGetAttribute").unwrap();

    let result = func(pi, attrib, dev);
    let actual_pi = 0;
    let actual_attrib: CUdevice_attribute = CUdevice_attribute_enum::CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT;
    let actual_dev: CUdevice = 0;
    eprintln!(
        "cuDeviceGetAttribute(pi: {:?}, attrib: {:?}, dev: {}) -> {:?}",
        pi.as_ref(),
        attrib,
        dev,
        result
    );
    eprintln!(
        ">>> cuDeviceGetAttribute(pi: {:?}, attrib: {:?}, dev: {}) -> {:?}",
        actual_pi,
        actual_attrib,
        actual_dev,
        CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuDeviceGetCount(count: *mut c_int) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*mut c_int) -> CUresult> =
        LIBCUDA.get(b"cuDeviceGetCount").unwrap();

    let result = func(count);
    let actual_count = 1;
    eprintln!(
        "cuDeviceGetCount(count: {:?}) -> {:?}",
        count.as_ref(),
        result
    );
    eprintln!(
        ">>> cuDeviceGetCount(count: {}) -> {:?}",
        actual_count,
        cudaError_enum::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuDeviceGetName(name: *mut c_char, _len: c_int, _dev: CUdevice) -> cudaError_enum {
    if name.is_null() {
        return cudaError_enum::CUDA_ERROR_INVALID_VALUE;
    }
    let actual_name = "OSPP Device";
    eprintln!(
        ">>> cuDeviceGetName(count: {}) -> {:?}",
        actual_name,
        cudaError_enum::CUDA_SUCCESS
    );
    cudaError_enum::CUDA_SUCCESS
}

pub unsafe extern "C" fn cuGetExportTable(
    ppExportTable: *mut *const c_void,
    pExportTableId: *const CUuuid,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut *const c_void, *const CUuuid) -> CUresult,
    > = LIBCUDA.get(b"cuGetExportTable").unwrap();

    let result = func(ppExportTable, pExportTableId);
    let actual_ppExportTable = 0;
    let bytes: [i8; 16] = [0; 16];
    let actual_pExportTableId: CUuuid = CUuuid_st {bytes};
    eprintln!(
        "cuGetExportTable(ppExportTable: {:?}, pExportTableId: {:?}) -> {:?}",
        ppExportTable.as_ref(),
        pExportTableId.as_ref(),
        result
    );
    eprintln!(
<<<<<<< HEAD
        ">>> cuGetExportTable(ppExportTable: {:?}, pExportTableId: {:?}) -> {:?}",
        actual_ppExportTable,
        actual_pExportTableId,
        CUresult::CUDA_SUCCESS
=======
        "cudaMemcpy(dst: {:?}, src: {:?}, count: {:?}, kind: {:?}) -> {:?}",
        dst,
        src,
        count,
        kind,
        result
    );
    result
}

pub unsafe extern "C" fn cudaMalloc(
    dev_ptr: *mut *mut c_void,
    size: usize,
    ) -> cudaError_t {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut *mut c_void, usize) -> cudaError_t,
    > = LIBCUDA.get(b"cudaMalloc_v2").unwrap();
    let result = func(dev_ptr, size);
    eprintln!(
        "cudaMalloc(dev_ptr: {:?}, size: {:?}) -> {:?}",
        dev_ptr.as_ref(),
        size,
        result
    );
    result
}

pub unsafe extern "C" fn cudaFree(
    dev_ptr: *mut c_void,
) -> cudaError_t {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_void) -> cudaError_t,
    > = LIBCUDA.get(b"cudaFree_v2").unwrap();

    let result = func(dev_ptr);
    eprintln!(
        "cudaFree(dev_ptr: {:?}) -> {:?}",
        dev_ptr,
        result
    );
    result
}

pub unsafe extern "C" fn cudaGetDeviceCount(
    count: *mut c_int
) -> cudaError_t {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int) -> cudaError_t,
    > = LIBCUDA.get(b"cudaGetDeviceCount_v2").unwrap();

    let result = func(count);
    eprintln!(
        "cudaGetDeviceCount(count: {:?}) -> {:?}",
        count,
        result
    );
    result
}

pub unsafe extern "C" fn cudaGetDeviceProperties(
    prop: *mut cudaDeviceProp, dev: c_int
) -> cudaError_t {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut cudaDeviceProp, c_int) -> cudaError_t,
    > = LIBCUDA.get(b"cudaGetDeviceProperties").unwrap();

    let result = func(prop, dev);
    eprintln!(
        "cudaGetDeviceProperties(prop: {:?}, dev: {:?}) -> {:?}",
        *prop,
        dev, 
        result
    );
    result
}

pub unsafe extern "C" fn cudaDeviceSynchronize(
    // NULL
) -> cudaError_t {
    let func: libloading::Symbol<
    unsafe extern "C" fn() -> cudaError_t,
    > = LIBCUDA.get(b"cudaDeviceSynchronize_v2").unwrap();

    let result = func();
    eprintln!(
        "cudaDeviceSynchronize() -> {:?}", 
        result
    );
    result
}

// pub unsafe extern "C" fn cudaGetLastError(
//     // NULL
// ) -> cudaError_t {
//     let func: libloading::Symbol<
//     unsafe extern "C" fn() -> cudaError_t,
//     > = LIBCUDA.get(b"cudaGetLastError").unwrap();
    
//     let result = func();
//     eprintln!(
//         "cudaGetLastError() -> {:?}", 
//         result
//     );
//     result
// }

// pub unsafe extern "C" fn cudaGetErrorString(
//     error: cudaError_t
// ) -> *const c_char {
//     let func: libloading::Symbol<
//     unsafe extern "C" fn(cudaError_t) -> *const c_char,
//     > = LIBCUDA.get(b"cudaGetErrorString").unwrap();
//     let result = func(error);
//     eprintln!(
//         "cudaGetErrorString(error: {:?}) -> {:?}",
//         error,
//         CStr::from_ptr(result)
//     );
//     result
// }

pub unsafe extern "C" fn cuDeviceGetP2PAttribute(
    value: *mut c_int,
    attrib: CUdevice_P2PAttribute,
    srcDevice: CUdevice,
    dstDevice: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int, CUdevice_P2PAttribute, CUdevice, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceGetP2PAttribute").unwrap();

    let result = func(value, attrib, srcDevice, dstDevice);
    eprintln!(
        "cuDeviceGetP2PAttribute(value: {:?}, attrib: {:?}, srcDevice: {:?}, dstDevice: {:?}) -> {:?}",
        value,
        attrib,
        srcDevice,
        dstDevice,
        result
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    );
    result
}

pub unsafe extern "C" fn cuDriverGetVersion(
    version: *mut c_int,
) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*mut c_int) -> CUresult> =
        LIBCUDA.get(b"cuDriverGetVersion").unwrap();

    let result = func(version);
    let actual_version = 12.2;
    eprintln!(
        "cuDriverGetVersion(driverVersion: {:?}) -> {:?}",
        version,
        result
    );
    eprintln!(
        ">>> cuDriverGetVersion(driverVersion: {:?}) -> {:?}",
        actual_version,
        CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuGraphRemoveDependencies(
    hGraph: CUgraph,
    from: *const CUgraphNode,
    to: *const CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, *const CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphRemoveDependencies").unwrap();

    // 不透明的结构体，我们先用传入的参数来为其赋值
    let result = func(hGraph, from, to, numDependencies);
    let actual_hGraph: CUgraph = hGraph;
    let actual_from: *const CUgraphNode = from;
    let actual_to: *const CUgraphNode = to;
    let actual_numDependencies: size_t = numDependencies + 1;
    eprintln!(
        "cuGraphRemoveDependencies(hGraph: {:?}, from: {:?}, to: {:?}, numDependencies: {:?}) -> {:?}",
        hGraph, from, to, numDependencies, result
    );
    eprintln!(
        ">>> cuGraphRemoveDependencies(hGraph: {:?}, from: {:?}, to: {:?}, numDependencies: {:?}) -> {:?}",
        actual_hGraph, actual_from, actual_to, actual_numDependencies, CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuGraphDestroyNode(hNode: CUgraphNode) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUgraphNode) -> CUresult> =
        LIBCUDA.get(b"cuGraphDestroyNode").unwrap();

    let result = func(hNode);
    let actual_hNode: CUgraphNode = hNode;
    eprintln!("cuGraphDestroyNode(hNode: {:?}) -> {:?}", hNode, result);
    eprintln!(">>> cuGraphDestroyNode(hNode: {:?}) -> {:?}", actual_hNode, CUresult::CUDA_SUCCESS);
    result
}

pub unsafe extern "C" fn cuGraphInstantiate(
    phGraphExec: *mut CUgraphExec,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphExec, CUgraph, *mut CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiate").unwrap();

    let result = func(phGraphExec, hGraph, dependencies, numDependencies);
    let actual_phGraphExec: CUgraphExec = *phGraphExec;
    let actual_hGraph: CUgraph = hGraph;
    let actual_dependencies: *mut CUgraphNode = dependencies;
    let actual_numDependencies: size_t = numDependencies + 1;
    eprintln!(
        "cuGraphInstantiate(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        phGraphExec, hGraph, dependencies, numDependencies, result
    );
    eprintln!(
        ">>> cuGraphInstantiate(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        actual_phGraphExec, actual_hGraph, actual_dependencies, actual_numDependencies, CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuGraphInstantiateWithFlags(
    phGraphExec: *mut CUgraphExec,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: size_t,
    flags: CUgraphExec_st,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *mut CUgraphExec,
            CUgraph,
            *mut CUgraphNode,
            size_t,
            CUgraphExec_st,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiateWithFlags").unwrap();

    let result = func(phGraphExec, hGraph, dependencies, numDependencies, flags);
    let actual_phGraphExec: CUgraphExec = *phGraphExec;
    let actual_hGraph: CUgraph = hGraph;
    let actual_dependencies: *mut CUgraphNode = dependencies;
    let actual_numDependencies: size_t = numDependencies + 1;
    let actual_flags: CUgraphExec_st = flags;
    eprintln!(
        "cuGraphInstantiateWithFlags(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, flags: {:?}) -> {:?}",
        phGraphExec, hGraph, dependencies, numDependencies, flags, result
    );
    eprintln!(
        ">>> cuGraphInstantiateWithFlags(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, flags: {:?}) -> {:?}",
        actual_phGraphExec, actual_hGraph, actual_dependencies, actual_numDependencies, actual_flags, CUresult::CUDA_SUCCESS
    );
    result
}
pub unsafe extern "C" fn cuGraphUpload(
    hGraphExec: CUgraphExec,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphUpload").unwrap();

    let result = func(hGraphExec, stream);
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_stream: CUstream = stream;
    eprintln!(
        "cuGraphUpload(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        hGraphExec, stream, result
    );
    eprintln!(
        ">>> cuGraphUpload(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        actual_hGraphExec, actual_stream, CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuGraphLaunch(
    hGraphExec: CUgraphExec,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphLaunch").unwrap();

    let result = func(hGraphExec, stream);
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_stream: CUstream = stream;
    eprintln!(
        "cuGraphLaunch(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        hGraphExec, stream, result
    );
    eprintln!(
        ">>> cuGraphLaunch(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        actual_hGraphExec, actual_stream, CUresult::CUDA_SUCCESS
    );
    result
}

<<<<<<< HEAD
=======
pub unsafe extern "C" fn cuMemFreeAsync(
    dptr: CUdeviceptr,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemFreeAsync").unwrap();

    let result = func(dptr, stream);
    eprintln!(
        "cuMemFreeAsync(dptr: {:?}, stream: {:?}) -> {:?}",
        dptr, stream, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolTrimTo(
    pool: CUmemoryPool,
    bytesize: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUmemoryPool, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolTrimTo").unwrap();

    let result = func(pool, bytesize);
    eprintln!(
        "cuMemPoolTrimTo(pool: {:?}, bytesize: {:?}) -> {:?}",
        pool, bytesize, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolSetAttribute(
    pool: CUmemoryPool,
    value: *const std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUmemoryPool, *const std::ffi::c_void) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolSetAttribute").unwrap();

    let result = func(pool, value);
    eprintln!(
        "cuMemPoolSetAttribute(pool: {:?}, value: {:?}) -> {:?}",
        pool, value, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolGetAccess(
    pool: CUmemoryPool,
    peer: CUdevice,
    flags: CUmemAccess_flags,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUmemoryPool, CUdevice, CUmemAccess_flags) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolGetAccess").unwrap();

    let result = func(pool, peer, flags);
    eprintln!(
        "cuMemPoolGetAccess(pool: {:?}, peer: {:?}, flags: {:?}) -> {:?}",
        pool, peer, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolCreate(
    pool: *mut CUmemoryPool,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUmemoryPool, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolCreate").unwrap();

    let result = func(pool, flags);
    eprintln!(
        "cuMemPoolCreate(pool: {:?}, flags: {:?}) -> {:?}",
        pool, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolDestroy(
    pool: CUmemoryPool,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUmemoryPool) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolDestroy").unwrap();

    let result = func(pool);
    eprintln!(
        "cuMemPoolDestroy(pool: {:?}) -> {:?}",
        pool, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolExportToShareableHandle(
    pool: CUmemoryPool,
    memHandle: *mut CUmemPoolHandle_st,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUmemoryPool, *mut CUmemPoolHandle_st, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolExportToShareableHandle").unwrap();

    let result = func(pool, memHandle, flags);
    eprintln!(
        "cuMemPoolExportToShareableHandle(pool: {:?}, memHandle: {:?}, flags: {:?}) -> {:?}",
        pool, memHandle, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolImportFromShareableHandle(
    pool: *mut CUmemoryPool,
    memHandle: CUmemPoolHandle_st,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUmemoryPool, CUmemPoolHandle_st, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolImportFromShareableHandle").unwrap();

    let result = func(pool, memHandle, flags);
    eprintln!(
        "cuMemPoolImportFromShareableHandle(pool: {:?}, memHandle: {:?}, flags: {:?}) -> {:?}",
        pool, memHandle, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolExportPointer(
    poolPtrExported: *mut CUdeviceptr,
    basePtr: CUdeviceptr,
    pool: CUmemoryPool,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, CUdeviceptr, CUmemoryPool, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolExportPointer").unwrap();

    let result = func(poolPtrExported, basePtr, pool, flags);
    eprintln!(
        "cuMemPoolExportPointer(poolPtrExported: {:?}, basePtr: {:?}, pool: {:?}, flags: {:?}) -> {:?}",
        poolPtrExported, basePtr, pool, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemPoolImportPointer(
    basePtr: *mut CUdeviceptr,
    pool: CUmemoryPool,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, CUmemoryPool, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMemPoolImportPointer").unwrap();

    let result = func(basePtr, pool, flags);
    eprintln!(
        "cuMemPoolImportPointer(basePtr: {:?}, pool: {:?}, flags: {:?}) -> {:?}",
        basePtr, pool, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpy(
    dst: CUdeviceptr,
    src: CUdeviceptr,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUdeviceptr, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpy").unwrap();

    let result = func(dst, src, byteCount);
    eprintln!(
        "cuMemcpy(dst: {:?}, src: {:?}, byteCount: {:?}) -> {:?}",
        dst, src, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyAsync(
    dst: CUdeviceptr,
    src: CUdeviceptr,
    byteCount: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUdeviceptr, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyAsync").unwrap();

    let result = func(dst, src, byteCount, hStream);
    eprintln!(
        "cuMemcpyAsync(dst: {:?}, src: {:?}, byteCount: {:?}, hStream: {:?}) -> {:?}",
        dst, src, byteCount, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyPeer(
    dstDevice: CUdeviceptr,
    dstContext: CUcontext,
    srcDevice: CUdeviceptr,
    srcContext: CUcontext,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUcontext, CUdeviceptr, CUcontext, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyPeer").unwrap();

    let result = func(dstDevice, dstContext, srcDevice, srcContext, byteCount);
    eprintln!(
        "cuMemcpyPeer(dstDevice: {:?}, dstContext: {:?}, srcDevice: {:?}, srcContext: {:?}, byteCount: {:?}) -> {:?}",
        dstDevice, dstContext, srcDevice, srcContext, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyPeerAsync(
    dstDevice: CUdeviceptr,
    dstContext: CUcontext,
    srcDevice: CUdeviceptr,
    srcContext: CUcontext,
    byteCount: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUcontext, CUdeviceptr, CUcontext, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyPeerAsync").unwrap();

    let result = func(dstDevice, dstContext, srcDevice, srcContext, byteCount, hStream);
    eprintln!(
        "cuMemcpyPeerAsync(dstDevice: {:?}, dstContext: {:?}, srcDevice: {:?}, srcContext: {:?}, byteCount: {:?}, hStream: {:?}) -> {:?}",
        dstDevice, dstContext, srcDevice, srcContext, byteCount, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyHtoD(
    dstDevice: CUdeviceptr,
    srcHost: *const std::ffi::c_void,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, *const std::ffi::c_void, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyHtoD").unwrap();

    let result = func(dstDevice, srcHost, byteCount);
    eprintln!(
        "cuMemcpyHtoD(dstDevice: {:?}, srcHost: {:?}, byteCount: {:?}) -> {:?}",
        dstDevice, srcHost, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyHtoDAsync(
    dstDevice: CUdeviceptr,
    srcHost: *const std::ffi::c_void,
    byteCount: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, *const std::ffi::c_void, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyHtoDAsync").unwrap();

    let result = func(dstDevice, srcHost, byteCount, hStream);
    eprintln!(
        "cuMemcpyHtoDAsync(dstDevice: {:?}, srcHost: {:?}, byteCount: {:?}, hStream: {:?}) -> {:?}",
        dstDevice, srcHost, byteCount, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyDtoH(
    dstHost: *mut std::ffi::c_void,
    srcDevice: CUdeviceptr,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut std::ffi::c_void, CUdeviceptr, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyDtoH").unwrap();

    let result = func(dstHost, srcDevice, byteCount);
    eprintln!(
        "cuMemcpyDtoH(dstHost: {:?}, srcDevice: {:?}, byteCount: {:?}) -> {:?}",
        dstHost, srcDevice, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyDtoHAsync(
    dstHost: *mut std::ffi::c_void,
    srcDevice: CUdeviceptr,
    byteCount: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut std::ffi::c_void, CUdeviceptr, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyDtoHAsync").unwrap();

    let result = func(dstHost, srcDevice, byteCount, hStream);
    eprintln!(
        "cuMemcpyDtoHAsync(dstHost: {:?}, srcDevice: {:?}, byteCount: {:?}, hStream: {:?}) -> {:?}",
        dstHost, srcDevice, byteCount, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpyDtoD(
    dstDevice: CUdeviceptr,
    srcDevice: CUdeviceptr,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, CUdeviceptr, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpyDtoD").unwrap();

    let result = func(dstDevice, srcDevice, byteCount);
    eprintln!(
        "cuMemcpyDtoD(dstDevice: {:?}, srcDevice: {:?}, byteCount: {:?}) -> {:?}",
        dstDevice, srcDevice, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpy2DUnaligned(
    pCopy: *const CUDA_MEMCPY2D,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*const CUDA_MEMCPY2D) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpy2DUnaligned").unwrap();

    let result = func(pCopy);
    eprintln!(
        "cuMemcpy2DUnaligned(pCopy: {:?}) -> {:?}",
        pCopy, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpy2DAsync(
    pCopy: *const CUDA_MEMCPY2D,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*const CUDA_MEMCPY2D, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpy2DAsync").unwrap();

    let result = func(pCopy, hStream);
    eprintln!(
        "cuMemcpy2DAsync(pCopy: {:?}, hStream: {:?}) -> {:?}",
        pCopy, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpy3D(
    pCopy: *const CUDA_MEMCPY3D) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*const CUDA_MEMCPY3D) -> CUresult> =
        LIBCUDA.get(b"cuMemcpy3D").unwrap();

    let result = func(pCopy);
    eprintln!("cuMemcpy3D(pCopy: {:?}) -> {:?}", pCopy, result);
    result
}

pub unsafe extern "C" fn cuMemcpy3DAsync(
    pCopy: *const CUDA_MEMCPY3D,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*const CUDA_MEMCPY3D, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpy3DAsync").unwrap();

    let result = func(pCopy, hStream);
    eprintln!(
        "cuMemcpy3DAsync(pCopy: {:?}, hStream: {:?}) -> {:?}",
        pCopy, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemcpy3DPeer(
    pCopy: *const CUDA_MEMCPY3D_PEER) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*const CUDA_MEMCPY3D_PEER) -> CUresult> =
        LIBCUDA.get(b"cuMemcpy3DPeer").unwrap();

    let result = func(pCopy);
    eprintln!("cuMemcpy3DPeer(pCopy: {:?}) -> {:?}", pCopy, result);
    result
}

pub unsafe extern "C" fn cuMemcpy3DPeerAsync(
    pCopy: *const CUDA_MEMCPY3D_PEER,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*const CUDA_MEMCPY3D_PEER, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemcpy3DPeerAsync").unwrap();

    let result = func(pCopy, hStream);
    eprintln!(
        "cuMemcpy3DPeerAsync(pCopy: {:?}, hStream: {:?}) -> {:?}",
        pCopy, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemsetD8(
    dstDevice: CUdeviceptr,
    value: u8,
    byteCount: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, u8, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemsetD8").unwrap();

    let result = func(dstDevice, value, byteCount);
    eprintln!(
        "cuMemsetD8(dstDevice: {:?}, value: {:?}, byteCount: {:?}) -> {:?}",
        dstDevice, value, byteCount, result
    );
    result
}

pub unsafe extern "C" fn cuMemsetD8Async(
    dstDevice: CUdeviceptr,
    value: u8,
    byteCount: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, u8, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemsetD8Async").unwrap();

    let result = func(dstDevice, value, byteCount, hStream);
    eprintln!(
        "cuMemsetD8Async(dstDevice: {:?}, value: {:?}, byteCount: {:?}, hStream: {:?}) -> {:?}",
        dstDevice, value, byteCount, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuMemsetD2D8(
    dstDevice: CUdeviceptr,
    dstPitch: usize,
    value: u8,
    width: usize,
    height: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, usize, u8, usize, usize) -> CUresult,
    > = LIBCUDA.get(b"cuMemsetD2D8").unwrap();

    let result = func(dstDevice, dstPitch, value, width, height);
    eprintln!(
        "cuMemsetD2D8(dstDevice: {:?}, dstPitch: {:?}, value: {:?}, width: {:?}, height: {:?}) -> {:?}",
        dstDevice, dstPitch, value, width, height, result
    );
    result
}

pub unsafe extern "C" fn cuMemsetD2D8Async(
    dstDevice: CUdeviceptr,
    dstPitch: usize,
    value: u8,
    width: usize,
    height: usize,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, usize, u8, usize, usize, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemsetD2D8Async").unwrap();

    let result = func(dstDevice, dstPitch, value, width, height, hStream);
    eprintln!(
        "cuMemsetD2D8Async(dstDevice: {:?}, dstPitch: {:?}, value: {:?}, width: {:?}, height: {:?}, hStream: {:?}) -> {:?}",
        dstDevice, dstPitch, value, width, height, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuFuncSetCacheConfig(
    hfunc: CUfunction,
    cacheConfig: CUfunc_cache,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUfunction, CUfunc_cache) -> CUresult,
    > = LIBCUDA.get(b"cuFuncSetCacheConfig").unwrap();

    let result = func(hfunc, cacheConfig);
    eprintln!(
        "cuFuncSetCacheConfig(hfunc: {:?}, cacheConfig: {:?}) -> {:?}",
        hfunc, cacheConfig, result
    );
    result
}

pub unsafe extern "C" fn cuFuncSetSharedMemConfig(
    hfunc: CUfunction,
    config: CUsharedconfig,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUfunction, CUsharedconfig) -> CUresult,
    > = LIBCUDA.get(b"cuFuncSetSharedMemConfig").unwrap();

    let result = func(hfunc, config);
    eprintln!(
        "cuFuncSetSharedMemConfig(hfunc: {:?}, config: {:?}) -> {:?}",
        hfunc, config, result
    );
    result
}

pub unsafe extern "C" fn cuFuncGetAttribute(
    pi: *mut i32,
    attrib: CUfunction_attribute,
    hfunc: CUfunction,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut i32, CUfunction_attribute, CUfunction) -> CUresult,
    > = LIBCUDA.get(b"cuFuncGetAttribute").unwrap();

    let result = func(pi, attrib, hfunc);
    eprintln!(
        "cuFuncGetAttribute(pi: {:?}, attrib: {:?}, hfunc: {:?}) -> {:?}",
        pi, attrib, hfunc, result
    );
    result
}

pub unsafe extern "C" fn cuFuncSetAttribute(
    hfunc: CUfunction,
    attrib: CUfunction_attribute,
    value: i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUfunction, CUfunction_attribute, i32) -> CUresult,
    > = LIBCUDA.get(b"cuFuncSetAttribute").unwrap();

    let result = func(hfunc, attrib, value);
    eprintln!(
        "cuFuncSetAttribute(hfunc: {:?}, attrib: {:?}, value: {:?}) -> {:?}",
        hfunc, attrib, value, result
    );
    result
}

pub unsafe extern "C" fn cuArrayCreate(
    pHandle: *mut CUarray,
    pAllocateArray: *const CUDA_ARRAY_DESCRIPTOR,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUarray, *const CUDA_ARRAY_DESCRIPTOR) -> CUresult,
    > = LIBCUDA.get(b"cuArrayCreate").unwrap();

    let result = func(pHandle, pAllocateArray);
    eprintln!(
        "cuArrayCreate(pHandle: {:?}, pAllocateArray: {:?}) -> {:?}",
        pHandle, pAllocateArray, result
    );
    result
}

pub unsafe extern "C" fn cuArrayGetDescriptor(
    pArrayDescriptor: *mut CUDA_ARRAY_DESCRIPTOR,
    hArray: CUarray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_ARRAY_DESCRIPTOR, CUarray) -> CUresult,
    > = LIBCUDA.get(b"cuArrayGetDescriptor").unwrap();

    let result = func(pArrayDescriptor, hArray);
    eprintln!(
        "cuArrayGetDescriptor(pArrayDescriptor: {:?}, hArray: {:?}) -> {:?}",
        pArrayDescriptor, hArray, result
    );
    result
}

pub unsafe extern "C" fn cuArrayGetSparseProperties(
    pSparseProperties: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
    hArray: CUarray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_ARRAY_SPARSE_PROPERTIES, CUarray) -> CUresult,
    > = LIBCUDA.get(b"cuArrayGetSparseProperties").unwrap();

    let result = func(pSparseProperties, hArray);
    eprintln!(
        "cuArrayGetSparseProperties(pSparseProperties: {:?}, hArray: {:?}) -> {:?}",
        pSparseProperties, hArray, result
    );
    result
}

pub unsafe extern "C" fn cuArrayGetPlane(
    pPlaneArray: *mut CUarray,
    hArray: CUarray,
    planeIdx: usize,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUarray, CUarray, usize) -> CUresult,
    > = LIBCUDA.get(b"cuArrayGetPlane").unwrap();

    let result = func(pPlaneArray, hArray, planeIdx);
    eprintln!(
        "cuArrayGetPlane(pPlaneArray: {:?}, hArray: {:?}, planeIdx: {:?}) -> {:?}",
        pPlaneArray, hArray, planeIdx, result
    );
    result
}

pub unsafe extern "C" fn cuArray3DCreate(
    pHandle: *mut CUarray,
    pAllocateArray: *const CUDA_ARRAY3D_DESCRIPTOR,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUarray, *const CUDA_ARRAY3D_DESCRIPTOR) -> CUresult,
    > = LIBCUDA.get(b"cuArray3DCreate").unwrap();

    let result = func(pHandle, pAllocateArray);
    eprintln!(
        "cuArray3DCreate(pHandle: {:?}, pAllocateArray: {:?}) -> {:?}",
        pHandle, pAllocateArray, result
    );
    result
}

pub unsafe extern "C" fn cuArray3DGetDescriptor(
    pArrayDescriptor: *mut CUDA_ARRAY3D_DESCRIPTOR,
    hArray: CUarray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_ARRAY3D_DESCRIPTOR, CUarray) -> CUresult,
    > = LIBCUDA.get(b"cuArray3DGetDescriptor").unwrap();

    let result = func(pArrayDescriptor, hArray);
    eprintln!(
        "cuArray3DGetDescriptor(pArrayDescriptor: {:?}, hArray: {:?}) -> {:?}",
        pArrayDescriptor, hArray, result
    );
    result
}

pub unsafe extern "C" fn cuArrayDestroy(hArray: CUarray) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUarray) -> CUresult> =
        LIBCUDA.get(b"cuArrayDestroy").unwrap();

    let result = func(hArray);
    eprintln!("cuArrayDestroy(hArray: {:?}) -> {:?}", hArray, result);
    result
}

pub unsafe extern "C" fn cuMipmappedArrayCreate(
    pHandle: *mut CUmipmappedArray,
    pMipmappedArrayDesc: *const CUDA_ARRAY3D_DESCRIPTOR,
    numMipmapLevels: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUmipmappedArray, *const CUDA_ARRAY3D_DESCRIPTOR, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMipmappedArrayCreate").unwrap();

    let result = func(pHandle, pMipmappedArrayDesc, numMipmapLevels);
    eprintln!(
        "cuMipmappedArrayCreate(pHandle: {:?}, pMipmappedArrayDesc: {:?}, numMipmapLevels: {:?}) -> {:?}",
        pHandle, pMipmappedArrayDesc, numMipmapLevels, result
    );
    result
}

pub unsafe extern "C" fn cuMipmappedArrayGetLevel(
    pLevelArray: *mut CUarray,
    hMipmappedArray: CUmipmappedArray,
    level: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUarray, CUmipmappedArray, u32) -> CUresult,
    > = LIBCUDA.get(b"cuMipmappedArrayGetLevel").unwrap();

    let result = func(pLevelArray, hMipmappedArray, level);
    eprintln!(
        "cuMipmappedArrayGetLevel(pLevelArray: {:?}, hMipmappedArray: {:?}, level: {:?}) -> {:?}",
        pLevelArray, hMipmappedArray, level, result
    );
    result
}

pub unsafe extern "C" fn cuMipmappedArrayGetSparseProperties(
    pSparseProperties: *mut CUDA_ARRAY_SPARSE_PROPERTIES,
    hMipmappedArray: CUmipmappedArray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_ARRAY_SPARSE_PROPERTIES, CUmipmappedArray) -> CUresult,
    > = LIBCUDA.get(b"cuMipmappedArrayGetSparseProperties").unwrap();

    let result = func(pSparseProperties, hMipmappedArray);
    eprintln!(
        "cuMipmappedArrayGetSparseProperties(pSparseProperties: {:?}, hMipmappedArray: {:?}) -> {:?}",
        pSparseProperties, hMipmappedArray, result
    );
    result
}

pub unsafe extern "C" fn cuMipmappedArrayDestroy(hMipmappedArray: CUmipmappedArray) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUmipmappedArray) -> CUresult> =
        LIBCUDA.get(b"cuMipmappedArrayDestroy").unwrap();

    let result = func(hMipmappedArray);
    eprintln!(
        "cuMipmappedArrayDestroy(hMipmappedArray: {:?}) -> {:?}",
        hMipmappedArray, result
    );
    result
}

pub unsafe extern "C" fn cuArrayGetMemoryRequirements(
    pMemoryRequirements: *mut size_t,
    hArray: CUarray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut size_t, CUarray) -> CUresult,
    > = LIBCUDA.get(b"cuArrayGetMemoryRequirements").unwrap();

    let result = func(pMemoryRequirements, hArray);
    eprintln!(
        "cuArrayGetMemoryRequirements(pMemoryRequirements: {:?}, hArray: {:?}) -> {:?}",
        pMemoryRequirements, hArray, result
    );
    result
}

pub unsafe extern "C" fn cuMipmappedArrayGetMemoryRequirements(
    pMemoryRequirements: *mut size_t,
    hMipmappedArray: CUmipmappedArray,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut size_t, CUmipmappedArray) -> CUresult,
    > = LIBCUDA.get(b"cuMipmappedArrayGetMemoryRequirements").unwrap();

    let result = func(pMemoryRequirements, hMipmappedArray);
    eprintln!(
        "cuMipmappedArrayGetMemoryRequirements(pMemoryRequirements: {:?}, hMipmappedArray: {:?}) -> {:?}",
        pMemoryRequirements, hMipmappedArray, result
    );
    result
}

pub unsafe extern "C" fn cuTexObjectCreate(
    pTexObject: *mut CUtexObject,
    pResDesc: *const CUDA_RESOURCE_DESC,
    pTexDesc: *const CUDA_TEXTURE_DESC,
    pResViewDesc: *const CUDA_RESOURCE_VIEW_DESC,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUtexObject, *const CUDA_RESOURCE_DESC, *const CUDA_TEXTURE_DESC, *const CUDA_RESOURCE_VIEW_DESC) -> CUresult,
    > = LIBCUDA.get(b"cuTexObjectCreate").unwrap();

    let result = func(pTexObject, pResDesc, pTexDesc, pResViewDesc);
    eprintln!(
        "cuTexObjectCreate(pTexObject: {:?}, pResDesc: {:?}, pTexDesc: {:?}, pResViewDesc: {:?}) -> {:?}",
        pTexObject, pResDesc, pTexDesc, pResViewDesc, result
    );
    result
}

pub unsafe extern "C" fn cuTexObjectDestroy(hTexObject: CUtexObject) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUtexObject) -> CUresult> =
        LIBCUDA.get(b"cuTexObjectDestroy").unwrap();

    let result = func(hTexObject);
    eprintln!("cuTexObjectDestroy(hTexObject: {:?}) -> {:?}", hTexObject, result);
    result
}

pub unsafe extern "C" fn cuTexObjectGetResourceDesc(
    pResDesc: *mut CUDA_RESOURCE_DESC,
    hTexObject: CUtexObject,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_RESOURCE_DESC, CUtexObject) -> CUresult,
    > = LIBCUDA.get(b"cuTexObjectGetResourceDesc").unwrap();

    let result = func(pResDesc, hTexObject);
    eprintln!(
        "cuTexObjectGetResourceDesc(pResDesc: {:?}, hTexObject: {:?}) -> {:?}",
        pResDesc, hTexObject, result
    );
    result
}

pub unsafe extern "C" fn cuTexObjectGetTextureDesc(
    pTexDesc: *mut CUDA_TEXTURE_DESC,
    hTexObject: CUtexObject,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_TEXTURE_DESC, CUtexObject) -> CUresult,
    > = LIBCUDA.get(b"cuTexObjectGetTextureDesc").unwrap();

    let result = func(pTexDesc, hTexObject);
    eprintln!(
        "cuTexObjectGetTextureDesc(pTexDesc: {:?}, hTexObject: {:?}) -> {:?}",
        pTexDesc, hTexObject, result
    );
    result
}

pub unsafe extern "C" fn cuTexObjectGetResourceViewDesc(
    pResViewDesc: *mut CUDA_RESOURCE_VIEW_DESC,
    hTexObject: CUtexObject,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_RESOURCE_VIEW_DESC, CUtexObject) -> CUresult,
    > = LIBCUDA.get(b"cuTexObjectGetResourceViewDesc").unwrap();

    let result = func(pResViewDesc, hTexObject);
    eprintln!(
        "cuTexObjectGetResourceViewDesc(pResViewDesc: {:?}, hTexObject: {:?}) -> {:?}",
        pResViewDesc, hTexObject, result
    );
    result
}

pub unsafe extern "C" fn cuSurfObjectCreate(
    pSurfObject: *mut CUsurfObject,
    pResDesc: *const CUDA_RESOURCE_DESC,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUsurfObject, *const CUDA_RESOURCE_DESC) -> CUresult,
    > = LIBCUDA.get(b"cuSurfObjectCreate").unwrap();

    let result = func(pSurfObject, pResDesc);
    eprintln!(
        "cuSurfObjectCreate(pSurfObject: {:?}, pResDesc: {:?}) -> {:?}",
        pSurfObject, pResDesc, result
    );
    result
}

pub unsafe extern "C" fn cuSurfObjectDestroy(hSurfObject: CUsurfObject) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUsurfObject) -> CUresult> =
        LIBCUDA.get(b"cuSurfObjectDestroy").unwrap();

    let result = func(hSurfObject);
    eprintln!("cuSurfObjectDestroy(hSurfObject: {:?}) -> {:?}", hSurfObject, result);
    result
}

pub unsafe extern "C" fn cuSurfObjectGetResourceDesc(
    pResDesc: *mut CUDA_RESOURCE_DESC,
    hSurfObject: CUsurfObject,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUDA_RESOURCE_DESC, CUsurfObject) -> CUresult,
    > = LIBCUDA.get(b"cuSurfObjectGetResourceDesc").unwrap();

    let result = func(pResDesc, hSurfObject);
    eprintln!(
        "cuSurfObjectGetResourceDesc(pResDesc: {:?}, hSurfObject: {:?}) -> {:?}",
        pResDesc, hSurfObject, result
    );
    result
}

pub unsafe extern "C" fn cuImportExternalMemory(
    extMemoryOut: *mut CUexternalMemory,
    memImportDesc: *const CUDA_EXTERNAL_MEMORY_HANDLE_DESC,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUexternalMemory, *const CUDA_EXTERNAL_MEMORY_HANDLE_DESC) -> CUresult,
    > = LIBCUDA.get(b"cuImportExternalMemory").unwrap();

    let result = func(extMemoryOut, memImportDesc);
    eprintln!(
        "cuImportExternalMemory(extMemoryOut: {:?}, memImportDesc: {:?}) -> {:?}",
        extMemoryOut, memImportDesc, result
    );
    result
}

pub unsafe extern "C" fn cuExternalMemoryGetMappedBuffer(
    pSize: *mut usize,
    extMemory: CUexternalMemory,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut usize, CUexternalMemory, u32) -> CUresult,
    > = LIBCUDA.get(b"cuExternalMemoryGetMappedBuffer").unwrap();

    let result = func(pSize, extMemory, flags);
    eprintln!(
        "cuExternalMemoryGetMappedBuffer(pSize: {:?}, extMemory: {:?}, flags: {:?}) -> {:?}",
        pSize, extMemory, flags, result
    );
    result
}

pub unsafe extern "C" fn cuExternalMemoryGetMappedMipmappedArray(
    pMipmappedArray: *mut CUmipmappedArray,
    extMemory: CUexternalMemory,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUmipmappedArray, CUexternalMemory, u32) -> CUresult,
    > = LIBCUDA.get(b"cuExternalMemoryGetMappedMipmappedArray").unwrap();

    let result = func(pMipmappedArray, extMemory, flags);
    eprintln!(
        "cuExternalMemoryGetMappedMipmappedArray(pMipmappedArray: {:?}, extMemory: {:?}, flags: {:?}) -> {:?}",
        pMipmappedArray, extMemory, flags, result
    );
    result
}

pub unsafe extern "C" fn cuDestroyExternalMemory(extMemory: CUexternalMemory) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUexternalMemory) -> CUresult> =
        LIBCUDA.get(b"cuDestroyExternalMemory").unwrap();

    let result = func(extMemory);
    eprintln!("cuDestroyExternalMemory(extMemory: {:?}) -> {:?}", extMemory, result);
    result
}

pub unsafe extern "C" fn cuImportExternalSemaphore(
    extSemaphoreOut: *mut CUexternalSemaphore,
    semImportDesc: *const CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUexternalSemaphore, *const CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC) -> CUresult,
    > = LIBCUDA.get(b"cuImportExternalSemaphore").unwrap();

    let result = func(extSemaphoreOut, semImportDesc);
    eprintln!(
        "cuImportExternalSemaphore(extSemaphoreOut: {:?}, semImportDesc: {:?}) -> {:?}",
        extSemaphoreOut, semImportDesc, result
    );
    result
}

pub unsafe extern "C" fn cuSignalExternalSemaphoresAsync(
    extSemaphores: *const CUexternalSemaphore,
    params: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    numExtSemaphores: u32,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *const CUexternalSemaphore,
            *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
            u32,
            CUstream,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuSignalExternalSemaphoresAsync").unwrap();

    let result = func(extSemaphores, params, numExtSemaphores, stream);
    eprintln!(
        "cuSignalExternalSemaphoresAsync(extSemaphores: {:?}, params: {:?}, numExtSemaphores: {:?}, stream: {:?}) -> {:?}",
        extSemaphores, params, numExtSemaphores, stream, result
    );
    result
}

pub unsafe extern "C" fn cuWaitExternalSemaphoresAsync(
    extSemaphores: *const CUexternalSemaphore,
    params: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    numExtSemaphores: u32,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *const CUexternalSemaphore,
            *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
            u32,
            CUstream,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuWaitExternalSemaphoresAsync").unwrap();

    let result = func(extSemaphores, params, numExtSemaphores, stream);
    eprintln!(
        "cuWaitExternalSemaphoresAsync(extSemaphores: {:?}, params: {:?}, numExtSemaphores: {:?}, stream: {:?}) -> {:?}",
        extSemaphores, params, numExtSemaphores, stream, result
    );
    result
}

pub unsafe extern "C" fn cuDestroyExternalSemaphore(extSemaphore: CUexternalSemaphore) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUexternalSemaphore) -> CUresult> =
        LIBCUDA.get(b"cuDestroyExternalSemaphore").unwrap();

    let result = func(extSemaphore);
    eprintln!("cuDestroyExternalSemaphore(extSemaphore: {:?}) -> {:?}", extSemaphore, result);
    result
}

pub unsafe extern "C" fn cuDeviceGetNvSciSyncAttributes(
    device: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceGetNvSciSyncAttributes").unwrap();

    let result = func(device);
    eprintln!(
        "cuDeviceGetNvSciSyncAttributes(device: {:?}) -> {:?}",
        device, result
    );
    result
}

pub unsafe extern "C" fn cuLaunchKernel(
    f: CUfunction,
    gridDimX: u32,
    gridDimY: u32,
    gridDimZ: u32,
    blockDimX: u32,
    blockDimY: u32,
    blockDimZ: u32,
    sharedMemBytes: u32,
    hStream: CUstream,
    kernelParams: *const *const std::ffi::c_void,
    extra: *mut *mut std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            CUfunction,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            CUstream,
            *const *const std::ffi::c_void,
            *mut *mut std::ffi::c_void,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuLaunchKernel").unwrap();

    let result = func(
        f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, hStream,
        kernelParams, extra,
    );
    eprintln!(
        "cuLaunchKernel(f: {:?}, gridDimX: {:?}, gridDimY: {:?}, gridDimZ: {:?}, blockDimX: {:?}, blockDimY: {:?}, blockDimZ: {:?}, sharedMemBytes: {:?}, hStream: {:?}, kernelParams: {:?}, extra: {:?}) -> {:?}",
        f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, hStream, kernelParams, extra, result
    );
    result
}

pub unsafe extern "C" fn cuLaunchCooperativeKernel(
    f: CUfunction,
    gridDimX: u32,
    gridDimY: u32,
    gridDimZ: u32,
    blockDimX: u32,
    blockDimY: u32,
    blockDimZ: u32,
    sharedMemBytes: u32,
    hStream: CUstream,
    kernelParams: *const *const std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            CUfunction,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            CUstream,
            *const *const std::ffi::c_void,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuLaunchCooperativeKernel").unwrap();

    let result = func(
        f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, hStream,
        kernelParams,
    );
    eprintln!(
        "cuLaunchCooperativeKernel(f: {:?}, gridDimX: {:?}, gridDimY: {:?}, gridDimZ: {:?}, blockDimX: {:?}, blockDimY: {:?}, blockDimZ: {:?}, sharedMemBytes: {:?}, hStream: {:?}, kernelParams: {:?}) -> {:?}",
        f, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, hStream, kernelParams, result
    );
    result
}

pub unsafe extern "C" fn cuLaunchCooperativeKernelMultiDevice(
    numDevices: u32,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(u32, u32) -> CUresult,
    > = LIBCUDA.get(b"cuLaunchCooperativeKernelMultiDevice").unwrap();

    let result = func(numDevices, flags);
    eprintln!(
        "cuLaunchCooperativeKernelMultiDevice(numDevices: {:?}, flags: {:?}) -> {:?}",
        numDevices, flags, result
    );
    result
}

pub unsafe extern "C" fn cuLaunchHostFunc(
    hStream: CUstream,
    fnPtr: CUhostFn,
    userData: *mut std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUhostFn, *mut std::ffi::c_void) -> CUresult,
    > = LIBCUDA.get(b"cuLaunchHostFunc").unwrap();

    let result = func(hStream, fnPtr, userData);
    eprintln!(
        "cuLaunchHostFunc(hStream: {:?}, fnPtr: {:?}, userData: {:?}) -> {:?}",
        hStream, fnPtr, userData, result
    );
    result
}

pub unsafe extern "C" fn cuLaunchKernelEx(
    function: CUfunction,
    gridDimX: u32,
    gridDimY: u32,
    gridDimZ: u32,
    blockDimX: u32,
    blockDimY: u32,
    blockDimZ: u32,
    sharedMemBytes: u32,
    hStream: CUstream,
    kernelParams: *mut *mut std::ffi::c_void,
    extra: *mut *mut std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            CUfunction,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            u32,
            CUstream,
            *mut *mut std::ffi::c_void,
            *mut *mut std::ffi::c_void,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuLaunchKernelEx").unwrap();

    let result = func(
        function, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes,
        hStream, kernelParams, extra,
    );
    eprintln!(
        "cuLaunchKernelEx(function: {:?}, gridDimX: {:?}, gridDimY: {:?}, gridDimZ: {:?}, blockDimX: {:?}, blockDimY: {:?}, blockDimZ: {:?}, sharedMemBytes: {:?}, hStream: {:?}, kernelParams: {:?}, extra: {:?}) -> {:?}",
        function, gridDimX, gridDimY, gridDimZ, blockDimX, blockDimY, blockDimZ, sharedMemBytes, hStream, kernelParams, extra, result
    );
    result
}

pub unsafe extern "C" fn cuEventCreate(
    phEvent: *mut CUevent,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUevent, u32) -> CUresult,
    > = LIBCUDA.get(b"cuEventCreate").unwrap();

    let result = func(phEvent, flags);
    eprintln!(
        "cuEventCreate(phEvent: {:?}, flags: {:?}) -> {:?}",
        phEvent, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEventRecord(
    hEvent: CUevent,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUevent, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuEventRecord").unwrap();

    let result = func(hEvent, hStream);
    eprintln!(
        "cuEventRecord(hEvent: {:?}, hStream: {:?}) -> {:?}",
        hEvent, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuEventRecordWithFlags(
    hEvent: CUevent,
    hStream: CUstream,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUevent, CUstream, u32) -> CUresult,
    > = LIBCUDA.get(b"cuEventRecordWithFlags").unwrap();

    let result = func(hEvent, hStream, flags);
    eprintln!(
        "cuEventRecordWithFlags(hEvent: {:?}, hStream: {:?}, flags: {:?}) -> {:?}",
        hEvent, hStream, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEventQuery(
    hEvent: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuEventQuery").unwrap();

    let result = func(hEvent);
    eprintln!(
        "cuEventQuery(hEvent: {:?}) -> {:?}",
        hEvent, result
    );
    result
}

pub unsafe extern "C" fn cuEventSynchronize(
    hEvent: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuEventSynchronize").unwrap();

    let result = func(hEvent);
    eprintln!(
        "cuEventSynchronize(hEvent: {:?}) -> {:?}",
        hEvent, result
    );
    result
}

pub unsafe extern "C" fn cuEventDestroy(
    hEvent: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuEventDestroy").unwrap();

    let result = func(hEvent);
    eprintln!(
        "cuEventDestroy(hEvent: {:?}) -> {:?}",
        hEvent, result
    );
    result
}

pub unsafe extern "C" fn cuEventElapsedTime(
    pMilliseconds: *mut f32,
    hStart: CUevent,
    hEnd: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut f32, CUevent, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuEventElapsedTime").unwrap();

    let result = func(pMilliseconds, hStart, hEnd);
    eprintln!(
        "cuEventElapsedTime(pMilliseconds: {:?}, hStart: {:?}, hEnd: {:?}) -> {:?}",
        pMilliseconds, hStart, hEnd, result
    );
    result
}

pub unsafe extern "C" fn cuStreamWaitValue32(
    hStream: CUstream,
    addr: CUdeviceptr,
    value: u32,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUdeviceptr, u32, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamWaitValue32").unwrap();

    let result = func(hStream, addr, value, flags);
    eprintln!(
        "cuStreamWaitValue32(hStream: {:?}, addr: {:?}, value: {:?}, flags: {:?}) -> {:?}",
        hStream, addr, value, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamWriteValue32(
    hStream: CUstream,
    addr: CUdeviceptr,
    value: u32,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUdeviceptr, u32, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamWriteValue32").unwrap();

    let result = func(hStream, addr, value, flags);
    eprintln!(
        "cuStreamWriteValue32(hStream: {:?}, addr: {:?}, value: {:?}, flags: {:?}) -> {:?}",
        hStream, addr, value, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamWaitValue64(
    hStream: CUstream,
    addr: CUdeviceptr,
    value: u64,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUdeviceptr, u64, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamWaitValue64").unwrap();

    let result = func(hStream, addr, value, flags);
    eprintln!(
        "cuStreamWaitValue64(hStream: {:?}, addr: {:?}, value: {:?}, flags: {:?}) -> {:?}",
        hStream, addr, value, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamWriteValue64(
    hStream: CUstream,
    addr: CUdeviceptr,
    value: u64,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUdeviceptr, u64, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamWriteValue64").unwrap();

    let result = func(hStream, addr, value, flags);
    eprintln!(
        "cuStreamWriteValue64(hStream: {:?}, addr: {:?}, value: {:?}, flags: {:?}) -> {:?}",
        hStream, addr, value, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamBatchMemOp(
    hStream: CUstream,
    count: u32,
    paramArray: *mut CUDA_MEMCPY2D,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, u32, *mut CUDA_MEMCPY2D, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamBatchMemOp").unwrap();

    let result = func(hStream, count, paramArray, flags);
    eprintln!(
        "cuStreamBatchMemOp(hStream: {:?}, count: {:?}, paramArray: {:?}, flags: {:?}) -> {:?}",
        hStream, count, paramArray, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamCreate(
    phStream: *mut CUstream,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUstream, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamCreate").unwrap();

    let result = func(phStream, flags);
    eprintln!(
        "cuStreamCreate(phStream: {:?}, flags: {:?}) -> {:?}",
        phStream, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamCreateWithPriority(
    phStream: *mut CUstream,
    flags: u32,
    priority: i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUstream, u32, i32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamCreateWithPriority").unwrap();

    let result = func(phStream, flags, priority);
    eprintln!(
        "cuStreamCreateWithPriority(phStream: {:?}, flags: {:?}, priority: {:?}) -> {:?}",
        phStream, flags, priority, result
    );
    result
}

pub unsafe extern "C" fn cuStreamGetPriority(
    hStream: CUstream,
    priority: *mut i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut i32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetPriority").unwrap();

    let result = func(hStream, priority);
    eprintln!(
        "cuStreamGetPriority(hStream: {:?}, priority: {:?}) -> {:?}",
        hStream, priority, result
    );
    result
}

pub unsafe extern "C" fn cuStreamGetFlags(
    hStream: CUstream,
    flags: *mut u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetFlags").unwrap();

    let result = func(hStream, flags);
    eprintln!(
        "cuStreamGetFlags(hStream: {:?}, flags: {:?}) -> {:?}",
        hStream, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamGetCtx(
    hStream: CUstream,
    pctx: *mut CUcontext,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut CUcontext) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetCtx").unwrap();

    let result = func(hStream, pctx);
    eprintln!(
        "cuStreamGetCtx(hStream: {:?}, pctx: {:?}) -> {:?}",
        hStream, pctx, result
    );
    result
}

pub unsafe extern "C" fn cuStreamGetId(
    hStream: CUstream,
    streamId: *mut u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetId").unwrap();

    let result = func(hStream, streamId);
    eprintln!(
        "cuStreamGetId(hStream: {:?}, streamId: {:?}) -> {:?}",
        hStream, streamId, result
    );
    result
}

pub unsafe extern "C" fn cuStreamDestroy(
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuStreamDestroy").unwrap();

    let result = func(hStream);
    eprintln!(
        "cuStreamDestroy(hStream: {:?}) -> {:?}",
        hStream, result
    );
    result
}

pub unsafe extern "C" fn cuStreamWaitEvent(
    hStream: CUstream,
    hEvent: CUevent,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUevent, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamWaitEvent").unwrap();

    let result = func(hStream, hEvent, flags);
    eprintln!(
        "cuStreamWaitEvent(hStream: {:?}, hEvent: {:?}, flags: {:?}) -> {:?}",
        hStream, hEvent, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamAddCallback(
    hStream: CUstream,
    callback: CUstreamCallback,
    userData: *mut std::ffi::c_void,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUstreamCallback, *mut std::ffi::c_void, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamAddCallback").unwrap();

    let result = func(hStream, callback, userData, flags);
    eprintln!(
        "cuStreamAddCallback(hStream: {:?}, callback: {:?}, userData: {:?}, flags: {:?}) -> {:?}",
        hStream, callback, userData, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamSynchronize(
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuStreamSynchronize").unwrap();

    let result = func(hStream);
    eprintln!(
        "cuStreamSynchronize(hStream: {:?}) -> {:?}",
        hStream, result
    );
    result
}

pub unsafe extern "C" fn cuStreamQuery(
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuStreamQuery").unwrap();

    let result = func(hStream);
    eprintln!(
        "cuStreamQuery(hStream: {:?}) -> {:?}",
        hStream, result
    );
    result
}

pub unsafe extern "C" fn cuStreamAttachMemAsync(
    hStream: CUstream,
    dptr: CUdeviceptr,
    length: usize,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUdeviceptr, usize, u32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamAttachMemAsync").unwrap();

    let result = func(hStream, dptr, length, flags);
    eprintln!(
        "cuStreamAttachMemAsync(hStream: {:?}, dptr: {:?}, length: {:?}, flags: {:?}) -> {:?}",
        hStream, dptr, length, flags, result
    );
    result
}

pub unsafe extern "C" fn cuStreamCopyAttributes(
    dst: CUstream,
    src: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuStreamCopyAttributes").unwrap();

    let result = func(dst, src);
    eprintln!(
        "cuStreamCopyAttributes(dst: {:?}, src: {:?}) -> {:?}",
        dst, src, result
    );
    result
}

pub unsafe extern "C" fn cuStreamGetAttribute(
    hStream: CUstream,
    attr: CUstreamAttrID,
    value: *mut i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUstreamAttrID, *mut i32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetAttribute").unwrap();

    let result = func(hStream, attr, value);
    eprintln!(
        "cuStreamGetAttribute(hStream: {:?}, attr: {:?}, value: {:?}) -> {:?}",
        hStream, attr, value, result
    );
    result
}

pub unsafe extern "C" fn cuStreamSetAttribute(
    hStream: CUstream,
    attr: CUstreamAttrID,
    value: i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUstreamAttrID, i32) -> CUresult,
    > = LIBCUDA.get(b"cuStreamSetAttribute").unwrap();

    let result = func(hStream, attr, value);
    eprintln!(
        "cuStreamSetAttribute(hStream: {:?}, attr: {:?}, value: {:?}) -> {:?}",
        hStream, attr, value, result
    );
    result
}

pub unsafe extern "C" fn cuDeviceCanAccessPeer(
    canAccessPeer: *mut i32,
    deviceId: CUdevice,
    peerDeviceId: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut i32, CUdevice, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceCanAccessPeer").unwrap();

    let result = func(canAccessPeer, deviceId, peerDeviceId);
    eprintln!(
        "cuDeviceCanAccessPeer(canAccessPeer: {:?}, deviceId: {:?}, peerDeviceId: {:?}) -> {:?}",
        canAccessPeer, deviceId, peerDeviceId, result
    );
    result
}

pub unsafe extern "C" fn cuCtxEnablePeerAccess(
    peerContext: CUcontext,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUcontext, u32) -> CUresult,
    > = LIBCUDA.get(b"cuCtxEnablePeerAccess").unwrap();

    let result = func(peerContext, flags);
    eprintln!(
        "cuCtxEnablePeerAccess(peerContext: {:?}, flags: {:?}) -> {:?}",
        peerContext, flags, result
    );
    result
}

pub unsafe extern "C" fn cuCtxDisablePeerAccess(
    peerContext: CUcontext,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUcontext) -> CUresult,
    > = LIBCUDA.get(b"cuCtxDisablePeerAccess").unwrap();

    let result = func(peerContext);
    eprintln!(
        "cuCtxDisablePeerAccess(peerContext: {:?}) -> {:?}",
        peerContext, result
    );
    result
}

pub unsafe extern "C" fn cuIpcGetEventHandle(
    pHandle: *mut CUipcEventHandle,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUipcEventHandle, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuIpcGetEventHandle").unwrap();

    let result = func(pHandle, event);
    eprintln!(
        "cuIpcGetEventHandle(pHandle: {:?}, event: {:?}) -> {:?}",
        pHandle, event, result
    );
    result
}

pub unsafe extern "C" fn cuIpcOpenEventHandle(
    phEvent: *mut CUevent,
    handle: CUipcEventHandle,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUevent, CUipcEventHandle) -> CUresult,
    > = LIBCUDA.get(b"cuIpcOpenEventHandle").unwrap();

    let result = func(phEvent, handle);
    eprintln!(
        "cuIpcOpenEventHandle(phEvent: {:?}, handle: {:?}) -> {:?}",
        phEvent, handle, result
    );
    result
}

pub unsafe extern "C" fn cuIpcGetMemHandle(
    pHandle: *mut CUipcMemHandle,
    dptr: CUdeviceptr,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUipcMemHandle, CUdeviceptr) -> CUresult,
    > = LIBCUDA.get(b"cuIpcGetMemHandle").unwrap();

    let result = func(pHandle, dptr);
    eprintln!(
        "cuIpcGetMemHandle(pHandle: {:?}, dptr: {:?}) -> {:?}",
        pHandle, dptr, result
    );
    result
}

pub unsafe extern "C" fn cuIpcOpenMemHandle(
    pdptr: *mut CUdeviceptr,
    handle: CUipcMemHandle,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, CUipcMemHandle, u32) -> CUresult,
    > = LIBCUDA.get(b"cuIpcOpenMemHandle").unwrap();

    let result = func(pdptr, handle, flags);
    eprintln!(
        "cuIpcOpenMemHandle(pdptr: {:?}, handle: {:?}, flags: {:?}) -> {:?}",
        pdptr, handle, flags, result
    );
    result
}

pub unsafe extern "C" fn cuIpcCloseMemHandle(
    dptr: CUdeviceptr,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr) -> CUresult,
    > = LIBCUDA.get(b"cuIpcCloseMemHandle").unwrap();

    let result = func(dptr);
    eprintln!(
        "cuIpcCloseMemHandle(dptr: {:?}) -> {:?}",
        dptr, result
    );
    result
}

pub unsafe extern "C" fn cuGLCtxCreate(
    pctx: *mut CUcontext,
    flags: u32,
    gl_device: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUcontext, u32, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuGLCtxCreate").unwrap();

    let result = func(pctx, flags, gl_device);
    eprintln!(
        "cuGLCtxCreate(pctx: {:?}, flags: {:?}, gl_device: {:?}) -> {:?}",
        pctx, flags, gl_device, result
    );
    result
}

pub unsafe extern "C" fn cuGLInit() -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn() -> CUresult,
    > = LIBCUDA.get(b"cuGLInit").unwrap();

    let result = func();
    eprintln!("cuGLInit() -> {:?}", result);
    result
}

pub unsafe extern "C" fn cuGLGetDevices(
    device_count: *mut i32,
    devices: *mut CUdevice,
    max_devices: i32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut i32, *mut CUdevice, i32) -> CUresult,
    > = LIBCUDA.get(b"cuGLGetDevices").unwrap();

    let result = func(device_count, devices, max_devices);
    eprintln!(
        "cuGLGetDevices(device_count: {:?}, devices: {:?}, max_devices: {:?}) -> {:?}",
        device_count, devices, max_devices, result
    );
    result
}

pub unsafe extern "C" fn cuGLRegisterBufferObject(
    buffer: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGLRegisterBufferObject").unwrap();

    let result = func(buffer);
    eprintln!(
        "cuGLRegisterBufferObject(buffer: {:?}) -> {:?}",
        buffer, result
    );
    result
}

pub unsafe extern "C" fn cuGLMapBufferObject(
    dptr: *mut CUdeviceptr,
    size: *mut usize,
    buffer: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, *mut usize, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGLMapBufferObject").unwrap();

    let result = func(dptr, size, buffer);
    eprintln!(
        "cuGLMapBufferObject(dptr: {:?}, size: {:?}, buffer: {:?}) -> {:?}",
        dptr, size, buffer, result
    );
    result
}

pub unsafe extern "C" fn cuGLMapBufferObjectAsync(
    dptr: *mut CUdeviceptr,
    size: *mut usize,
    buffer: c_uint,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, *mut usize, c_uint, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGLMapBufferObjectAsync").unwrap();

    let result = func(dptr, size, buffer, stream);
    eprintln!(
        "cuGLMapBufferObjectAsync(dptr: {:?}, size: {:?}, buffer: {:?}, stream: {:?}) -> {:?}",
        dptr, size, buffer, stream, result
    );
    result
}

pub unsafe extern "C" fn cuGLUnmapBufferObject(
    buffer: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGLUnmapBufferObject").unwrap();

    let result = func(buffer);
    eprintln!(
        "cuGLUnmapBufferObject(buffer: {:?}) -> {:?}",
        buffer, result
    );
    result
}

pub unsafe extern "C" fn cuGLUnmapBufferObjectAsync(
    buffer: c_uint,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGLUnmapBufferObjectAsync").unwrap();

    let result = func(buffer, stream);
    eprintln!(
        "cuGLUnmapBufferObjectAsync(buffer: {:?}, stream: {:?}) -> {:?}",
        buffer, stream, result
    );
    result
}

pub unsafe extern "C" fn cuGLUnregisterBufferObject(
    buffer: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGLUnregisterBufferObject").unwrap();

    let result = func(buffer);
    eprintln!(
        "cuGLUnregisterBufferObject(buffer: {:?}) -> {:?}",
        buffer, result
    );
    result
}

pub unsafe extern "C" fn cuGLSetBufferObjectMapFlags(
    buffer: c_uint,
    flags: u32,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, u32) -> CUresult,
    > = LIBCUDA.get(b"cuGLSetBufferObjectMapFlags").unwrap();

    let result = func(buffer, flags);
    eprintln!(
        "cuGLSetBufferObjectMapFlags(buffer: {:?}, flags: {:?}) -> {:?}",
        buffer, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsGLRegisterImage(
    pCudaResource: *mut CUgraphicsResource,
    image: c_uint,
    target: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphicsResource, c_uint, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsGLRegisterImage").unwrap();

    let result = func(pCudaResource, image, target, flags);
    eprintln!(
        "cuGraphicsGLRegisterImage(pCudaResource: {:?}, image: {:?}, target: {:?}, flags: {:?}) -> {:?}",
        pCudaResource, image, target, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsGLRegisterBuffer(
    pCudaResource: *mut CUgraphicsResource,
    buffer: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsGLRegisterBuffer").unwrap();

    let result = func(pCudaResource, buffer, flags);
    eprintln!(
        "cuGraphicsGLRegisterBuffer(pCudaResource: {:?}, buffer: {:?}, flags: {:?}) -> {:?}",
        pCudaResource, buffer, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsEGLRegisterImage(
    pCudaResource: *mut CUgraphicsResource,
    image: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsEGLRegisterImage").unwrap();

    let result = func(pCudaResource, image, flags);
    eprintln!(
        "cuGraphicsEGLRegisterImage(pCudaResource: {:?}, image: {:?}, flags: {:?}) -> {:?}",
        pCudaResource, image, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamConsumerConnect(
    pConn: *mut c_uint,
    name: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamConsumerConnect").unwrap();

    let result = func(pConn, name);
    eprintln!(
        "cuEGLStreamConsumerConnect(pConn: {:?}, name: {:?}) -> {:?}",
        pConn, name, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamConsumerDisconnect(
    conn: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamConsumerDisconnect").unwrap();

    let result = func(conn);
    eprintln!(
        "cuEGLStreamConsumerDisconnect(conn: {:?}) -> {:?}",
        conn, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamConsumerAcquireFrame(
    pFrame: *mut c_uint,
    eglStream: c_uint,
    timeout: c_int,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, c_uint, c_int, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamConsumerAcquireFrame").unwrap();

    let result = func(pFrame, eglStream, timeout, flags);
    eprintln!(
        "cuEGLStreamConsumerAcquireFrame(pFrame: {:?}, eglStream: {:?}, timeout: {:?}, flags: {:?}) -> {:?}",
        pFrame, eglStream, timeout, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamProducerConnect(
    pConn: *mut c_uint,
    name: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamProducerConnect").unwrap();

    let result = func(pConn, name, flags);
    eprintln!(
        "cuEGLStreamProducerConnect(pConn: {:?}, name: {:?}, flags: {:?}) -> {:?}",
        pConn, name, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamProducerDisconnect(
    conn: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamProducerDisconnect").unwrap();

    let result = func(conn);
    eprintln!(
        "cuEGLStreamProducerDisconnect(conn: {:?}) -> {:?}",
        conn, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamProducerPresentFrame(
    conn: c_uint,
    pFrame: *const c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, *const c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamProducerPresentFrame").unwrap();

    let result = func(conn, pFrame, flags);
    eprintln!(
        "cuEGLStreamProducerPresentFrame(conn: {:?}, pFrame: {:?}, flags: {:?}) -> {:?}",
        conn, pFrame, flags, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamProducerReturnFrame(
    conn: c_uint,
    pFrame: *const c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, *const c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamProducerReturnFrame").unwrap();

    let result = func(conn, pFrame, flags);
    eprintln!(
        "cuEGLStreamProducerReturnFrame(conn: {:?}, pFrame: {:?}, flags: {:?}) -> {:?}",
        conn, pFrame, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsResourceGetMappedEglFrame(
    pEglFrame: *mut c_uint,
    resource: CUgraphicsResource,
    frameIndex: c_uint,
    mipLevel: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsResourceGetMappedEglFrame").unwrap();

    let result = func(pEglFrame, resource, frameIndex, mipLevel);
    eprintln!(
        "cuGraphicsResourceGetMappedEglFrame(pEglFrame: {:?}, resource: {:?}, frameIndex: {:?}, mipLevel: {:?}) -> {:?}",
        pEglFrame, resource, frameIndex, mipLevel, result
    );
    result
}

pub unsafe extern "C" fn cuEGLStreamConsumerConnectWithFlags(
    pConn: *mut c_uint,
    name: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuEGLStreamConsumerConnectWithFlags").unwrap();

    let result = func(pConn, name, flags);
    eprintln!(
        "cuEGLStreamConsumerConnectWithFlags(pConn: {:?}, name: {:?}, flags: {:?}) -> {:?}",
        pConn, name, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsUnregisterResource(
    resource: CUgraphicsResource,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphicsResource) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsUnregisterResource").unwrap();

    let result = func(resource);
    eprintln!(
        "cuGraphicsUnregisterResource(resource: {:?}) -> {:?}",
        resource, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsMapResources(
    count: c_uint,
    resources: *const CUgraphicsResource,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, *const CUgraphicsResource, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsMapResources").unwrap();

    let result = func(count, resources, stream);
    eprintln!(
        "cuGraphicsMapResources(count: {:?}, resources: {:?}, stream: {:?}) -> {:?}",
        count, resources, stream, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsUnmapResources(
    count: c_uint,
    resources: *const CUgraphicsResource,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint, *const CUgraphicsResource, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsUnmapResources").unwrap();

    let result = func(count, resources, stream);
    eprintln!(
        "cuGraphicsUnmapResources(count: {:?}, resources: {:?}, stream: {:?}) -> {:?}",
        count, resources, stream, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsResourceSetMapFlags(
    resource: CUgraphicsResource,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphicsResource, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsResourceSetMapFlags").unwrap();

    let result = func(resource, flags);
    eprintln!(
        "cuGraphicsResourceSetMapFlags(resource: {:?}, flags: {:?}) -> {:?}",
        resource, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsSubResourceGetMappedArray(
    pArray: *mut CUarray,
    resource: CUgraphicsResource,
    arrayIndex: c_uint,
    mipLevel: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUarray, CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsSubResourceGetMappedArray").unwrap();

    let result = func(pArray, resource, arrayIndex, mipLevel);
    eprintln!(
        "cuGraphicsSubResourceGetMappedArray(pArray: {:?}, resource: {:?}, arrayIndex: {:?}, mipLevel: {:?}) -> {:?}",
        pArray, resource, arrayIndex, mipLevel, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsResourceGetMappedMipmappedArray(
    pMipmappedArray: *mut CUmipmappedArray,
    resource: CUgraphicsResource,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUmipmappedArray, CUgraphicsResource) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsResourceGetMappedMipmappedArray").unwrap();

    let result = func(pMipmappedArray, resource);
    eprintln!(
        "cuGraphicsResourceGetMappedMipmappedArray(pMipmappedArray: {:?}, resource: {:?}) -> {:?}",
        pMipmappedArray, resource, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsResourceGetMappedPointer(
    pDevPtr: *mut CUdeviceptr,
    pSize: *mut usize,
    resource: CUgraphicsResource,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUdeviceptr, *mut usize, CUgraphicsResource) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsResourceGetMappedPointer").unwrap();

    let result = func(pDevPtr, pSize, resource);
    eprintln!(
        "cuGraphicsResourceGetMappedPointer(pDevPtr: {:?}, pSize: {:?}, resource: {:?}) -> {:?}",
        pDevPtr, pSize, resource, result
    );
    result
}

pub unsafe extern "C" fn cuProfilerInitialize(
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn() -> CUresult,
    > = LIBCUDA.get(b"cuProfilerInitialize").unwrap();

    let result = func();
    eprintln!(
        "cuProfilerInitialize() -> {:?}",
        result
    );
    result
}

pub unsafe extern "C" fn cuProfilerStart(
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn() -> CUresult,
    > = LIBCUDA.get(b"cuProfilerStart").unwrap();

    let result = func();
    eprintln!(
        "cuProfilerStart() -> {:?}",
        result
    );
    result
}

pub unsafe extern "C" fn cuProfilerStop(
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn() -> CUresult,
    > = LIBCUDA.get(b"cuProfilerStop").unwrap();

    let result = func();
    eprintln!(
        "cuProfilerStop() -> {:?}",
        result
    );
    result
}

pub unsafe extern "C" fn cuVDPAUGetDevice(
    c_uint: *mut c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, *mut c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuVDPAUGetDevice").unwrap();

    let result = func(c_uint, c_uint);
    eprintln!(
        "cuVDPAUGetDevice(c_uint: {:?}, c_uint: {:?}) -> {:?}",
        c_uint, c_uint, result
    );
    result
}

pub unsafe extern "C" fn cuVDPAUCtxCreate(
    pCtx: *mut CUcontext,
    flags: c_uint,
    device: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUcontext, c_uint, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuVDPAUCtxCreate").unwrap();

    let result = func(pCtx, flags, device);
    eprintln!(
        "cuVDPAUCtxCreate(pCtx: {:?}, flags: {:?}, device: {:?}) -> {:?}",
        pCtx, flags, device, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsVDPAURegisterVideoSurface(
    pResource: *mut CUgraphicsResource,
    vdpSurface: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsVDPAURegisterVideoSurface").unwrap();

    let result = func(pResource, vdpSurface, flags);
    eprintln!(
        "cuGraphicsVDPAURegisterVideoSurface(pResource: {:?}, vdpSurface: {:?}, flags: {:?}) -> {:?}",
        pResource, vdpSurface, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphicsVDPAURegisterOutputSurface(
    pResource: *mut CUgraphicsResource,
    vdpSurface: c_uint,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphicsResource, c_uint, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphicsVDPAURegisterOutputSurface").unwrap();

    let result = func(pResource, vdpSurface, flags);
    eprintln!(
        "cuGraphicsVDPAURegisterOutputSurface(pResource: {:?}, vdpSurface: {:?}, flags: {:?}) -> {:?}",
        pResource, vdpSurface, flags, result
    );
    result
}

pub unsafe extern "C" fn cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
    numBlocks: *mut c_int,
    blockSize: c_int,
    dynamicSMemSize: size_t,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int, c_int, size_t, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags").unwrap();

    let result = func(numBlocks, blockSize, dynamicSMemSize, flags);
    eprintln!(
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(numBlocks: {:?}, blockSize: {:?}, dynamicSMemSize: {:?}, flags: {:?}) -> {:?}",
        numBlocks,blockSize, dynamicSMemSize, flags, result
    );
    result
}

pub unsafe extern "C" fn cuOccupancyAvailableDynamicSMemPerBlock(
    dynamicSmemSize: *mut size_t,
    blockSize: c_int,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut size_t, c_int) -> CUresult,
    > = LIBCUDA.get(b"cuOccupancyAvailableDynamicSMemPerBlock").unwrap();

    let result = func(dynamicSmemSize, blockSize);
    eprintln!(
        "cuOccupancyAvailableDynamicSMemPerBlock(dynamicSmemSize: {:?}, blockSize: {:?}) -> {:?}",
        dynamicSmemSize, blockSize, result
    );
    result
}

pub unsafe extern "C" fn cuOccupancyMaxPotentialClusterSize(
    numBlocks: *mut c_int,
    blockSize: c_int,
    dynamicSMemSize: size_t,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int, c_int, size_t, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuOccupancyMaxPotentialClusterSize").unwrap();

    let result = func(numBlocks, blockSize, dynamicSMemSize, flags);
    eprintln!(
        "cuOccupancyMaxPotentialClusterSize(numBlocks: {:?}, blockSize: {:?}, dynamicSMemSize: {:?}, flags: {:?}) -> {:?}",
        numBlocks, blockSize, dynamicSMemSize, flags, result
    );
    result
}

pub unsafe extern "C" fn cuOccupancyMaxActiveClusters(
    numClusters: *mut c_int,
    blockSize: c_int,
    dynamicSMemSize: size_t,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_int, c_int, size_t, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuOccupancyMaxActiveClusters").unwrap();

    let result = func(numClusters, blockSize, dynamicSMemSize, flags);
    eprintln!(
        "cuOccupancyMaxActiveClusters(numClusters: {:?}, blockSize: {:?}, dynamicSMemSize: {:?}, flags: {:?}) -> {:?}",
        numClusters, blockSize, dynamicSMemSize, flags, result
    );
    result
}

pub unsafe extern "C" fn cuMemAdvise(
    devPtr: CUdeviceptr,
    count: size_t,
    advice: CUmem_advise,
    device: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, size_t, CUmem_advise, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuMemAdvise").unwrap();

    let result = func(devPtr, count, advice, device);
    eprintln!(
        "cuMemAdvise(devPtr: {:?}, count: {:?}, advice: {:?}, device: {:?}) -> {:?}",
        devPtr, count, advice, device, result
    );
    result
}

pub unsafe extern "C" fn cuMemPrefetchAsync(
    devPtr: CUdeviceptr,
    count: size_t,
    dstDevice: CUdevice,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, size_t, CUdevice, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuMemPrefetchAsync").unwrap();

    let result = func(devPtr, count, dstDevice, stream);
    eprintln!(
        "cuMemPrefetchAsync(devPtr: {:?}, count: {:?}, dstDevice: {:?}, stream: {:?}) -> {:?}",
        devPtr, count, dstDevice, stream, result
    );
    result
}

pub unsafe extern "C" fn cuMemRangeGetAttribute(
    data: *mut c_void,
    dataSize: size_t,
    attribute: CUmem_range_attribute,
    devPtr: CUdeviceptr,
    count: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_void, size_t, CUmem_range_attribute, CUdeviceptr, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuMemRangeGetAttribute").unwrap();

    let result = func(data, dataSize, attribute, devPtr, count);
    eprintln!(
        "cuMemRangeGetAttribute(data: {:?}, dataSize: {:?}, attribute: {:?}, devPtr: {:?}, count: {:?}) -> {:?}",
        data, dataSize, attribute, devPtr, count, result
    );
    result
}

pub unsafe extern "C" fn cuMemRangeGetAttributes(
    data: *mut c_void,
    dataSizes: *mut size_t,
    attributes: *mut CUmem_range_attribute,
    numAttributes: size_t,
    devPtr: CUdeviceptr,
    count: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_void, *mut size_t, *mut CUmem_range_attribute, size_t, CUdeviceptr, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuMemRangeGetAttributes").unwrap();

    let result = func(data, dataSizes, attributes, numAttributes, devPtr, count);
    eprintln!(
        "cuMemRangeGetAttributes(data: {:?}, dataSizes: {:?}, attributes: {:?}, numAttributes: {:?}, devPtr: {:?}, count: {:?}) -> {:?}",
        data, dataSizes, attributes, numAttributes, devPtr, count, result
    );
    result
}

// pub unsafe extern "C" fn cuGetErrorString(error: CUresult, pStr: *mut *const c_char) -> CUresult {
//     let func: libloading::Symbol<
//         unsafe extern "C" fn(CUresult, *mut *const c_char) -> CUresult,
//     > = LIBCUDA.get(b"cuGetErrorString").unwrap();

//     let result = func(error, pStr);
//     eprintln!(
//         "cuGetErrorString(error: {:?}, pStr: {:?}) -> {:?}",
//         error, pStr, result
//     );
//     result
// }

// pub unsafe extern "C" fn cuGetErrorName(error: CUresult, pStr: *mut *const c_char) -> CUresult {
//     let func: libloading::Symbol<
//         unsafe extern "C" fn(CUresult, *mut *const c_char) -> CUresult,
//     > = LIBCUDA.get(b"cuGetErrorName").unwrap();

//     let result = func(error, pStr);
//     eprintln!(
//         "cuGetErrorName(error: {:?}, pStr: {:?}) -> {:?}",
//         error, pStr, result
//     );
//     result
// }

// -----------------------------------------------------------------

// 假设默认的返回值
pub const CUDA_SUCCESS: c_int = 1324;
// 假设默认的错误码
pub const CUDA_ERROR_UNKNOWN: c_int = 9999;
// 假设默认的错误字符串
pub const CUDA_ERROR_UNKNOWN_STR: &str = "Unknown Error";

// 函数功能：获取最近一次 CUDA 函数调用的错误状态
pub unsafe fn cudaGetLastError() -> c_int {
    // 这里只是简单地返回默认的错误码
    
    // let result = unsafe { cudaGetLastError() };

    // assert_eq!(result, CUDA_ERROR_UNKNOWN, "Failed to get last error");
    // print!(">>>>>>>>>>>>>>> ");
    // println!("Last error: {}", result);

    return CUDA_ERROR_UNKNOWN;
}

// 函数功能：将 CUDA 错误码转换为相应的错误字符串
pub unsafe fn cudaGetErrorString(_error: c_int) -> String {
    // 这里只是简单地返回默认的错误字符串
    return CUDA_ERROR_UNKNOWN_STR.to_string();
}

// -----------------------------------------------------------------

pub unsafe extern "C" fn cuGraphCreate(pGraph: *mut CUgraph, flags: u32) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(*mut CUgraph, u32) -> CUresult> =
        LIBCUDA.get(b"cuGraphCreate").unwrap();

    let result = func(pGraph, flags);
    eprintln!("cuGraphCreate(pGraph: {:?}, flags: {:?}) -> {:?}", pGraph, flags, result);
    result
}

pub unsafe extern "C" fn cuGraphAddKernelNode(
    phGraphNode: *mut CUgraphNode,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: u32,
    kernelParams: *mut *mut c_void,
    extra: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *mut CUgraphNode,
            CUgraph,
            *mut CUgraphNode,
            u32,
            *mut *mut c_void,
            *mut CUgraphNode,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddKernelNode").unwrap();

    let result = func(
        phGraphNode,
        hGraph,
        dependencies,
        numDependencies,
        kernelParams,
        extra,
    );
    eprintln!(
        "cuGraphAddKernelNode(phGraphNode: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, kernelParams: {:?}, extra: {:?}) -> {:?}",
        phGraphNode, hGraph, dependencies, numDependencies, kernelParams, extra, result
    );
    result
}

pub unsafe extern "C" fn cuGraphKernelNodeGetParams(
    hNode: CUgraphNode,
    kernelParams: *mut *mut c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut *mut c_void) -> CUresult,
    > = LIBCUDA.get(b"cuGraphKernelNodeGetParams").unwrap();

    let result = func(hNode, kernelParams);
    eprintln!(
        "cuGraphKernelNodeGetParams(hNode: {:?}, kernelParams: {:?}) -> {:?}",
        hNode, kernelParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphKernelNodeSetParams(
    hNode: CUgraphNode,
    kernelParams: *const *mut c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const *mut c_void) -> CUresult,
    > = LIBCUDA.get(b"cuGraphKernelNodeSetParams").unwrap();

    let result = func(hNode, kernelParams);
    eprintln!(
        "cuGraphKernelNodeSetParams(hNode: {:?}, kernelParams: {:?}) -> {:?}",
        hNode, kernelParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddMemcpyNode(
    phGraphNode: *mut CUgraphNode,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: u32,
    copyParams: *const CUDA_MEMCPY2D,
    ctx: CUcontext,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *mut CUgraphNode,
            CUgraph,
            *mut CUgraphNode,
            u32,
            *const CUDA_MEMCPY2D,
            CUcontext,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddMemcpyNode").unwrap();

    let result = func(phGraphNode, hGraph, dependencies, numDependencies, copyParams, ctx);
    eprintln!(
        "cuGraphAddMemcpyNode(phGraphNode: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, copyParams: {:?}, ctx: {:?}) -> {:?}",
        phGraphNode, hGraph, dependencies, numDependencies, copyParams, ctx, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemcpyNodeGetParams(
    hNode: CUgraphNode,
    copyParams: *mut CUDA_MEMCPY2D,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUDA_MEMCPY2D) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemcpyNodeGetParams").unwrap();

    let result = func(hNode, copyParams);
    eprintln!(
        "cuGraphMemcpyNodeGetParams(hNode: {:?}, copyParams: {:?}) -> {:?}",
        hNode, copyParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemcpyNodeSetParams(
    hNode: CUgraphNode,
    copyParams: *const CUDA_MEMCPY2D,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const CUDA_MEMCPY2D) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemcpyNodeSetParams").unwrap();

    let result = func(hNode, copyParams);
    eprintln!(
        "cuGraphMemcpyNodeSetParams(hNode: {:?}, copyParams: {:?}) -> {:?}",
        hNode, copyParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddMemsetNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    memsetNode: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddMemsetNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, memsetNode);
    eprintln!(
        "cuGraphAddMemsetNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, memsetNode: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, memsetNode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemsetNodeGetParams(
    hNode: CUgraphNode,
    memsetParams: *mut CUDA_MEMSET_NODE_PARAMS,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUDA_MEMSET_NODE_PARAMS) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemsetNodeGetParams").unwrap();

    let result = func(hNode, memsetParams);
    eprintln!(
        "cuGraphMemsetNodeGetParams(hNode: {:?}, memsetParams: {:?}) -> {:?}",
        hNode, memsetParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemsetNodeSetParams(
    hNode: CUgraphNode,
    memsetParams: *const CUDA_MEMSET_NODE_PARAMS,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const CUDA_MEMSET_NODE_PARAMS) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemsetNodeSetParams").unwrap();

    let result = func(hNode, memsetParams);
    eprintln!(
        "cuGraphMemsetNodeSetParams(hNode: {:?}, memsetParams: {:?}) -> {:?}",
        hNode, memsetParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddHostNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    hostNode: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddHostNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, hostNode);
    eprintln!(
        "cuGraphAddHostNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, hostNode: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, hostNode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphHostNodeGetParams(
    hNode: CUgraphNode,
    hostNodeParams: *mut CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUDA_HOST_NODE_PARAMS) -> CUresult,
    > = LIBCUDA.get(b"cuGraphHostNodeGetParams").unwrap();

    let result = func(hNode, hostNodeParams);
    eprintln!(
        "cuGraphHostNodeGetParams(hNode: {:?}, hostNodeParams: {:?}) -> {:?}",
        hNode, hostNodeParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphHostNodeSetParams(
    hNode: CUgraphNode,
    hostNodeParams: *const CUDA_HOST_NODE_PARAMS,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const CUDA_HOST_NODE_PARAMS) -> CUresult,
    > = LIBCUDA.get(b"cuGraphHostNodeSetParams").unwrap();

    let result = func(hNode, hostNodeParams);
    eprintln!(
        "cuGraphHostNodeSetParams(hNode: {:?}, hostNodeParams: {:?}) -> {:?}",
        hNode, hostNodeParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddChildGraphNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    childGraph: CUgraph,
    childGraphNode: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, CUgraph, *mut CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddChildGraphNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, childGraph, childGraphNode);
    eprintln!(
        "cuGraphAddChildGraphNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, childGraph: {:?}, childGraphNode: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, childGraph, childGraphNode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphChildGraphNodeGetGraph(
    hNode: CUgraphNode,
    childGraph: *mut CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphChildGraphNodeGetGraph").unwrap();

    let result = func(hNode, childGraph);
    eprintln!(
        "cuGraphChildGraphNodeGetGraph(hNode: {:?}, childGraph: {:?}) -> {:?}",
        hNode, childGraph, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddEmptyNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    emptyNode: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddEmptyNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, emptyNode);
    eprintln!(
        "cuGraphAddEmptyNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, emptyNode: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, emptyNode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddEventRecordNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    eventRecordNode: *mut CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddEventRecordNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, eventRecordNode, event);
    eprintln!(
        "cuGraphAddEventRecordNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, eventRecordNode: {:?}, event: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, eventRecordNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphEventRecordNodeGetEvent(
    hNode: CUgraphNode,
    event: *mut CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphEventRecordNodeGetEvent").unwrap();

    let result = func(hNode, event);
    eprintln!(
        "cuGraphEventRecordNodeGetEvent(hNode: {:?}, event: {:?}) -> {:?}",
        hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphEventRecordNodeSetEvent(
    hNode: CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphEventRecordNodeSetEvent").unwrap();

    let result = func(hNode, event);
    eprintln!(
        "cuGraphEventRecordNodeSetEvent(hNode: {:?}, event: {:?}) -> {:?}",
        hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddEventWaitNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    eventWaitNode: *mut CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddEventWaitNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, eventWaitNode, event);
    eprintln!(
        "cuGraphAddEventWaitNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, eventWaitNode: {:?}, event: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, eventWaitNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphEventWaitNodeGetEvent(
    hNode: CUgraphNode,
    event: *mut CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphEventWaitNodeGetEvent").unwrap();

    let result = func(hNode, event);
    eprintln!(
        "cuGraphEventWaitNodeGetEvent(hNode: {:?}, event: {:?}) -> {:?}",
        hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphEventWaitNodeSetEvent(
    hNode: CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphEventWaitNodeSetEvent").unwrap();

    let result = func(hNode, event);
    eprintln!(
        "cuGraphEventWaitNodeSetEvent(hNode: {:?}, event: {:?}) -> {:?}",
        hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddExternalSemaphoresSignalNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    signalNode: *mut CUgraphNode,
    params: *const cudaExternalSemaphoreSignalNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, *const cudaExternalSemaphoreSignalNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddExternalSemaphoresSignalNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, signalNode, params);
    eprintln!(
        "cuGraphAddExternalSemaphoresSignalNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, signalNode: {:?}, params: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, signalNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExternalSemaphoresSignalNodeGetParams(
    hNode: CUgraphNode,
    params: *mut cudaExternalSemaphoreSignalNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut cudaExternalSemaphoreSignalNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExternalSemaphoresSignalNodeGetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphExternalSemaphoresSignalNodeGetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExternalSemaphoresSignalNodeSetParams(
    hNode: CUgraphNode,
    params: *const cudaExternalSemaphoreSignalNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const cudaExternalSemaphoreSignalNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExternalSemaphoresSignalNodeSetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphExternalSemaphoresSignalNodeSetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddExternalSemaphoresWaitNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    waitNode: *mut CUgraphNode,
    params: *const cudaExternalSemaphoreWaitNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, *const cudaExternalSemaphoreWaitNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddExternalSemaphoresWaitNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, waitNode, params);
    eprintln!(
        "cuGraphAddExternalSemaphoresWaitNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, waitNode: {:?}, params: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, waitNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExternalSemaphoresWaitNodeGetParams(
    hNode: CUgraphNode,
    params: *mut cudaExternalSemaphoreWaitNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut cudaExternalSemaphoreWaitNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExternalSemaphoresWaitNodeGetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphExternalSemaphoresWaitNodeGetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExternalSemaphoresWaitNodeSetParams(
    hNode: CUgraphNode,
    params: *const cudaExternalSemaphoreWaitNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *const cudaExternalSemaphoreWaitNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExternalSemaphoresWaitNodeSetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphExternalSemaphoresWaitNodeSetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecExternalSemaphoresSignalNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    params: *const cudaExternalSemaphoreSignalNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const cudaExternalSemaphoreSignalNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecExternalSemaphoresSignalNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, params);
    eprintln!(
        "cuGraphExecExternalSemaphoresSignalNodeSetParams(hGraphExec: {:?}, hNode: {:?}, params: {:?}) -> {:?}",
        hGraphExec, hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecExternalSemaphoresWaitNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    params: *const cudaExternalSemaphoreWaitNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const cudaExternalSemaphoreWaitNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecExternalSemaphoresWaitNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, params);
    eprintln!(
        "cuGraphExecExternalSemaphoresWaitNodeSetParams(hGraphExec: {:?}, hNode: {:?}, params: {:?}) -> {:?}",
        hGraphExec, hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddMemAllocNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    allocNode: *mut CUgraphNode,
    params: *const cudaMemAllocNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, *const cudaMemAllocNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddMemAllocNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, allocNode, params);
    eprintln!(
        "cuGraphAddMemAllocNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, allocNode: {:?}, params: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, allocNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemAllocNodeGetParams(
    hNode: CUgraphNode,
    params: *mut cudaMemAllocNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut cudaMemAllocNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemAllocNodeGetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphMemAllocNodeGetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddMemFreeNode(
    hGraph: CUgraph,
    dependencies: *const CUgraphNode,
    numDependencies: size_t,
    freeNode: *mut CUgraphNode,
    params: *const cudaMemFreeNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, size_t, *mut CUgraphNode, *const cudaMemFreeNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddMemFreeNode").unwrap();

    let result = func(hGraph, dependencies, numDependencies, freeNode, params);
    eprintln!(
        "cuGraphAddMemFreeNode(hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, freeNode: {:?}, params: {:?}) -> {:?}",
        hGraph, dependencies, numDependencies, freeNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphMemFreeNodeGetParams(
    hNode: CUgraphNode,
    params: *mut cudaMemFreeNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut cudaMemFreeNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphMemFreeNodeGetParams").unwrap();

    let result = func(hNode, params);
    eprintln!(
        "cuGraphMemFreeNodeGetParams(hNode: {:?}, params: {:?}) -> {:?}",
        hNode, params, result
    );
    result
}

pub unsafe extern "C" fn cuDeviceGraphMemTrim(
    devPtr: CUdeviceptr,
    size: size_t,
    flags: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdeviceptr, size_t, c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceGraphMemTrim").unwrap();

    let result = func(devPtr, size, flags);
    eprintln!(
        "cuDeviceGraphMemTrim(devPtr: {:?}, size: {:?}, flags: {:?}) -> {:?}",
        devPtr, size, flags, result
    );
    result
}

pub unsafe extern "C" fn cuDeviceGetGraphMemAttribute(
    value: *mut size_t,
    attrib: CUdevice_attribute,
    device: CUdevice,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut size_t, CUdevice_attribute, CUdevice) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceGetGraphMemAttribute").unwrap();

    let result = func(value, attrib, device);
    eprintln!(
        "cuDeviceGetGraphMemAttribute(value: {:?}, attrib: {:?}, device: {:?}) -> {:?}",
        value, attrib, device, result
    );
    result
}

pub unsafe extern "C" fn cuDeviceSetGraphMemAttribute(
    device: CUdevice,
    attrib: CUdevice_attribute,
    value: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUdevice, CUdevice_attribute, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuDeviceSetGraphMemAttribute").unwrap();

    let result = func(device, attrib, value);
    eprintln!(
        "cuDeviceSetGraphMemAttribute(device: {:?}, attrib: {:?}, value: {:?}) -> {:?}",
        device, attrib, value, result
    );
    result
}

pub unsafe extern "C" fn cuGraphClone(
    phGraphClone: *mut CUgraph,
    hGraph: CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraph, CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphClone").unwrap();

    let result = func(phGraphClone, hGraph);
    eprintln!(
        "cuGraphClone(phGraphClone: {:?}, hGraph: {:?}) -> {:?}",
        phGraphClone, hGraph, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeFindInClone(
    phNode: *mut CUgraphNode,
    hOriginalNode: CUgraphNode,
    hClonedGraph: CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphNode, CUgraphNode, CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeFindInClone").unwrap();

    let result = func(phNode, hOriginalNode, hClonedGraph);
    eprintln!(
        "cuGraphNodeFindInClone(phNode: {:?}, hOriginalNode: {:?}, hClonedGraph: {:?}) -> {:?}",
        phNode, hOriginalNode, hClonedGraph, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeGetType(
    hNode: CUgraphNode,
    pType: *mut CUgraphNodeType_enum,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUgraphNodeType_enum) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeGetType").unwrap();

    let result = func(hNode, pType);
    eprintln!(
        "cuGraphNodeGetType(hNode: {:?}, pType: {:?}) -> {:?}",
        hNode, pType, result
    );
    result
}

pub unsafe extern "C" fn cuGraphGetNodes(
    hGraph: CUgraph,
    nodes: *mut CUgraphNode,
    numNodes: *mut size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *mut CUgraphNode, *mut size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphGetNodes").unwrap();

    let result = func(hGraph, nodes, numNodes);
    eprintln!(
        "cuGraphGetNodes(hGraph: {:?}, nodes: {:?}, numNodes: {:?}) -> {:?}",
        hGraph, nodes, numNodes, result
    );
    result
}

pub unsafe extern "C" fn cuGraphGetRootNodes(
    hGraph: CUgraph,
    rootNodes: *mut CUgraphNode,
    numRootNodes: *mut size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *mut CUgraphNode, *mut size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphGetRootNodes").unwrap();

    let result = func(hGraph, rootNodes, numRootNodes);
    eprintln!(
        "cuGraphGetRootNodes(hGraph: {:?}, rootNodes: {:?}, numRootNodes: {:?}) -> {:?}",
        hGraph, rootNodes, numRootNodes, result
    );
    result
}

pub unsafe extern "C" fn cuGraphGetEdges(
    hGraph: CUgraph,
    from: *mut CUgraphNode,
    to: *mut CUgraphNode,
    numEdges: *mut size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *mut CUgraphNode, *mut CUgraphNode, *mut size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphGetEdges").unwrap();

    let result = func(hGraph, from, to, numEdges);
    eprintln!(
        "cuGraphGetEdges(hGraph: {:?}, from: {:?}, to: {:?}, numEdges: {:?}) -> {:?}",
        hGraph, from, to, numEdges, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeGetDependencies(
    hNode: CUgraphNode,
    dependencies: *mut CUgraphNode,
    numDependencies: *mut size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUgraphNode, *mut size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeGetDependencies").unwrap();

    let result = func(hNode, dependencies, numDependencies);
    eprintln!(
        "cuGraphNodeGetDependencies(hNode: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        hNode, dependencies, numDependencies, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeGetDependentNodes(
    hNode: CUgraphNode,
    dependentNodes: *mut CUgraphNode,
    numDependentNodes: *mut size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUgraphNode, *mut size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeGetDependentNodes").unwrap();

    let result = func(hNode, dependentNodes, numDependentNodes);
    eprintln!(
        "cuGraphNodeGetDependentNodes(hNode: {:?}, dependentNodes: {:?}, numDependentNodes: {:?}) -> {:?}",
        hNode, dependentNodes, numDependentNodes, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddDependencies(
    hGraph: CUgraph,
    from: *const CUgraphNode,
    to: *const CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, *const CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddDependencies").unwrap();

    let result = func(hGraph, from, to, numDependencies);
    eprintln!(
        "cuGraphAddDependencies(hGraph: {:?}, from: {:?}, to: {:?}, numDependencies: {:?}) -> {:?}",
        hGraph, from, to, numDependencies, result
    );
    result
}

pub unsafe extern "C" fn cuGraphRemoveDependencies(
    hGraph: CUgraph,
    from: *const CUgraphNode,
    to: *const CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const CUgraphNode, *const CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphRemoveDependencies").unwrap();

    let result = func(hGraph, from, to, numDependencies);
    eprintln!(
        "cuGraphRemoveDependencies(hGraph: {:?}, from: {:?}, to: {:?}, numDependencies: {:?}) -> {:?}",
        hGraph, from, to, numDependencies, result
    );
    result
}

pub unsafe extern "C" fn cuGraphDestroyNode(hNode: CUgraphNode) -> CUresult {
    let func: libloading::Symbol<unsafe extern "C" fn(CUgraphNode) -> CUresult> =
        LIBCUDA.get(b"cuGraphDestroyNode").unwrap();

    let result = func(hNode);
    eprintln!("cuGraphDestroyNode(hNode: {:?}) -> {:?}", hNode, result);
    result
}

pub unsafe extern "C" fn cuGraphInstantiate(
    phGraphExec: *mut CUgraphExec,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphExec, CUgraph, *mut CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiate").unwrap();

    let result = func(phGraphExec, hGraph, dependencies, numDependencies);
    eprintln!(
        "cuGraphInstantiate(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        phGraphExec, hGraph, dependencies, numDependencies, result
    );
    result
}

pub unsafe extern "C" fn cuGraphInstantiateWithFlags(
    phGraphExec: *mut CUgraphExec,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: size_t,
    flags: CUgraphExec_st,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            *mut CUgraphExec,
            CUgraph,
            *mut CUgraphNode,
            size_t,
            CUgraphExec_st,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiateWithFlags").unwrap();

    let result = func(phGraphExec, hGraph, dependencies, numDependencies, flags);
    eprintln!(
        "cuGraphInstantiateWithFlags(phGraphExec: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, flags: {:?}) -> {:?}",
        phGraphExec, hGraph, dependencies, numDependencies, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphUpload(
    hGraphExec: CUgraphExec,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphUpload").unwrap();

    let result = func(hGraphExec, stream);
    eprintln!(
        "cuGraphUpload(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        hGraphExec, stream, result
    );
    result
}

pub unsafe extern "C" fn cuGraphLaunch(
    hGraphExec: CUgraphExec,
    stream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphLaunch").unwrap();

    let result = func(hGraphExec, stream);
    eprintln!(
        "cuGraphLaunch(hGraphExec: {:?}, stream: {:?}) -> {:?}",
        hGraphExec, stream, result
    );
    result
}

>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
pub unsafe extern "C" fn cuGraphExecDestroy(
    hGraphExec: CUgraphExec,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecDestroy").unwrap();

    let result = func(hGraphExec);
<<<<<<< HEAD
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    eprintln!(
        "cuGraphExecDestroy(hGraphExec: {:?}) -> {:?}",
        hGraphExec, result
    );
    eprintln!(
        ">>> cuGraphExecDestroy(hGraphExec: {:?}) -> {:?}",
        actual_hGraphExec, CUresult::CUDA_SUCCESS
    );
=======
    eprintln!("cuGraphExecDestroy(hGraphExec: {:?}) -> {:?}", hGraphExec, result);
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuGraphDestroy(
    hGraph: CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphDestroy").unwrap();

    let result = func(hGraph);
<<<<<<< HEAD
    let actual_hGraph: CUgraph = hGraph;
    eprintln!(
        "cuGraphDestroy(hGraph: {:?}) -> {:?}",
        hGraph, result
    );
    eprintln!(
        ">>> cuGraphDestroy(hGraph: {:?}) -> {:?}",
        actual_hGraph, CUresult::CUDA_SUCCESS
    );
=======
    eprintln!("cuGraphDestroy(hGraph: {:?}) -> {:?}", hGraph, result);
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuStreamBeginCapture(
    stream: CUstream,
    mode: CUstreamCaptureMode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUstreamCaptureMode) -> CUresult,
    > = LIBCUDA.get(b"cuStreamBeginCapture").unwrap();

    let result = func(stream, mode);
<<<<<<< HEAD
    let actual_stream: CUstream = stream;
    let actual_mode: CUstreamCaptureMode = mode;
    eprintln!(
        "cuStreamBeginCapture(stream: {:?}, mode: {:?}) -> {:?}",
        stream, mode, result
    );
    eprintln!(
        ">>> cuStreamBeginCapture(stream: {:?}, mode: {:?}) -> {:?}",
        actual_stream, actual_mode, CUresult::CUDA_SUCCESS
    );
=======
    eprintln!("cuStreamBeginCapture(stream: {:?}, mode: {:?}) -> {:?}", stream, mode, result);
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuStreamEndCapture(
    stream: CUstream,
    phCaptureGraph: *mut CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuStreamEndCapture").unwrap();

    let result = func(stream, phCaptureGraph);
<<<<<<< HEAD
    let actual_stream: CUstream = stream;
    let actual_phCaptureGraph: *mut CUgraph = phCaptureGraph;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuStreamEndCapture(stream: {:?}, phCaptureGraph: {:?}) -> {:?}",
        stream, phCaptureGraph, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuStreamEndCapture(stream: {:?}, phCaptureGraph: {:?}) -> {:?}",
        actual_stream, actual_phCaptureGraph, CUresult::CUDA_SUCCESS
    );
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuStreamIsCapturing(
    stream: CUstream,
    pCaptureStatus: *mut CUstreamCaptureStatus,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut CUstreamCaptureStatus) -> CUresult,
    > = LIBCUDA.get(b"cuStreamIsCapturing").unwrap();

    let result = func(stream, pCaptureStatus);
<<<<<<< HEAD
    let actual_stream: CUstream = stream;
    let actual_pCaptureStatus: *mut CUstreamCaptureStatus = pCaptureStatus;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuStreamIsCapturing(stream: {:?}, pCaptureStatus: {:?}) -> {:?}",
        stream, pCaptureStatus, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuStreamIsCapturing(stream: {:?}, pCaptureStatus: {:?}) -> {:?}",
        actual_stream, actual_pCaptureStatus, CUresult::CUDA_SUCCESS
    );
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuStreamGetCaptureInfo(
    stream: CUstream,
    pInfo: *mut CUstreamCaptureStatus,
    pId: *mut u64,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, *mut CUstreamCaptureStatus, *mut u64) -> CUresult,
    > = LIBCUDA.get(b"cuStreamGetCaptureInfo").unwrap();

    let result = func(stream, pInfo, pId);
<<<<<<< HEAD
    let actual_stream: CUstream = stream;
    let actual_pInfo: CUstreamCaptureStatus = *pInfo;
    let actual_pId: u64 = *pId;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuStreamGetCaptureInfo(stream: {:?}, pInfo: {:?}, pId: {:?}) -> {:?}",
        stream, pInfo, pId, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuStreamGetCaptureInfo(stream: {:?}, pInfo: {:?}, pId: {:?}) -> {:?}",
        actual_stream, actual_pInfo, actual_pId, CUresult::CUDA_SUCCESS
    );
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuStreamUpdateCaptureDependencies(
    stream: CUstream,
    graph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: size_t,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUstream, CUgraph, *mut CUgraphNode, size_t) -> CUresult,
    > = LIBCUDA.get(b"cuStreamUpdateCaptureDependencies").unwrap();

    let result = func(stream, graph, dependencies, numDependencies);
<<<<<<< HEAD
    let actual_stream: CUstream = stream;
    let actual_graph: CUgraph = graph;
    let actual_dependencies: *mut CUgraphNode = dependencies;
    let actual_numDependencies: size_t = numDependencies;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuStreamUpdateCaptureDependencies(stream: {:?}, graph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        stream, graph, dependencies, numDependencies, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuStreamUpdateCaptureDependencies(stream: {:?}, graph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        actual_stream, actual_graph, actual_dependencies, actual_numDependencies, CUresult::CUDA_SUCCESS
    );
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    result
}

pub unsafe extern "C" fn cuGraphExecKernelNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    nodeParams: *const cudaKernelNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const cudaKernelNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecKernelNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, nodeParams);
<<<<<<< HEAD
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_hNode: CUgraphNode = hNode;
    let actual_nodeParams: *const cudaKernelNodeParams = nodeParams;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuGraphExecKernelNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuGraphExecKernelNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        actual_hGraphExec, actual_hNode, actual_nodeParams, CUresult::CUDA_SUCCESS
=======
    result
}

pub unsafe extern "C" fn cuGraphExecMemcpyNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    nodeParams: *const c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecMemcpyNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, nodeParams);
    eprintln!(
        "cuGraphExecMemcpyNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    );
    result
}

pub unsafe extern "C" fn cuGraphExecMemsetNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    nodeParams: *const c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecMemsetNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, nodeParams);
<<<<<<< HEAD
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_hNode: CUgraphNode = hNode;
    let actual_nodeParams: *const c_uint = nodeParams;
=======
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    eprintln!(
        "cuGraphExecMemsetNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
    );
<<<<<<< HEAD
    eprintln!(
        ">>> cuGraphExecMemsetNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        actual_hGraphExec, actual_hNode, actual_nodeParams, CUresult::CUDA_SUCCESS
=======
    result
}

pub unsafe extern "C" fn cuGraphExecHostNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    nodeParams: *const CUgraphNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *const CUgraphNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecHostNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, nodeParams);
    eprintln!(
        "cuGraphExecHostNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecChildGraphNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    hGraph: CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecChildGraphNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, hGraph);
    eprintln!(
        "cuGraphExecChildGraphNodeSetParams(hGraphExec: {:?}, hNode: {:?}, hGraph: {:?}) -> {:?}",
        hGraphExec, hNode, hGraph, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecEventRecordNodeSetEvent(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecEventRecordNodeSetEvent").unwrap();

    let result = func(hGraphExec, hNode, event);
    eprintln!(
        "cuGraphExecEventRecordNodeSetEvent(hGraphExec: {:?}, hNode: {:?}, event: {:?}) -> {:?}",
        hGraphExec, hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecEventWaitNodeSetEvent(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    event: CUevent,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, CUevent) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecEventWaitNodeSetEvent").unwrap();

    let result = func(hGraphExec, hNode, event);
    eprintln!(
        "cuGraphExecEventWaitNodeSetEvent(hGraphExec: {:?}, hNode: {:?}, event: {:?}) -> {:?}",
        hGraphExec, hNode, event, result
    );
    result
}

pub unsafe extern "C" fn cuThreadExchangeStreamCaptureMode(
    mode: *mut CUstreamCaptureMode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUstreamCaptureMode) -> CUresult,
    > = LIBCUDA.get(b"cuThreadExchangeStreamCaptureMode").unwrap();

    let result = func(mode);
    eprintln!(
        "cuThreadExchangeStreamCaptureMode(mode: {:?}) -> {:?}",
        mode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecUpdate(
    hGraphExec: CUgraphExec,
    hStream: CUstream,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUstream) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecUpdate").unwrap();

    let result = func(hGraphExec, hStream);
    eprintln!(
        "cuGraphExecUpdate(hGraphExec: {:?}, hStream: {:?}) -> {:?}",
        hGraphExec, hStream, result
    );
    result
}

pub unsafe extern "C" fn cuGraphKernelNodeCopyAttributes(
    dstNode: CUgraphNode,
    srcNode: CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphKernelNodeCopyAttributes").unwrap();

    let result = func(dstNode, srcNode);
    eprintln!(
        "cuGraphKernelNodeCopyAttributes(dstNode: {:?}, srcNode: {:?}) -> {:?}",
        dstNode, srcNode, result
    );
    result
}

pub unsafe extern "C" fn cuGraphKernelNodeGetAttribute(
    hNode: CUgraphNode,
    attrib: CUkernelNodeAttrID,
    value: *mut std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, CUkernelNodeAttrID, *mut std::ffi::c_void) -> CUresult,
    > = LIBCUDA.get(b"cuGraphKernelNodeGetAttribute").unwrap();

    let result = func(hNode, attrib, value);
    eprintln!(
        "cuGraphKernelNodeGetAttribute(hNode: {:?}, attrib: {:?}, value: {:?}) -> {:?}",
        hNode, attrib, value, result
    );
    result
}

pub unsafe extern "C" fn cuGraphKernelNodeSetAttribute(
    hNode: CUgraphNode,
    attrib: CUkernelNodeAttrID,
    value: *const std::ffi::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, CUkernelNodeAttrID, *const std::ffi::c_void) -> CUresult,
    > = LIBCUDA.get(b"cuGraphKernelNodeSetAttribute").unwrap();

    let result = func(hNode, attrib, value);
    eprintln!(
        "cuGraphKernelNodeSetAttribute(hNode: {:?}, attrib: {:?}, value: {:?}) -> {:?}",
        hNode, attrib, value, result
    );
    result
}

pub unsafe extern "C" fn cuGraphDebugDotPrint(
    hGraph: CUgraph,
    file: *const std::os::raw::c_char,
    flags: std::os::raw::c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph, *const std::os::raw::c_char, std::os::raw::c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphDebugDotPrint").unwrap();

    let result = func(hGraph, file, flags);
    eprintln!(
        "cuGraphDebugDotPrint(hGraph: {:?}, file: {:?}, flags: {:?}) -> {:?}",
        hGraph, file, flags, result
    );
    result
}

pub unsafe extern "C" fn cuUserObjectCreate(
    phObject: *mut c_uint,
    pCreateInfo: *mut c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut c_uint, *mut c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuUserObjectCreate").unwrap();

    let result = func(phObject, pCreateInfo);
    eprintln!(
        "cuUserObjectCreate(phObject: {:?}, pCreateInfo: {:?}) -> {:?}",
        phObject, pCreateInfo, result
    );
    result
}

pub unsafe extern "C" fn cuUserObjectRetain(
    hObject: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuUserObjectRetain").unwrap();

    let result = func(hObject);
    eprintln!(
        "cuUserObjectRetain(hObject: {:?}) -> {:?}",
        hObject, result
    );
    result
}

pub unsafe extern "C" fn cuUserObjectRelease(
    hObject: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuUserObjectRelease").unwrap();

    let result = func(hObject);
    eprintln!(
        "cuUserObjectRelease(hObject: {:?}) -> {:?}",
        hObject, result
    );
    result
}

pub unsafe extern "C" fn cuGraphRetainUserObject(
    hObject: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphRetainUserObject").unwrap();

    let result = func(hObject);
    eprintln!(
        "cuGraphRetainUserObject(hObject: {:?}) -> {:?}",
        hObject, result
    );
    result
}

pub unsafe extern "C" fn cuGraphReleaseUserObject(
    hObject: c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphReleaseUserObject").unwrap();

    let result = func(hObject);
    eprintln!(
        "cuGraphReleaseUserObject(hObject: {:?}) -> {:?}",
        hObject, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeSetEnabled(
    hNode: CUgraphNode,
    bEnabled: std::os::raw::c_int,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, std::os::raw::c_int) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeSetEnabled").unwrap();

    let result = func(hNode, bEnabled);
    eprintln!(
        "cuGraphNodeSetEnabled(hNode: {:?}, bEnabled: {:?}) -> {:?}",
        hNode, bEnabled, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeGetEnabled(
    hNode: CUgraphNode,
    pbEnabled: *mut std::os::raw::c_int,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut std::os::raw::c_int) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeGetEnabled").unwrap();

    let result = func(hNode, pbEnabled);
    eprintln!(
        "cuGraphNodeGetEnabled(hNode: {:?}, pbEnabled: {:?}) -> {:?}",
        hNode, pbEnabled, result
    );
    result
}

pub unsafe extern "C" fn cuGraphInstantiateWithParams(
    hGraphExec: CUgraphExec,
    hGraph: CUgraph,
    extra: *mut std::os::raw::c_void,
    params: *mut CUgraphExec_st,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(
            CUgraphExec,
            CUgraph,
            *mut std::os::raw::c_void,
            *mut CUgraphExec_st,
        ) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiateWithParams").unwrap();

    let result = func(hGraphExec, hGraph, extra, params);
    eprintln!(
        "cuGraphInstantiateWithParams(hGraphExec: {:?}, hGraph: {:?}, extra: {:?}, params: {:?}) -> {:?}",
        hGraphExec, hGraph, extra, params, result
    );
    result
}

pub unsafe extern "C" fn cuGraphInstantiateWithParams_ptsz(
    phGraphExec: *mut CUgraphExec,
    hGraph: CUgraph,
    params: *mut CUgraphExec_st,
    extra: *mut std::os::raw::c_void,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphExec, CUgraph, *mut CUgraphExec_st, *mut std::os::raw::c_void) -> CUresult,
    > = LIBCUDA.get(b"cuGraphInstantiateWithParams_ptsz").unwrap();

    let result = func(phGraphExec, hGraph, params, extra);
    eprintln!(
        "cuGraphInstantiateWithParams_ptsz(phGraphExec: {:?}, hGraph: {:?}, params: {:?}, extra: {:?}) -> {:?}",
        phGraphExec, hGraph, params, extra, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecGetFlags(
    hGraphExec: CUgraphExec,
    flags: *mut std::os::raw::c_uint,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, *mut std::os::raw::c_uint) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecGetFlags").unwrap();

    let result = func(hGraphExec, flags);
    eprintln!(
        "cuGraphExecGetFlags(hGraphExec: {:?}, flags: {:?}) -> {:?}",
        hGraphExec, flags, result
    );
    result
}

pub unsafe extern "C" fn cuGraphAddNode(
    phGraphNode: *mut CUgraphNode,
    hGraph: CUgraph,
    dependencies: *mut CUgraphNode,
    numDependencies: std::os::raw::c_uint,
    node: *mut CUgraphNode,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(*mut CUgraphNode, CUgraph, *mut CUgraphNode, std::os::raw::c_uint, *mut CUgraphNode) -> CUresult,
    > = LIBCUDA.get(b"cuGraphAddNode").unwrap();

    let result = func(phGraphNode, hGraph, dependencies, numDependencies, node);
    eprintln!(
        "cuGraphAddNode(phGraphNode: {:?}, hGraph: {:?}, dependencies: {:?}, numDependencies: {:?}, node: {:?}) -> {:?}",
        phGraphNode, hGraph, dependencies, numDependencies, node, result
    );
    result
}

pub unsafe extern "C" fn cuGraphNodeSetParams(
    hNode: CUgraphNode,
    nodeParams: *mut CUgraphNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphNode, *mut CUgraphNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphNodeSetParams").unwrap();

    let result = func(hNode, nodeParams);
    eprintln!(
        "cuGraphNodeSetParams(hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hNode, nodeParams, result
    );
    result
}

pub unsafe extern "C" fn cuGraphExecNodeSetParams(
    hGraphExec: CUgraphExec,
    hNode: CUgraphNode,
    nodeParams: *mut CUgraphNodeParams,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec, CUgraphNode, *mut CUgraphNodeParams) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecNodeSetParams").unwrap();

    let result = func(hGraphExec, hNode, nodeParams);
    eprintln!(
        "cuGraphExecNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
>>>>>>> 1d9fa9cdb73d57785fe8516cb9527cb594a08b26
    );
    result
}
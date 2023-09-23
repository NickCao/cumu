#![allow(non_snake_case)]
use libc::size_t;
use cuda_sys::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void, c_ulonglong};
use std::sync::Mutex;


lazy_static::lazy_static! {
    static ref LIBCUDA: libloading::Library = unsafe {
        libloading::Library::new(std::env::var("LIBCUDA").unwrap_or("/usr/lib/wsl/lib/libcuda.so".to_string())).unwrap()
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
        ">>> cuGetExportTable(ppExportTable: {:?}, pExportTableId: {:?}) -> {:?}",
        actual_ppExportTable,
        actual_pExportTableId,
        CUresult::CUDA_SUCCESS
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

pub unsafe extern "C" fn cuGraphExecDestroy(
    hGraphExec: CUgraphExec,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraphExec) -> CUresult,
    > = LIBCUDA.get(b"cuGraphExecDestroy").unwrap();

    let result = func(hGraphExec);
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    eprintln!(
        "cuGraphExecDestroy(hGraphExec: {:?}) -> {:?}",
        hGraphExec, result
    );
    eprintln!(
        ">>> cuGraphExecDestroy(hGraphExec: {:?}) -> {:?}",
        actual_hGraphExec, CUresult::CUDA_SUCCESS
    );
    result
}

pub unsafe extern "C" fn cuGraphDestroy(
    hGraph: CUgraph,
) -> CUresult {
    let func: libloading::Symbol<
        unsafe extern "C" fn(CUgraph) -> CUresult,
    > = LIBCUDA.get(b"cuGraphDestroy").unwrap();

    let result = func(hGraph);
    let actual_hGraph: CUgraph = hGraph;
    eprintln!(
        "cuGraphDestroy(hGraph: {:?}) -> {:?}",
        hGraph, result
    );
    eprintln!(
        ">>> cuGraphDestroy(hGraph: {:?}) -> {:?}",
        actual_hGraph, CUresult::CUDA_SUCCESS
    );
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
    let actual_stream: CUstream = stream;
    let actual_phCaptureGraph: *mut CUgraph = phCaptureGraph;
    eprintln!(
        "cuStreamEndCapture(stream: {:?}, phCaptureGraph: {:?}) -> {:?}",
        stream, phCaptureGraph, result
    );
    eprintln!(
        ">>> cuStreamEndCapture(stream: {:?}, phCaptureGraph: {:?}) -> {:?}",
        actual_stream, actual_phCaptureGraph, CUresult::CUDA_SUCCESS
    );
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
    let actual_stream: CUstream = stream;
    let actual_pCaptureStatus: *mut CUstreamCaptureStatus = pCaptureStatus;
    eprintln!(
        "cuStreamIsCapturing(stream: {:?}, pCaptureStatus: {:?}) -> {:?}",
        stream, pCaptureStatus, result
    );
    eprintln!(
        ">>> cuStreamIsCapturing(stream: {:?}, pCaptureStatus: {:?}) -> {:?}",
        actual_stream, actual_pCaptureStatus, CUresult::CUDA_SUCCESS
    );
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
    let actual_stream: CUstream = stream;
    let actual_pInfo: CUstreamCaptureStatus = *pInfo;
    let actual_pId: u64 = *pId;
    eprintln!(
        "cuStreamGetCaptureInfo(stream: {:?}, pInfo: {:?}, pId: {:?}) -> {:?}",
        stream, pInfo, pId, result
    );
    eprintln!(
        ">>> cuStreamGetCaptureInfo(stream: {:?}, pInfo: {:?}, pId: {:?}) -> {:?}",
        actual_stream, actual_pInfo, actual_pId, CUresult::CUDA_SUCCESS
    );
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
    let actual_stream: CUstream = stream;
    let actual_graph: CUgraph = graph;
    let actual_dependencies: *mut CUgraphNode = dependencies;
    let actual_numDependencies: size_t = numDependencies;
    eprintln!(
        "cuStreamUpdateCaptureDependencies(stream: {:?}, graph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        stream, graph, dependencies, numDependencies, result
    );
    eprintln!(
        ">>> cuStreamUpdateCaptureDependencies(stream: {:?}, graph: {:?}, dependencies: {:?}, numDependencies: {:?}) -> {:?}",
        actual_stream, actual_graph, actual_dependencies, actual_numDependencies, CUresult::CUDA_SUCCESS
    );
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
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_hNode: CUgraphNode = hNode;
    let actual_nodeParams: *const cudaKernelNodeParams = nodeParams;
    eprintln!(
        "cuGraphExecKernelNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
    );
    eprintln!(
        ">>> cuGraphExecKernelNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        actual_hGraphExec, actual_hNode, actual_nodeParams, CUresult::CUDA_SUCCESS
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
    let actual_hGraphExec: CUgraphExec = hGraphExec;
    let actual_hNode: CUgraphNode = hNode;
    let actual_nodeParams: *const c_uint = nodeParams;
    eprintln!(
        "cuGraphExecMemsetNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        hGraphExec, hNode, nodeParams, result
    );
    eprintln!(
        ">>> cuGraphExecMemsetNodeSetParams(hGraphExec: {:?}, hNode: {:?}, nodeParams: {:?}) -> {:?}",
        actual_hGraphExec, actual_hNode, actual_nodeParams, CUresult::CUDA_SUCCESS
    );
    result
}
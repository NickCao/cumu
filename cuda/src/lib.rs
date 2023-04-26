#![allow(non_snake_case)]
use cuda_sys::*;
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_uint, c_void},
};

extern "C" fn nop() -> c_int {
    0
}

static EXPORT: [extern "C" fn() -> c_int; 5] = [nop, nop, nop, nop, nop];

#[no_mangle]
pub unsafe extern "C" fn cuGetProcAddress_v2(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    cudaVersion: c_int,
    flags: cuuint64_t,
    status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    let symbol1 = CStr::from_ptr(symbol).to_str().unwrap();
    eprintln!(
        "cuGetProcAddress_v2(symbol: {}, version: {}, flags: {})",
        &symbol1, cudaVersion, flags,
    );
    *pfn = match symbol1 {
        "cuInit" => cuInit as _,
        "cuGetProcAddress" => cuGetProcAddress_v2 as _,
        "cuDriverGetVersion" => cuDriverGetVersion as _,
        "cuGetExportTable" => cuGetExportTable as _,
        "cuModuleGetLoadingMode" => cuModuleGetLoadingMode as _,
        "cuDeviceGetCount" => cuDeviceGetCount as _,
        "cuDeviceGet" => cuDeviceGet as _,
        "cuDeviceGetName" => cuDeviceGetName as _,
        "cuDeviceTotalMem" => cuDeviceTotalMem_v2 as _,
        "cuArray3DCreate" => stub::<1> as _,
        "cuArray3DGetDescriptor" => stub::<2> as _,
        "cuArrayCreate" => stub::<3> as _,
        "cuArrayDestroy" => stub::<4> as _,
        "cuArrayGetDescriptor" => stub::<5> as _,
        "cuArrayGetMemoryRequirements" => stub::<6> as _,
        "cuArrayGetPlane" => stub::<7> as _,
        "cuArrayGetSparseProperties" => stub::<8> as _,
        "cuCtxCreate" => stub::<9> as _,
        "cuCtxDetach" => stub::<10> as _,
        "cuCtxDisablePeerAccess" => stub::<11> as _,
        "cuCtxEnablePeerAccess" => stub::<12> as _,
        "cuCtxGetApiVersion" => stub::<13> as _,
        "cuCtxGetCacheConfig" => stub::<14> as _,
        "cuCtxGetCurrent" => stub::<15> as _,
        "cuCtxGetDevice" => stub::<16> as _,
        "cuCtxGetFlags" => stub::<17> as _,
        "cuCtxGetLimit" => stub::<18> as _,
        "cuCtxGetSharedMemConfig" => stub::<19> as _,
        "cuCtxGetStreamPriorityRange" => stub::<20> as _,
        "cuCtxPopCurrent" => stub::<21> as _,
        "cuCtxPushCurrent" => stub::<22> as _,
        "cuCtxResetPersistingL2Cache" => stub::<23> as _,
        "cuCtxSetCacheConfig" => stub::<24> as _,
        "cuCtxSetCurrent" => stub::<25> as _,
        "cuCtxSetLimit" => stub::<26> as _,
        "cuCtxSetSharedMemConfig" => stub::<27> as _,
        "cuCtxSynchronize" => stub::<28> as _,
        "cuDestroyExternalMemory" => stub::<29> as _,
        "cuDestroyExternalSemaphore" => stub::<30> as _,
        "cuDeviceCanAccessPeer" => stub::<31> as _,
        "cuDeviceGetAttribute" => stub::<33> as _,
        "cuDeviceGetByPCIBusId" => stub::<34> as _,
        "cuDeviceGetDefaultMemPool" => stub::<36> as _,
        "cuDeviceGetGraphMemAttribute" => stub::<37> as _,
        "cuDeviceGetMemPool" => stub::<38> as _,
        "cuDeviceGetNvSciSyncAttributes" => stub::<40> as _,
        "cuDeviceGetP2PAttribute" => stub::<41> as _,
        "cuDeviceGetPCIBusId" => stub::<42> as _,
        "cuDeviceGetTexture1DLinearMaxWidth" => stub::<43> as _,
        "cuDeviceGetUuid" => stub::<44> as _,
        "cuDeviceGraphMemTrim" => stub::<45> as _,
        "cuDevicePrimaryCtxGetState" => stub::<46> as _,
        "cuDevicePrimaryCtxRelease" => stub::<47> as _,
        "cuDevicePrimaryCtxReset" => stub::<48> as _,
        "cuDevicePrimaryCtxRetain" => stub::<49> as _,
        "cuDevicePrimaryCtxSetFlags" => stub::<50> as _,
        "cuDeviceSetGraphMemAttribute" => stub::<51> as _,
        "cuDeviceSetMemPool" => stub::<52> as _,
        "cuEGLStreamConsumerAcquireFrame" => stub::<55> as _,
        "cuEGLStreamConsumerConnect" => stub::<56> as _,
        "cuEGLStreamConsumerConnectWithFlags" => stub::<57> as _,
        "cuEGLStreamConsumerDisconnect" => stub::<58> as _,
        "cuEGLStreamConsumerReleaseFrame" => stub::<59> as _,
        "cuEGLStreamProducerConnect" => stub::<60> as _,
        "cuEGLStreamProducerDisconnect" => stub::<61> as _,
        "cuEGLStreamProducerPresentFrame" => stub::<62> as _,
        "cuEGLStreamProducerReturnFrame" => stub::<63> as _,
        "cuEventCreate" => stub::<64> as _,
        "cuEventDestroy" => stub::<65> as _,
        "cuEventElapsedTime" => stub::<66> as _,
        "cuEventQuery" => stub::<67> as _,
        "cuEventRecord" => stub::<68> as _,
        "cuEventRecordWithFlags" => stub::<69> as _,
        "cuEventSynchronize" => stub::<70> as _,
        "cuExternalMemoryGetMappedBuffer" => stub::<71> as _,
        "cuExternalMemoryGetMappedMipmappedArray" => stub::<72> as _,
        "cuFlushGPUDirectRDMAWrites" => stub::<73> as _,
        "cuFuncGetAttribute" => stub::<74> as _,
        "cuFuncSetAttribute" => stub::<75> as _,
        "cuFuncSetCacheConfig" => stub::<76> as _,
        "cuFuncSetSharedMemConfig" => stub::<77> as _,
        "cuGLCtxCreate" => stub::<78> as _,
        "cuGLGetDevices" => stub::<79> as _,
        "cuGLMapBufferObject" => stub::<81> as _,
        "cuGLMapBufferObjectAsync" => stub::<82> as _,
        "cuGLRegisterBufferObject" => stub::<83> as _,
        "cuGLSetBufferObjectMapFlags" => stub::<84> as _,
        "cuGLUnmapBufferObject" => stub::<85> as _,
        "cuGLUnmapBufferObjectAsync" => stub::<86> as _,
        "cuGLUnregisterBufferObject" => stub::<87> as _,
        "cuGetErrorName" => stub::<88> as _,
        "cuGetErrorString" => stub::<89> as _,
        "cuGraphAddChildGraphNode" => stub::<92> as _,
        "cuGraphAddDependencies" => stub::<93> as _,
        "cuGraphAddEmptyNode" => stub::<94> as _,
        "cuGraphAddEventRecordNode" => stub::<95> as _,
        "cuGraphAddEventWaitNode" => stub::<96> as _,
        "cuGraphAddExternalSemaphoresSignalNode" => stub::<97> as _,
        "cuGraphAddExternalSemaphoresWaitNode" => stub::<98> as _,
        "cuGraphAddHostNode" => stub::<99> as _,
        "cuGraphAddKernelNode" => stub::<100> as _,
        "cuGraphAddMemAllocNode" => stub::<101> as _,
        "cuGraphAddMemFreeNode" => stub::<102> as _,
        "cuGraphAddMemcpyNode" => stub::<103> as _,
        "cuGraphAddMemsetNode" => stub::<104> as _,
        "cuGraphChildGraphNodeGetGraph" => stub::<105> as _,
        "cuGraphClone" => stub::<106> as _,
        "cuGraphCreate" => stub::<107> as _,
        "cuGraphDebugDotPrint" => stub::<108> as _,
        "cuGraphDestroy" => stub::<109> as _,
        "cuGraphDestroyNode" => stub::<110> as _,
        "cuGraphEventRecordNodeGetEvent" => stub::<111> as _,
        "cuGraphEventRecordNodeSetEvent" => stub::<112> as _,
        "cuGraphEventWaitNodeGetEvent" => stub::<113> as _,
        "cuGraphEventWaitNodeSetEvent" => stub::<114> as _,
        "cuGraphExecChildGraphNodeSetParams" => stub::<115> as _,
        "cuGraphExecDestroy" => stub::<116> as _,
        "cuGraphExecEventRecordNodeSetEvent" => stub::<117> as _,
        "cuGraphExecEventWaitNodeSetEvent" => stub::<118> as _,
        "cuGraphExecExternalSemaphoresSignalNodeSetParams" => stub::<119> as _,
        "cuGraphExecExternalSemaphoresWaitNodeSetParams" => stub::<120> as _,
        "cuGraphExecGetFlags" => stub::<121> as _,
        "cuGraphExecHostNodeSetParams" => stub::<122> as _,
        "cuGraphExecKernelNodeSetParams" => stub::<123> as _,
        "cuGraphExecMemcpyNodeSetParams" => stub::<124> as _,
        "cuGraphExecMemsetNodeSetParams" => stub::<125> as _,
        "cuGraphExecUpdate" => stub::<126> as _,
        "cuGraphExternalSemaphoresSignalNodeGetParams" => stub::<127> as _,
        "cuGraphExternalSemaphoresSignalNodeSetParams" => stub::<128> as _,
        "cuGraphExternalSemaphoresWaitNodeGetParams" => stub::<129> as _,
        "cuGraphExternalSemaphoresWaitNodeSetParams" => stub::<130> as _,
        "cuGraphGetEdges" => stub::<131> as _,
        "cuGraphGetNodes" => stub::<132> as _,
        "cuGraphGetRootNodes" => stub::<133> as _,
        "cuGraphHostNodeGetParams" => stub::<134> as _,
        "cuGraphHostNodeSetParams" => stub::<135> as _,
        "cuGraphInstantiate" => stub::<136> as _,
        "cuGraphInstantiateWithFlags" => stub::<137> as _,
        "cuGraphInstantiateWithParams" => stub::<138> as _,
        "cuGraphInstantiateWithParams_ptsz" => stub::<139> as _,
        "cuGraphKernelNodeCopyAttributes" => stub::<140> as _,
        "cuGraphKernelNodeGetAttribute" => stub::<141> as _,
        "cuGraphKernelNodeGetParams" => stub::<142> as _,
        "cuGraphKernelNodeSetAttribute" => stub::<143> as _,
        "cuGraphKernelNodeSetParams" => stub::<144> as _,
        "cuGraphLaunch" => stub::<145> as _,
        "cuGraphMemAllocNodeGetParams" => stub::<146> as _,
        "cuGraphMemFreeNodeGetParams" => stub::<147> as _,
        "cuGraphMemcpyNodeGetParams" => stub::<148> as _,
        "cuGraphMemcpyNodeSetParams" => stub::<149> as _,
        "cuGraphMemsetNodeGetParams" => stub::<150> as _,
        "cuGraphMemsetNodeSetParams" => stub::<151> as _,
        "cuGraphNodeFindInClone" => stub::<152> as _,
        "cuGraphNodeGetDependencies" => stub::<153> as _,
        "cuGraphNodeGetDependentNodes" => stub::<154> as _,
        "cuGraphNodeGetEnabled" => stub::<155> as _,
        "cuGraphNodeGetType" => stub::<156> as _,
        "cuGraphNodeSetEnabled" => stub::<157> as _,
        "cuGraphReleaseUserObject" => stub::<158> as _,
        "cuGraphRemoveDependencies" => stub::<159> as _,
        "cuGraphRetainUserObject" => stub::<160> as _,
        "cuGraphUpload" => stub::<161> as _,
        "cuGraphicsEGLRegisterImage" => stub::<162> as _,
        "cuGraphicsGLRegisterBuffer" => stub::<163> as _,
        "cuGraphicsGLRegisterImage" => stub::<164> as _,
        "cuGraphicsMapResources" => stub::<165> as _,
        "cuGraphicsResourceGetMappedEglFrame" => stub::<166> as _,
        "cuGraphicsResourceGetMappedMipmappedArray" => stub::<167> as _,
        "cuGraphicsResourceGetMappedPointer" => stub::<168> as _,
        "cuGraphicsResourceSetMapFlags" => stub::<169> as _,
        "cuGraphicsSubResourceGetMappedArray" => stub::<170> as _,
        "cuGraphicsUnmapResources" => stub::<171> as _,
        "cuGraphicsUnregisterResource" => stub::<172> as _,
        "cuGraphicsVDPAURegisterOutputSurface" => stub::<173> as _,
        "cuGraphicsVDPAURegisterVideoSurface" => stub::<174> as _,
        "cuImportExternalMemory" => stub::<175> as _,
        "cuImportExternalSemaphore" => stub::<176> as _,
        "cuIpcCloseMemHandle" => stub::<178> as _,
        "cuIpcGetEventHandle" => stub::<179> as _,
        "cuIpcGetMemHandle" => stub::<180> as _,
        "cuIpcOpenEventHandle" => stub::<181> as _,
        "cuIpcOpenMemHandle" => stub::<182> as _,
        "cuKernelGetAttribute" => stub::<183> as _,
        "cuKernelGetFunction" => stub::<184> as _,
        "cuKernelSetAttribute" => stub::<185> as _,
        "cuKernelSetCacheConfig" => stub::<186> as _,
        "cuLaunchCooperativeKernel" => stub::<187> as _,
        "cuLaunchCooperativeKernelMultiDevice" => stub::<188> as _,
        "cuLaunchHostFunc" => stub::<189> as _,
        "cuLaunchKernel" => stub::<190> as _,
        "cuLaunchKernelEx" => stub::<191> as _,
        "cuLibraryGetGlobal" => stub::<192> as _,
        "cuLibraryGetKernel" => stub::<193> as _,
        "cuLibraryGetManaged" => stub::<194> as _,
        "cuLibraryGetModule" => stub::<195> as _,
        "cuLibraryLoadData" => stub::<196> as _,
        "cuLibraryLoadFromFile" => stub::<197> as _,
        "cuLibraryUnload" => stub::<198> as _,
        "cuLinkAddData" => stub::<199> as _,
        "cuLinkAddFile" => stub::<200> as _,
        "cuLinkComplete" => stub::<201> as _,
        "cuLinkCreate" => stub::<202> as _,
        "cuLinkDestroy" => stub::<203> as _,
        "cuMemAdvise" => stub::<204> as _,
        "cuMemAlloc" => stub::<205> as _,
        "cuMemAllocAsync" => stub::<206> as _,
        "cuMemAllocFromPoolAsync" => stub::<207> as _,
        "cuMemAllocManaged" => stub::<208> as _,
        "cuMemAllocPitch" => stub::<209> as _,
        "cuMemFree" => stub::<210> as _,
        "cuMemFreeAsync" => stub::<211> as _,
        "cuMemFreeHost" => stub::<212> as _,
        "cuMemGetAddressRange" => stub::<213> as _,
        "cuMemGetInfo" => stub::<214> as _,
        "cuMemHostAlloc" => stub::<215> as _,
        "cuMemHostGetDevicePointer" => stub::<216> as _,
        "cuMemHostGetFlags" => stub::<217> as _,
        "cuMemHostRegister" => stub::<218> as _,
        "cuMemHostUnregister" => stub::<219> as _,
        "cuMemPoolCreate" => stub::<220> as _,
        "cuMemPoolDestroy" => stub::<221> as _,
        "cuMemPoolExportPointer" => stub::<222> as _,
        "cuMemPoolExportToShareableHandle" => stub::<223> as _,
        "cuMemPoolGetAccess" => stub::<224> as _,
        "cuMemPoolGetAttribute" => stub::<225> as _,
        "cuMemPoolImportFromShareableHandle" => stub::<226> as _,
        "cuMemPoolImportPointer" => stub::<227> as _,
        "cuMemPoolSetAccess" => stub::<228> as _,
        "cuMemPoolSetAttribute" => stub::<229> as _,
        "cuMemPoolTrimTo" => stub::<230> as _,
        "cuMemPrefetchAsync" => stub::<231> as _,
        "cuMemRangeGetAttribute" => stub::<232> as _,
        "cuMemRangeGetAttributes" => stub::<233> as _,
        "cuMemcpy" => stub::<234> as _,
        "cuMemcpy2DAsync" => stub::<235> as _,
        "cuMemcpy2DUnaligned" => stub::<236> as _,
        "cuMemcpy3D" => stub::<237> as _,
        "cuMemcpy3DAsync" => stub::<238> as _,
        "cuMemcpy3DPeer" => stub::<239> as _,
        "cuMemcpy3DPeerAsync" => stub::<240> as _,
        "cuMemcpyAsync" => stub::<241> as _,
        "cuMemcpyDtoD" => stub::<242> as _,
        "cuMemcpyDtoDAsync" => stub::<243> as _,
        "cuMemcpyDtoH" => stub::<244> as _,
        "cuMemcpyDtoHAsync" => stub::<245> as _,
        "cuMemcpyHtoD" => stub::<246> as _,
        "cuMemcpyHtoDAsync" => stub::<247> as _,
        "cuMemcpyPeer" => stub::<248> as _,
        "cuMemcpyPeerAsync" => stub::<249> as _,
        "cuMemsetD2D8" => stub::<250> as _,
        "cuMemsetD2D8Async" => stub::<251> as _,
        "cuMemsetD8" => stub::<252> as _,
        "cuMemsetD8Async" => stub::<253> as _,
        "cuMipmappedArrayCreate" => stub::<254> as _,
        "cuMipmappedArrayDestroy" => stub::<255> as _,
        "cuMipmappedArrayGetLevel" => stub::<256> as _,
        "cuMipmappedArrayGetMemoryRequirements" => stub::<257> as _,
        "cuMipmappedArrayGetSparseProperties" => stub::<258> as _,
        "cuModuleGetFunction" => stub::<259> as _,
        "cuModuleGetGlobal" => stub::<260> as _,
        "cuModuleGetSurfRef" => stub::<262> as _,
        "cuModuleGetTexRef" => stub::<263> as _,
        "cuModuleLoad" => stub::<264> as _,
        "cuModuleLoadData" => stub::<265> as _,
        "cuModuleLoadFatBinary" => stub::<266> as _,
        "cuModuleUnload" => stub::<267> as _,
        "cuOccupancyAvailableDynamicSMemPerBlock" => stub::<268> as _,
        "cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags" => stub::<269> as _,
        "cuOccupancyMaxActiveClusters" => stub::<270> as _,
        "cuOccupancyMaxPotentialClusterSize" => stub::<271> as _,
        "cuPointerGetAttribute" => stub::<272> as _,
        "cuPointerGetAttributes" => stub::<273> as _,
        "cuProfilerInitialize" => stub::<274> as _,
        "cuProfilerStart" => stub::<275> as _,
        "cuProfilerStop" => stub::<276> as _,
        "cuSignalExternalSemaphoresAsync" => stub::<277> as _,
        "cuStreamAddCallback" => stub::<278> as _,
        "cuStreamAttachMemAsync" => stub::<279> as _,
        "cuStreamBatchMemOp" => stub::<280> as _,
        "cuStreamBeginCapture" => stub::<281> as _,
        "cuStreamCopyAttributes" => stub::<282> as _,
        "cuStreamCreate" => stub::<283> as _,
        "cuStreamCreateWithPriority" => stub::<284> as _,
        "cuStreamDestroy" => stub::<285> as _,
        "cuStreamEndCapture" => stub::<286> as _,
        "cuStreamGetAttribute" => stub::<287> as _,
        "cuStreamGetCaptureInfo" => stub::<288> as _,
        "cuStreamGetCtx" => stub::<289> as _,
        "cuStreamGetFlags" => stub::<290> as _,
        "cuStreamGetId" => stub::<291> as _,
        "cuStreamGetPriority" => stub::<292> as _,
        "cuStreamIsCapturing" => stub::<293> as _,
        "cuStreamQuery" => stub::<294> as _,
        "cuStreamSetAttribute" => stub::<295> as _,
        "cuStreamSynchronize" => stub::<296> as _,
        "cuStreamUpdateCaptureDependencies" => stub::<297> as _,
        "cuStreamWaitEvent" => stub::<298> as _,
        "cuStreamWaitValue32" => stub::<299> as _,
        "cuStreamWaitValue64" => stub::<300> as _,
        "cuStreamWriteValue32" => stub::<301> as _,
        "cuStreamWriteValue64" => stub::<302> as _,
        "cuSurfObjectCreate" => stub::<303> as _,
        "cuSurfObjectDestroy" => stub::<304> as _,
        "cuSurfObjectGetResourceDesc" => stub::<305> as _,
        "cuTexObjectCreate" => stub::<306> as _,
        "cuTexObjectDestroy" => stub::<307> as _,
        "cuTexObjectGetResourceDesc" => stub::<308> as _,
        "cuTexObjectGetResourceViewDesc" => stub::<309> as _,
        "cuTexObjectGetTextureDesc" => stub::<310> as _,
        "cuThreadExchangeStreamCaptureMode" => stub::<311> as _,
        "cuUserObjectCreate" => stub::<312> as _,
        "cuUserObjectRelease" => stub::<313> as _,
        "cuUserObjectRetain" => stub::<314> as _,
        "cuVDPAUCtxCreate" => stub::<315> as _,
        "cuVDPAUGetDevice" => stub::<316> as _,
        "cuWaitExternalSemaphoresAsync" => stub::<317> as _,
        _ => stub::<0> as _,
    };
    if !status.is_null() {
        *status = CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS;
    }
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn stub<const N: usize>() -> CUresult {
    eprintln!("cuUnkown{}", N);
    CUresult::CUDA_ERROR_UNKNOWN
}

unsafe extern "C" fn cuInit(_flags: c_uint) -> CUresult {
    eprintln!("cuInit");
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuDriverGetVersion(version: *mut c_int) -> CUresult {
    eprintln!("cuDriverGetVersion");
    *version = 1000 * 12 + 10 * 0;
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuGetExportTable(table: *mut *const c_void, id: *const CUuuid) -> CUresult {
    eprintln!("cuGetExportTable(id: {:?})", (*id).bytes);
    *table = EXPORT.as_ptr() as _;
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuModuleGetLoadingMode(mode: *mut CUmoduleLoadingMode) -> CUresult {
    eprintln!("cuModuleGetLoadingMode");
    *mode = CUmoduleLoadingMode::CU_MODULE_EAGER_LOADING;
    CUresult::CUDA_SUCCESS
}

unsafe extern "C" fn cuDeviceGetCount(count: *mut c_int) -> CUresult {
    eprintln!("cuDeviceGetCount");
    *count = 1;
    CUresult::CUDA_SUCCESS
}

pub unsafe extern "C" fn cuDeviceGet(device: *mut CUdevice, ordinal: c_int) -> CUresult {
    eprintln!("cuDeviceGet");
    *device = ordinal;
    CUresult::CUDA_SUCCESS
}

pub unsafe extern "C" fn cuDeviceGetName(name: *mut c_char, len: c_int, dev: CUdevice) -> CUresult {
    eprintln!("cuDeviceGetName(dev: {})", dev);
    let name = std::slice::from_raw_parts_mut(name as *mut u8, len.try_into().unwrap());
    let devname = CString::new("NVIDIA A100").unwrap();
    name[..devname.as_bytes().len()].copy_from_slice(devname.as_bytes());
    CUresult::CUDA_SUCCESS
}

pub unsafe extern "C" fn cuDeviceTotalMem_v2(bytes: *mut usize, dev: CUdevice) -> CUresult {
    eprintln!("cuDeviceTotalMem(dev: {})", dev);
    *bytes = 80 * 1024 * 1024 * 1024;
    CUresult::CUDA_SUCCESS
}

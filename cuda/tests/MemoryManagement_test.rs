// cudaMemcpy cudaMalloc cudaFree
#![allow(non_snake_case)]
use std::os::raw::{c_void, c_int};


// 假设默认的返回值
pub const CUDA_SUCCESS: c_int = 0;

// 函数作用：简单模拟 CUDA 的 cudaMemcpy 函数
pub unsafe extern "C" fn cudaMemcpy(
    dst: *mut c_void,
    src: *const c_void,
    count: usize,
    _kind: c_int,
) -> c_int {
    if dst.is_null() || src.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 这里只是简单地将源数据复制到目标数据，模拟 CUDA 的数据复制操作
    std::ptr::copy(src as *const u8, dst as *mut u8, count);

    return CUDA_SUCCESS;
}

// 函数作用：简单模拟 CUDA 的 cudaMalloc 函数
pub unsafe extern "C" fn cudaMalloc(
    devPtr: *mut *mut c_void,
    size: usize,
) -> c_int {
    if devPtr.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 这里只是简单地模拟 GPU 内存分配，使用 Vec 来模拟分配的 GPU 内存
    let mut memory = Vec::<u8>::with_capacity(size);
    let ptr = memory.as_mut_ptr() as *mut c_void;
    std::ptr::write(devPtr, ptr);

    return CUDA_SUCCESS;
}

// 函数作用：简单模拟 CUDA 的 cudaFree 函数
pub unsafe extern "C" fn cudaFree(
    devPtr: *mut *mut c_void,
) -> c_int {
    if devPtr.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 这里只是简单地模拟 GPU 内存释放，不执行实际操作

    return CUDA_SUCCESS;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cudaMemcpy() {
        // 假设源数据和目标数据
        let src_data: [u8; 4] = [1, 2, 3, 4];
        let mut dst_data: [u8; 4] = [0; 4];

        // 调用 cudaMemcpy 进行数据复制
        let result = unsafe {
            cudaMemcpy(
                dst_data.as_mut_ptr() as *mut c_void,
                src_data.as_ptr() as *const c_void,
                src_data.len(),
                0,
            )
        };

        assert_eq!(result, CUDA_SUCCESS, "Failed to copy data");
        assert_eq!(dst_data, src_data, "Data copy mismatch");
        print!(">>>>>>>>>>>>>>> ");
        println!("Data copied successfully");
    }

    #[test]
    fn test_cudaMalloc() {
        let size = 1024;

        // 调用 cudaMalloc 进行内存分配
        let mut devPtr: *mut c_void = std::ptr::null_mut();
        let result = unsafe { cudaMalloc(&mut devPtr, size) };

        assert_eq!(result, CUDA_SUCCESS, "Failed to allocate memory");
        assert_ne!(devPtr, std::ptr::null_mut(), "Memory allocation failed");
        print!(">>>>>>>>>>>>>>> ");
        println!("Memory allocation successful");
    }

    #[test]
    fn test_cudaFree() {
        let mut devPtr: *mut c_void = std::ptr::null_mut();

        // 调用 cudaFree 进行内存释放
        let result = unsafe { cudaFree(&mut devPtr) };

        assert_eq!(result, CUDA_SUCCESS, "Failed to free memory");
        print!(">>>>>>>>>>>>>>> ");
        println!("Memory deallocation successful");
    }
}
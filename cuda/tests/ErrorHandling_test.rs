// cudaGetLastError cudaGetErrorString
#![allow(non_snake_case)]
use std::os::raw::c_int;

// 假设默认的返回值
pub const CUDA_SUCCESS: c_int = 0;
// 假设默认的错误码
pub const CUDA_ERROR_UNKNOWN: c_int = 9999;
// 假设默认的错误字符串
pub const CUDA_ERROR_UNKNOWN_STR: &str = "Unknown error";

// 函数功能：获取最近一次 CUDA 函数调用的错误状态
pub unsafe fn cudaGetLastError() -> c_int {
    // 这里只是简单地返回默认的错误码
    return CUDA_ERROR_UNKNOWN;
}

// 函数功能：将 CUDA 错误码转换为相应的错误字符串
pub unsafe fn cudaGetErrorString(_error: c_int) -> String {
    // 这里只是简单地返回默认的错误字符串
    return CUDA_ERROR_UNKNOWN_STR.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cudaGetLastError() {
        // 调用 cudaGetLastError 获取错误状态
        let result = unsafe { cudaGetLastError() };

        assert_eq!(result, CUDA_ERROR_UNKNOWN, "Failed to get last error");
        print!(">>>>>>>>>>>>>>> ");
        println!("Last error: {}", result);
    }

    #[test]
    fn test_cudaGetErrorString() {
        let error = 9999; // 假设一个错误码

        // 调用 cudaGetErrorString 获取错误字符串
        let result = unsafe { cudaGetErrorString(error) };

        assert_eq!(result, CUDA_ERROR_UNKNOWN_STR.to_string(), "Failed to get error string");
        print!(">>>>>>>>>>>>>>> ");
        println!("Error string: {}", result);
    }
}
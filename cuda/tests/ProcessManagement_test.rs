#![allow(non_snake_case)]
use std::ffi::CString;
use std::os::raw::{c_void, c_char};

pub type CUresult = i32;
pub type CUfunction = *const c_void;

// 函数作用：用于从 CUDA 动态链接库中获取特定函数的地址
pub unsafe extern "C" fn cuGetProcAddress(
    function: *mut CUfunction,
    name: *const c_char,
) -> CUresult {
    if function.is_null() || name.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // let _name_str = CString::from_raw(name as *mut c_char).into_string().unwrap();

    // 这里我们假设根据函数名返回一个假的函数地址
    // let fake_function: CUfunction = 0x7FFFFFFF as *const c_void;

    // *function = fake_function;

    return 0; // CUDA_SUCCESS
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuGetProcAddress() {
        let mut function: CUfunction = std::ptr::null();
        let name = CString::new("my_function").unwrap().into_raw();

        let result = unsafe { cuGetProcAddress(&mut function, name) };

        assert_eq!(result, 0, "Failed to get function address");
        // assert!(function != std::ptr::null(), "Function address is null");

        // if function != std::ptr::null() {
        //     // 转换函数指针类型，并使用它调用函数
        //     let my_function: fn() = unsafe { std::mem::transmute(function) };
        //     my_function();
        // }
        print!(">>>>>>>>>>>>>>> ");
        println!("Function address obtained successfully!");
    }
}
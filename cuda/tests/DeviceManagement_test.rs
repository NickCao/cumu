#![allow(non_snake_case)]
use std::os::raw::{c_int, c_uint, c_char, c_ulonglong};

pub type CUdevice = c_uint;

// 函数作用：返回实际显卡数量
pub unsafe extern "C" fn cuDeviceGetCount(count: *mut c_int) -> c_int {
    if count.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 获取实际的 CUDA 设备数量，我们设置默认值为 2
    let actual_count = 2;

    *count = actual_count;

    return actual_count; // 返回实际的 CUDA 设备数量
}

// 函数作用：获取 CUDA 设备的属性值
// 该函数通过将属性值存储在 pi 指针指向的位置，并返回一个整数值来指示操作的成功与否
pub unsafe extern "C" fn cuDeviceGetAttribute(
    pi: *mut c_int, // pi：一个指向整数的指针，用于存储获取到的属性值
    attrib: c_int,  // attrib：表示要获取的属性的标识符
    _device: c_int  // device：表示要查询的 CUDA 设备的索引
) -> c_int {
    if pi.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    match attrib {  // match的功能类似switch
        0 => *pi = 111,
        1 => *pi = 222,
        _ => return -1, // CUDA_ERROR_INVALID_VALUE
    }

    return 0; // CUDA_SUCCESS
}

// 函数作用：获取与指定索引对应的CUDA设备句柄
pub unsafe extern "C" fn cuDeviceGet(device: *mut CUdevice, _ordinal: c_int) -> c_int {
    if device.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 模拟获取设备句柄的操作
    let default_device_handle: CUdevice = 111;

    *device = default_device_handle;

    return 0; // CUDA_SUCCESS
}

// 函数作用：获取给定设备的名称
pub unsafe extern "C" fn cuDeviceGetName(name: *mut c_char, len: c_int, _device: c_int) -> c_int {
    if name.is_null() || len <= 0 {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 在这里编写获取设备名称的代码
    // 使用示例的设备名称 "MockDevice"，复制到 name 指向的内存中
    let device_name = "MockDevice111";

    let name_ptr = name as *mut i8; //  u8的话会报错类型不匹配
    let name_bytes = device_name.as_bytes();

    // 确保不会超过指定的长度
    let copy_len = std::cmp::min(len - 1, name_bytes.len() as c_int);

    for i in 0..copy_len {
        *name_ptr.offset(i as isize) = name_bytes[i as usize] as c_char;
    }

    // 添加字符串末尾的终止符
    *name_ptr.offset(copy_len as isize) = 0;

    return 0; // CUDA_SUCCESS
}

// 函数作用：获取给定设备的总内存大小
pub unsafe extern "C" fn cuDeviceTotalMem(bytes: *mut c_ulonglong, _device: c_int) -> c_int {
    if bytes.is_null() {
        return -1; // CUDA_ERROR_INVALID_VALUE
    }

    // 示例的设备总内存大小 8 GB
    let total_mem_bytes: c_ulonglong = 8 * 1024 * 1024 * 1024;

    *bytes = total_mem_bytes;

    return 0; // CUDA_SUCCESS
}


// tests 模块包含测试函数，测试了我们上述实现的函数的功能
// 确保它们能够正确地返回结果，并将结果输出到控制台，格式与主函数保持一致
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cuDeviceGetCount() {
        let mut count: c_int = 0;
        let result = unsafe { cuDeviceGetCount(&mut count) };

        assert!(result >= 0, "Failed to get the number of CUDA devices");
        print!(">>>>>>>>>>>>>>> ");
        println!("Number of CUDA devices: {}", count);
    }

    #[test]
    fn test_cuDeviceGetAttribute() {
        let mut attribute: c_int = 0;
        let result = unsafe { cuDeviceGetAttribute(&mut attribute, 0, 0) };

        assert_eq!(result, 0, "Failed to get device attribute");
        print!(">>>>>>>>>>>>>>> ");
        println!("Device attribute: {}", attribute);
    }

    #[test]
    fn test_cuDeviceGet() {
        let mut device: CUdevice = 0;
        let result = unsafe { cuDeviceGet(&mut device, 0) };

        assert_eq!(result, 0, "Failed to get device");
        print!(">>>>>>>>>>>>>>> ");
        println!("Device handle: {:?}", device);
    }

    #[test]
    fn test_cuDeviceGetName() {
        let mut name: [c_char; 256] = [0; 256];
        let result = unsafe { cuDeviceGetName(name.as_mut_ptr(), 256, 0) };

        assert_eq!(result, 0, "Failed to get device name");
        let device_name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()).to_string_lossy() };
        print!(">>>>>>>>>>>>>>> ");
        println!("Device name: {}", device_name);
    }

    #[test]
    fn test_cuDeviceTotalMem() {
        let mut total_mem: c_ulonglong = 0;
        let result = unsafe { cuDeviceTotalMem(&mut total_mem, 0) };

        assert_eq!(result, 0, "Failed to get total device memory");
        print!(">>>>>>>>>>>>>>> ");
        println!("Total device memory: {} bytes", total_mem);
    }
}
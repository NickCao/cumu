#![allow(non_snake_case)]
use std::os::raw::c_uint;

//  函数作用：初始化 CUDA 运行时环境，为后续的 CUDA 操作做准备
pub unsafe extern "C" fn cuInit(_flags: c_uint) -> c_uint {
    // 这里我们假设调用 cuInit 成功并返回默认值 0
    let result = 0;

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cuInit() {
        let flags: c_uint = 0; // 假设使用 flags 为 0 来初始化 CUDA 运行时
        let result = unsafe { cuInit(flags) };

        assert_eq!(result, 0, "Failed to initialize CUDA runtime");
        print!(">>>>>>>>>>>>>>> ");
        println!("CUDA runtime initialized successfully!");
    }
}
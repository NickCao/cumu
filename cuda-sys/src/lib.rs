#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// 这个文件可能是通过绑定生成工具 (bindgen) 从 CUDA 库的 C/C++ 头文件自动生成的 Rust 绑定代码
// 作用可能是用于与 CUDA 库进行交互的 Rust 代码
fn main() {
  println!("cargo:rustc-link-arg=-Wl,-soname=libcuda.so.1");
}

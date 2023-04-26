fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .allowlist_function("cu.*")
        .generate()
        .unwrap()
        .write_to_file(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs"),
        )
        .unwrap();
}

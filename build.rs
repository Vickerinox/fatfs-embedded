use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let target = env::var("TARGET")?;

    let bindings = bindgen::Builder::default()
        .header("fatfs/source/ff.h")
        .clang_arg(format!("--target={}", target))
        .use_core()
        .ctypes_prefix("cty")
        .derive_copy(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut builder = cc::Build::new();
    
    // My host machine keeps f'ing this up, so lets try this instead.
    // This tries building with whatever cc picks
    let build_result = builder
        .file("fatfs/source/ff.c")
        .try_compile("fatfs");

    // If it fails, use the expected arm embedded compiler instead.
    if build_result.is_err() {
        builder
            .file("fatfs/source/ff.c")
            .compiler("arm-none-eabi-gcc")
            .compile("fatfs");
    }

    Ok(())
}
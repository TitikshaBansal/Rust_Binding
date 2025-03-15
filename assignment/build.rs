use bindgen::builder;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile the C code first
    cc::Build::new()
        .file("parse-lib.c")
        .compile("parse-lib");

    // Generate bindings
    println!("cargo:rerun-if-changed=parse-lib.h");
    let bindings = bindgen::Builder::default()
        .header("parse-lib.h")
        .allowlist_item("cups_.*")
        .generate()?;

    // Save bindings
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

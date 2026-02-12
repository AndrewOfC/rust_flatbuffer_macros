use flatbuffers_build::BuilderOptions;
use std::env;
use std::path::Path;

fn flatbuffers_schemas() -> Result<(), String> {
    // Specify the directory containing your .fbs files.
    let schema_dir = Path::new("./tests");

    // Tell Cargo to rerun the build script if any .fbs file in the
    // schemas directory is changed.
    println!("cargo:rerun-if-changed={}", schema_dir.display());

    // Collect all .fbs files from the schemas directory.
    let fbs_files: Vec<_> = std::fs::read_dir(schema_dir)
        .expect("Failed to read schemas directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().map_or(false, |ext| ext == "fbs") {
                Some(path.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    let compiler = if env::consts::OS == "macos" {
        "/opt/flatbuffers/bin/flatc"
    } else {
        "/opt/flatbuffers/bin/flatc"
    };

    // Compile the schemas.
    // NOTE: For multiple schemas, the order can matter if there are dependencies.
    // For simple cases, iterating over the directory contents is often sufficient.
    BuilderOptions::new_with_files(&fbs_files)
        .set_compiler(compiler)
        .set_output_path("./tests/fb")
        .add_flatc_arguments(&["--reflect-types", "--rust-module-root-file"])
        .compile()
        .expect("FlatBuffers compilation failed");

    Ok(())
}

fn main() {
    flatbuffers_schemas().unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

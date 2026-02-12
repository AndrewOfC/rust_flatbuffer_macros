// MIT License
//
// Copyright (c) 2026 Andrew Ellis Page
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// SPDX short identifier: MIT

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

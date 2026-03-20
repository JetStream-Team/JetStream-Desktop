extern crate prost_build;

fn main() {
    println!("cargo:rerun-if-changed=src/jetstream.proto");

    prost_build::compile_protos(
        &["src/jetstream.proto"],
        &["src/"])
        .expect("Failed to compile protobuf definitions");

    tauri_build::build()
}

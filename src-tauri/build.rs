extern crate prost_build;

fn main() {
    prost_build::compile_protos(
        &["src/jetstream.proto"],
        &["src/"])
        .expect("Failed to compile protobuf definitions");

    tauri_build::build()
}

use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("../coco-server/src");
    let proto_file = proto_dir.join("worker.proto");
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let descriptor_path = out_dir.join("worker_descriptor.bin");

    println!("cargo:rerun-if-changed={}", proto_file.display());

    let protoc = protoc_bin_vendored::protoc_bin_path()?;
    let status = Command::new(protoc)
        .arg("--include_imports")
        .arg("--include_source_info")
        .arg("-o")
        .arg(&descriptor_path)
        .arg("-I")
        .arg(&proto_dir)
        .arg(&proto_file)
        .status()?;
    if !status.success() {
        return Err("protoc failed".into());
    }

    tonic_build::configure()
        .file_descriptor_set_path(&descriptor_path)
        .skip_protoc_run()
        .build_client(false)
        .build_server(true)
        .compile(&[proto_file], &[proto_dir])?;

    Ok(())
}

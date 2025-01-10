use std::env; // Import environment module
use std::path::PathBuf; // Import PathBuf for handling paths
use prost_build; // Import prost_build for building protobufs
use which::which; // Import which for finding executables

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?); // Get output directory
    println!("OUT_DIR: {:?}", out_dir); // Print output directory

    match which("protoc") {
        Ok(path) => println!("Found protoc at: {:?}", path), // Print protoc path if found
        Err(e) => println!("Could not find protoc: {:?}", e), // Print error if protoc not found
    }

    prost_build::compile_protos(&["proto/messages.proto"], &["proto/"])?; // Compile protobufs
    println!("cargo:rerun-if-changed=proto/messages.proto"); // Print rerun-if-changed for proto file
    println!("cargo:rerun-if-changed=build.rs"); // Print rerun-if-changed for build script
    Ok(())
}
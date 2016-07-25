
use std::process::Command;

// protoc --rust_out . rpc.proto -I ../../nanopb/generator/proto/ -I .

fn main() {
    println!("Building proto files...");
    let output = Command::new("protoc").args(
        &["--rust_out=src", 
          "deps/ribbon-bridge/proto/rpc.proto", 
          "-Ideps/nanopb/generator/proto",
          "-Ideps/ribbon-bridge/proto"]).output().expect("Failed to build proto files");

	println!("status: {}", output.status);
	println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
	println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

	assert!(output.status.success());
}

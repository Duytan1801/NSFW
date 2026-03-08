fn main() {
    // Re-run this script if the library changes or moves
    println!("cargo:rerun-if-changed=libonnxruntime.so");
    println!("cargo:rerun-if-changed=models/");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir_path = std::path::Path::new(&out_dir);
    println!("OUT_DIR={}, is_dir={}", out_dir, out_dir_path.is_dir());
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}

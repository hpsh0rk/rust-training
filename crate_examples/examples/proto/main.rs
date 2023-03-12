use std::io::Result;
fn main() -> Result<()> {
    let dir = "examples/proto";
    prost_build::Config::new()
        .out_dir(format!("{}/.", dir))
        .compile_protos(&[format!("{}/abi.proto", dir)], &["."])?;
    Ok(())
}

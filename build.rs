fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ok = tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/bitcaskapi.proto"], &["proto"])?;
    
    Ok(ok)
}

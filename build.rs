fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../party_protocals/protocols/helloworld.proto")?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../party_protocals/protocols/helloworld.proto")?;

    tonic_build::compile_protos("../party_protocals/protocols/account_service.proto")?;

    tonic_build::compile_protos("../party_protocals/protocols/room_control_service.proto")?;

    Ok(())
}

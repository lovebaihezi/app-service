fn main() -> std::io::Result<()> {
    tonic_build::compile_protos("proto/todo.proto")?;
    Ok(())
}

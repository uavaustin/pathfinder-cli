fn main() -> Result<(), std::io::Error> {
    prost_build::compile_protos(&["src/proto/pathfinder.proto"], &["src/proto/"])?;

    Ok(())
}

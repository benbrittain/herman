use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["protos/gtfs-realtime.proto"], &["protos/"])?;
    Ok(())
}

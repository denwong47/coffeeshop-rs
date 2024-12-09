use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/models/proto/multicast.proto"], &["src/"])?;
    Ok(())
}

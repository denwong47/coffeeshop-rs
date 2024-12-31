use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/models/message/proto/multicast.proto"], &["src/"])?;
    Ok(())
}

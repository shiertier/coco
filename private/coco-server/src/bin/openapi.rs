use std::io::{self, Write};

fn main() -> io::Result<()> {
    let doc = coco_server::service::openapi_document();
    let json = serde_json::to_string_pretty(&doc).map_err(io::Error::other)?;
    io::stdout().write_all(json.as_bytes())?;
    Ok(())
}

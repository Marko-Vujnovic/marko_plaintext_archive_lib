// use crate as marko_plaintext_archive;
use marko_plaintext_archive::*;

fn main() -> core::result::Result<(), std::io::Error> { tokio::runtime::Runtime::new().unwrap().block_on(async {
    marko_plaintext_archive::entry_point(&[])?;
Ok(())})}

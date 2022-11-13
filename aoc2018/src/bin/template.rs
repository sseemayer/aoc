use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("I/O error: {}", _0)]
    Io(#[from] std::io::Error),

    #[error("Int parsing error: {}", _0)]
    ParseInt(#[from] std::num::ParseIntError),
}

fn main() -> Result<()> {
    Ok(())
}

pub use failure::{format_err, Error};

pub type Result<T> = std::result::Result<T, Error>;

pub mod board;
pub mod util;

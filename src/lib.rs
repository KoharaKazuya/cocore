mod color;
mod converter;
mod parser;
mod representation;

pub use converter::{convert, ConvertionError};
pub use parser::parse;
pub use representation::Representation;

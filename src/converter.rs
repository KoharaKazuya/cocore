use crate::color::{Hex, HSLA};
use crate::parser::parse;
use crate::representation::Representation;
use cssparser::{BasicParseError, Color as CssParserColor, ToCss};
use std::error::Error;
use std::fmt;

pub fn convert(raw: &str, representation: Representation) -> Result<String, ConvertionError> {
    match parse(raw)? {
        CssParserColor::CurrentColor => Err(ConvertionError::new_currentcolor_unconvertable()),
        CssParserColor::RGBA(rgba) => Ok(match representation {
            Representation::Hex => Hex::from(&rgba).to_css_string(),
            Representation::RGB => rgba.to_css_string(),
            Representation::HSL => HSLA::from(&rgba).to_css_string(),
        }),
    }
}

#[derive(Debug, PartialEq)]
pub struct ConvertionError {
    message: String,
}

impl ConvertionError {
    fn new_currentcolor_unconvertable() -> Self {
        ConvertionError {
            message: "cannot convert \"currentcolor\"".to_string(),
        }
    }
}

impl<'a> From<BasicParseError<'a>> for ConvertionError {
    fn from(err: BasicParseError<'a>) -> Self {
        ConvertionError {
            message: format!("parse failed: {:?}", err),
        }
    }
}

impl<'a> fmt::Display for ConvertionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl<'a> Error for ConvertionError {}

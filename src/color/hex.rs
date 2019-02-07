use cssparser::{ToCss, RGBA};
use std::fmt;

pub struct Hex<'a> {
    rgba: &'a RGBA,
}

impl<'a> From<&'a RGBA> for Hex<'a> {
    fn from(rgba: &'a RGBA) -> Self {
        Hex { rgba }
    }
}

impl<'a> ToCss for Hex<'a> {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        let &RGBA {
            red, green, blue, ..
        } = self.rgba;

        // #rrggbb can also be converted to #rgb
        let three_digit = red % 17 == 0 && green % 17 == 0 && blue % 17 == 0;

        if three_digit {
            write!(dest, "#{:X}{:X}{:X}", red / 17, green / 17, blue / 17)
        } else {
            write!(dest, "#{:<02X}{:<02X}{:<02X}", red, green, blue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_css_format() {
        let data = vec![
            (RGBA::new(0, 0, 0, 0), "#000"),
            (RGBA::new(17, 34, 51, 0), "#123"),
            (RGBA::new(255, 255, 255, 0), "#FFF"),
            (RGBA::new(1, 100, 255, 0), "#0164FF"),
        ];
        for (rgba, expect) in data {
            let hex = Hex::from(&rgba);
            assert_eq!(hex.to_css_string(), expect);
        }
    }
}

use cssparser::{ToCss, RGBA};
use std::fmt;

pub struct HSLA<'a> {
    rgba: &'a RGBA,
}

impl<'a> HSLA<'a> {
    fn calc_hsla(&self) -> (u16, u8, u8, u8) {
        let &RGBA {
            red,
            green,
            blue,
            alpha,
        } = self.rgba;

        let min = vec![red, blue, green].into_iter().min().unwrap();
        let max = vec![red, blue, green].into_iter().max().unwrap();
        let min_f32 = f32::from(min);
        let max_f32 = f32::from(max);

        let hue =
            (60. * if min == max {
                0.
            } else if red == max {
                (f32::from(green) - f32::from(blue)) / (max_f32 - min_f32)
            } else if green == max {
                (f32::from(blue) - f32::from(red)) / (max_f32 - min_f32) + 2.
            } else {
                (f32::from(red) - f32::from(green)) / (max_f32 - min_f32) + 4.
            } + 360.) as u16
                % 360;

        let cnt = (u16::from(max) + u16::from(min)) / 2;
        let saturation = (if cnt < 128 {
            (max_f32 - min_f32) / (max_f32 + min_f32)
        } else {
            (max_f32 - min_f32) / (510. - max_f32 - min_f32)
        } * 100.)
            .round() as u8;

        let lightness = ((max_f32 + min_f32) / 2. / 255. * 100.).round() as u8;

        (hue, saturation, lightness, alpha)
    }
}

impl<'a> From<&'a RGBA> for HSLA<'a> {
    fn from(rgba: &'a RGBA) -> Self {
        HSLA { rgba }
    }
}

impl<'a> ToCss for HSLA<'a> {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        let (hue, saturation, lightness, alpha) = self.calc_hsla();

        let serialize_alpha = alpha != 255;

        dest.write_str(if serialize_alpha { "hsla(" } else { "hsl(" })?;
        hue.to_css(dest)?;
        dest.write_str(", ")?;
        saturation.to_css(dest)?;
        dest.write_str("%, ")?;
        lightness.to_css(dest)?;
        dest.write_str("%")?;
        if serialize_alpha {
            dest.write_str(", ")?;

            // Try first with two decimal places, then with three.
            let mut rounded_alpha = (self.rgba.alpha_f32() * 100.).round() / 100.;
            if clamp_unit_f32(rounded_alpha) != alpha {
                rounded_alpha = (self.rgba.alpha_f32() * 1000.).round() / 1000.;
            }

            rounded_alpha.to_css(dest)?;
        }
        dest.write_char(')')?;

        Ok(())
    }
}

fn clamp_unit_f32(val: f32) -> u8 {
    // Whilst scaling by 256 and flooring would provide
    // an equal distribution of integers to percentage inputs,
    // this is not what Gecko does so we instead multiply by 255
    // and round (adding 0.5 and flooring is equivalent to rounding)
    //
    // Chrome does something similar for the alpha value, but not
    // the rgb values.
    //
    // See https://bugzilla.mozilla.org/show_bug.cgi?id=1340484
    //
    // Clamping to 256 and rounding after would let 1.0 map to 256, and
    // `256.0_f32 as u8` is undefined behavior:
    //
    // https://github.com/rust-lang/rust/issues/10184
    clamp_floor_256_f32(val * 255.)
}

fn clamp_floor_256_f32(val: f32) -> u8 {
    val.round().max(0.).min(255.) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::RGBA;

    #[test]
    fn to_css_format() {
        let data = vec![
            (RGBA::new(0, 0, 0, 255), "hsl(0, 0%, 0%)"),
            (RGBA::new(255, 153, 221, 255), "hsl(320, 100%, 80%)"),
            (RGBA::new(127, 127, 127, 128), "hsla(0, 0%, 50%, 0.5)"),
            (RGBA::new(0, 0, 0, 0), "hsla(0, 0%, 0%, 0)"),
        ];
        for (rgba, expect) in data {
            let hsla = HSLA::from(&rgba);
            assert_eq!(hsla.to_css_string(), expect);
        }
    }
}

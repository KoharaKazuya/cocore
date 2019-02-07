use cssparser::{BasicParseError, Color, Parser, ParserInput};

pub fn parse(value: &'_ str) -> Result<Color, BasicParseError<'_>> {
    let mut input = ParserInput::new(value);
    let mut parser = Parser::new(&mut input);
    Color::parse(&mut parser)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cssparser::RGBA;

    #[test]
    fn test_parse_ok() {
        let table = vec![
            ("rgb(0, 0, 0)", Color::RGBA(RGBA::new(0, 0, 0, 255))),
            (
                "rgb(255, 255, 255)",
                Color::RGBA(RGBA::new(255, 255, 255, 255)),
            ),
        ];
        for (expression, expect) in table {
            let actual = parse(expression);
            assert!(
                actual == Ok(expect),
                "assertion failed: parse(\"{}\") connot parse to `{:?}` (actual: `{:?}`)",
                expression,
                expect,
                actual
            )
        }
    }

    #[test]
    fn test_parse_error() {
        let table = vec!["", "rgb(0,0,0,0,0)", "rgb("];
        for expression in table {
            let actual = parse(expression);
            assert!(
                actual.is_err(),
                "assertion failed: parse(\"{}\") can parse to `{:?}`",
                expression,
                actual.unwrap()
            );
        }
    }
}

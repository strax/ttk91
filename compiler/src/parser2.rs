use combine::*;
use combine::char::{char, letter};
use combine::parser::char::string;
use combine::parser::char::string_cmp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Tok {
    EQU()
}

fn string_i<'a, I>(s: &'static str) -> impl Parser<Input = I, Output = &'static str>
    where I: RangeStream<Item = char, Range = &'a str>,
          I::Error: ParseError<I::Item, I::Range, I::Position> {
    string_cmp(s, |a, b| a.eq_ignore_ascii_case(&b))
}

pub fn kw_equ<'a, I>() -> impl Parser<Input = I, Output = Tok>
    where I: RangeStream<Item = char, Range = &'a str>,
          I::Error: ParseError<I::Item, I::Range, I::Position> {
    string_i("EQU").map(|_| Tok::EQU())
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::stream::easy::Errors;
    use combine::stream::PointerOffset;
    use combine::stream::easy::Error;

    fn format_error(err: Error<char, &str>) -> String {
        match err {
            Error::Expected(info) => info.
        }
    }

    fn format_errors(err: Errors<char, &str, PointerOffset>, input: &str) -> Vec<String> {
        let Errors { errors, position } = err.map_position(|p| p.translate_position(input));
        errors.iter().map(format_error).collect()
    }

    #[test]
    fn test_kw_equ() {
        let input = "EQJ";
        match kw_equ().easy_parse(input) {
            Ok(res) => assert_eq!(res, (Tok::EQU(), "")),
            Err(err) => {
                eprintln!("{}", format_errors(err, input));
                panic!()
            }
        }
    }
}
use std::{iter::Peekable, usize};

use super::LexerToken;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let peek = iterator.peek()?;

    if !peek.is_ascii_digit() {
        return None;
    }

    if *peek != '0' {
        let num = parse_number_radix(iterator, 10);
        return Some(LexerToken::Number(num));
    }

    // first char is 0, consume
    iterator.next().unwrap();

    let Some(peek) = iterator.peek() else {
        return Some(LexerToken::Number(0)) /* no more chars after that and we just read "0" */
    };

    let radix = match peek {
        'b' => 2,
        'o' => 8,
        'x' => 16,
        '0'..='9' => 10,
        ' ' | ',' | ';' => return Some(LexerToken::Number(0)),
        _ => return None,
    };

    let num = parse_number_radix(iterator, radix);
    Some(LexerToken::Number(num))
}

fn parse_number_radix<T>(iterator: &mut Peekable<T>, radix: u8) -> isize
where
    T: Iterator<Item = char>,
{
    let mut value: isize = 0;
    for char in iterator.by_ref() {
        if char == '_' {
            continue;
        }

        if is_valid_char_for_radix(char, radix) {
            value = value * isize::from(radix) + isize::from(get_char_value(char));
        } else {
            break;
        }
    }
    value
}

fn is_valid_char_for_radix(char: char, radix: u8) -> bool {
    let radix_usize = usize::from(radix);
    let valid_chars = "0123456789abcdef";
    let radix_chars = &valid_chars[..radix_usize];

    radix_chars.chars().any(|c| c == char)
}

fn get_char_value(char: char) -> u8 {
    match char {
        '0'..='9' => char as u8 - b'0',
        'a'..='f' => char as u8 - b'a' + 10,
        _ => panic!("no"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_lex {
        (input = $input:literal, expected = $expected:literal) => {
            let mut iterator = $input.chars().peekable();
            let LexerToken::Number(lexed) = lex(&mut iterator).unwrap() else {
                core::panic!("Lexer output is not a number!");
            };
            assert_eq!(lexed, $expected, "Lexed token value does not match expected value!");
        };
    }

    macro_rules! assert_parse {
        (input = $input:literal, radix = $radix:literal, expected = $expected:literal) => {
            let mut iterator = $input.chars().peekable();
            let number = parse_number_radix(&mut iterator, $radix);
            assert_eq!(number, $expected, "Parsed number does not equal exected number!");
        };
    }

    #[test]
    fn lex_no_input() {
        let input = "";

        let mut iterator = input.chars().peekable();
        let lexed = lex(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        );
    }

    #[test]
    fn lex_invalid_input() {
        let input = "hello, world!";

        let mut iterator = input.chars().peekable();
        let lexed = lex(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        );
    }

    #[test]
    fn lex_invalid_radix_string() {
        let input = "0z123456";

        let mut iterator = input.chars().peekable();
        let lexed = lex(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        );
    }

    #[test]
    fn lex_zero() { assert_lex!(input = "0;", expected = 0); }

    #[test]
    fn lex_decimal_with_several_zeros_at_beginning() {
        assert_lex!(input = "00123456", expected = 123_456);
    }

    #[test]
    fn lex_with_separator() {
        assert_lex!(input = "123_456", expected = 123_456);
    }

    #[test]
    fn lex_decimal() {
        assert_lex!(input = "123456789", expected = 123_456_789);
    }

    #[test]
    fn parse_number_binary() {
        assert_parse!(input = "00100100", radix = 2, expected = 0b00100100);
    }

    #[test]
    fn parse_number_octal() {
        assert_parse!(input = "777", radix = 8, expected = 0o777);
    }

    #[test]
    fn parse_number_hex() {
        assert_parse!(input = "2468acef", radix = 16, expected = 0x2468_acef);
    }
}

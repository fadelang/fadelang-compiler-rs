use std::iter::Peekable;

use super::LexerToken;

pub(crate) fn lex_number<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
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
        } else if is_valid_char_for_radix(char, radix) {
            value = value * radix as isize + get_char_value(char) as isize;
        } else {
            break;
        }
    }
    value
}

fn is_valid_char_for_radix(char: char, radix: u8) -> bool {
    let radix_usize = radix as usize;
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

    #[test]
    fn lex_number_no_input() {
        let input = "";

        let mut iterator = input.chars().peekable();
        let lexed = lex_number(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        )
    }

    #[test]
    fn lex_number_invalid_input() {
        let input = "hello, world!";

        let mut iterator = input.chars().peekable();
        let lexed = lex_number(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        )
    }

    #[test]
    fn lex_number_invalid_radix_string() {
        let input = "0z123456";

        let mut iterator = input.chars().peekable();
        let lexed = lex_number(&mut iterator);
        assert!(
            lexed.is_none(),
            "Lexer output is {lexed:?} on invalid input!"
        )
    }

    #[test]
    fn lex_number_decimal_with_several_zeros_at_beginning() {
        let input = "00123456";
        let expected = 123456;

        let mut iterator = input.chars().peekable();
        let LexerToken::Number(lexed) = lex_number(&mut iterator).unwrap();
        assert_eq!(
            lexed, expected,
            "Lexed token does not match expected token!"
        );
    }

    #[test]
    fn lex_number_with_separator() {
        let input = "123_456";
        let expected = 123_456;

        let mut iterator = input.chars().peekable();
        let LexerToken::Number(lexed) = lex_number(&mut iterator).unwrap();
        assert_eq!(
            lexed, expected,
            "Lexed token does not match expected token!"
        );
    }

    #[test]
    fn lex_number_decimal() {
        let input = "123456789";
        let expected = 123_456_789;

        let mut iterator = input.chars().peekable();
        let LexerToken::Number(lexed) = lex_number(&mut iterator).unwrap();
        assert_eq!(
            lexed, expected,
            "Lexed token does not match expected token!"
        );
    }

    #[test]
    fn parse_number_binary() {
        let number = "00100100";
        let expected = 0b0010_0100;

        let mut iterator = number.chars().peekable();
        let number = parse_number_radix(&mut iterator, 2);
        assert_eq!(
            number, expected,
            "Parsed number does not equal expected number!"
        );
    }

    #[test]
    fn parse_number_octal() {
        let number = "777";
        let expected = 0o777;

        let mut iterator = number.chars().peekable();
        let number = parse_number_radix(&mut iterator, 8);
        assert_eq!(
            number, expected,
            "Parsed number does not equal exected number!"
        );
    }

    #[test]
    fn parse_number_hex() {
        let number = "2468acef";
        let expected = 0x2468_acef;

        let mut iterator = number.chars().peekable();
        let number = parse_number_radix(&mut iterator, 16);
        assert_eq!(
            number, expected,
            "Parsed number does not equal exected number!"
        );
    }
}

use std::iter::Peekable;

pub(crate) enum LexerToken {
    //    Identifier(String),
    //    Keyword(String),
    Number(isize),
}

pub(crate) fn lex(buffer: &str) -> Vec<LexerToken> {
    let result = vec![];
    let mut iterator = buffer.chars().peekable();

    while let Some(&char) = iterator.peek() {
        if char.is_ascii_digit() {}
    }

    result
}

fn lex_number<T>(iterator: &mut Peekable<T>) -> LexerToken
where
    T: Iterator<Item = char>,
{
    let first = iterator.next().unwrap();
    let second = iterator.peek().unwrap();

    // todo: negatives

    if first == '0' {
        let radix: u8 = match second {
            'b' => 2,
            'o' => 8,
            'x' => 16,
            _ => 0,
        };

        if radix != 0 {
            iterator.next().unwrap();
            let number = parse_number_radix(iterator, radix);
            return LexerToken::Number(number);
        }
    }

    LexerToken::Number(0)
}

fn parse_number_radix<T>(iterator: &mut Peekable<T>, radix: u8) -> isize
where
    T: Iterator<Item = char>,
{
    let mut value: isize = 0;
    for char in iterator.by_ref() {
        // todo: cleanup
        if is_valid_char_for_radix(char, radix) {
            value = value * radix as isize + get_char_value(char) as isize;
        } else if char == ' ' || char == '\t' || char == '\n' {
            break;
        } else {
            panic!("unexpected char {char} while parsing binary number value");
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

    #[test]
    fn arse_number_binary() {
        let number = "00100100";
        let expected = 0b0010_0100;
        let mut iterator = number.chars().peekable();

        let number = super::parse_number_radix(&mut iterator, 2);
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
        let number = super::parse_number_radix(&mut iterator, 8);
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
        let number = super::parse_number_radix(&mut iterator, 16);
        assert_eq!(
            number, expected,
            "Parsed number does not equal exected number!"
        );
    }
}

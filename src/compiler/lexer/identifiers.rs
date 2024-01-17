use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let mut buffer = String::new();

    while let Some(peek) = iterator.peek() {
        let is_first_char = buffer.is_empty();

        // prevent the first char from being a number
        // todo: allow '_' as first char
        if is_first_char {
            if !peek.is_alphabetic() {
                break;
            }
        } else if !(peek.is_alphanumeric() || *peek == '_') {
            break;
        }

        let next = iterator.next()?;

        buffer.push(next);
    }

    if buffer.is_empty() {
        return None;
    }

    Some(LexerToken::Identifier(buffer))
}

use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let mut whitespace = false;

    while let Some(peek) = iterator.peek() {
        if !peek.is_ascii_whitespace() {
            break;
        }

        iterator.next()?;
        whitespace = true;
    }

    if !whitespace {
        return None;
    }

    Some(LexerToken::Whitespace)
}

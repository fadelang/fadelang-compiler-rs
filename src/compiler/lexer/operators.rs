use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    if let Some(peek) = iterator.peek() {
        if matches!(peek, ':' | '-' | ';') {
            return Some(LexerToken::Operator(String::from(iterator.next()?)));
        }

//         break;
    }

    None
}

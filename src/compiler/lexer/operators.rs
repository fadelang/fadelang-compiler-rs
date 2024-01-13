use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    None
}

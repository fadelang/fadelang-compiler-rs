use super::LexerToken;
use std::iter::Peekable;

#[derive(Debug)]
pub(crate) enum BracketKind {

    /// {
    OpenBrace,

    /// }
    ClosedBrace,

    /// [
    OpenBracket,

    /// ]
    ClosedBracket,

    /// (
    OpenParenthesis,

    /// )
    ClosedParenthesis,

}

impl BracketKind {
    fn matching_brace(&self) -> Self {
        match *self {
            Self::OpenBrace => Self::ClosedBrace,
            Self::ClosedBrace => Self::OpenBrace,
            Self::OpenBracket => Self::ClosedBracket,
            Self::ClosedBracket => Self::OpenBracket,
            Self::OpenParenthesis => Self::ClosedParenthesis,
            Self::ClosedParenthesis => Self::OpenParenthesis,
        }
    }
}

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    if let Some(char) = iterator.peek() {
        if !is_paren(*char) {
            return None;
        }

        let Some(char) = iterator.next() else {
            return None;
        };

        let paren = match char {
            '(' => Some(BracketKind::OpenParenthesis),
            ')' => Some(BracketKind::ClosedParenthesis),
            '[' => Some(BracketKind::OpenBracket),
            ']' => Some(BracketKind::ClosedBracket),
            '{' => Some(BracketKind::OpenBrace),
            '}' => Some(BracketKind::ClosedBrace),
            _ => None,
        };

        let paren = paren?;
        return Some(LexerToken::Parentheses(paren));
    }

    None
}

fn is_paren(char: char) -> bool {
    matches!(char, '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>')
}

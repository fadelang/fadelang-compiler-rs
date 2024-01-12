use super::LexerToken;
use std::iter::Peekable;

#[derive(Debug)]
pub(crate) enum Parentheses {
    Open(ParenthesisType),
    Closed(ParenthesisType),
}

#[derive(Debug)]
pub(crate) enum ParenthesisType {
    Braces, // { }
    Square, // [ ]
    Round,  // ( )
    Angle,  // < >
}

pub(crate) fn lex_parentheses<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
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
            '(' => Some(Parentheses::Open(ParenthesisType::Round)),
            ')' => Some(Parentheses::Closed(ParenthesisType::Round)),
            '[' => Some(Parentheses::Open(ParenthesisType::Square)),
            ']' => Some(Parentheses::Closed(ParenthesisType::Square)),
            '{' => Some(Parentheses::Open(ParenthesisType::Braces)),
            '}' => Some(Parentheses::Closed(ParenthesisType::Braces)),
            '<' => Some(Parentheses::Open(ParenthesisType::Angle)),
            '>' => Some(Parentheses::Closed(ParenthesisType::Angle)),
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

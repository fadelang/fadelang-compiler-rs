use crate::compiler::operators::Operator;

use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    enum CommentKind {
        Line,
        Block,
    }

    let peek = *iterator.peek()?;
    if peek != '/' {
        return None;
    }
    iterator.next()?;

    let peek = *iterator.peek()?;
    let kind = match peek {
        '/' => CommentKind::Line,
        '*' => CommentKind::Block,
        _ => return Some(LexerToken::Operator(Operator::Division)),
    };

    match kind {
        CommentKind::Line => loop {
            let next = iterator.next();
            if next.is_none() || next.unwrap() == '\n' {
                break;
            }
        },
        CommentKind::Block => loop {
            let next = iterator.next();
            assert!(next.is_some(), "Block comment ended before '*/'");
            let next = next?;

            if next != '*' {
                continue;
            }

            let peek = iterator.peek();
            assert!(peek.is_some(), "Block comment ended before '*/'");

            let peek = *peek?;
            if peek == '/' {
                iterator.next()?;
                break;
            }
        },
    }

    Some(LexerToken::Comment(String::new()))
}

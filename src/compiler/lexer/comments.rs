use super::LexerToken;
use std::iter::Peekable;

pub(crate) fn lex<T>(iterator: &mut Peekable<T>) -> Option<LexerToken>
where
    T: Iterator<Item = char>,
{
    let peek = *iterator.peek()?;
    if peek != '/' {
        return None;
    }
    iterator.next()?;

    enum CommentKind {
        Line,
        Block,
    }

    let peek = *iterator.peek()?;
    let kind = match peek {
        '/' => CommentKind::Line,
        '*' => CommentKind::Block,
        _ => return None,
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
            if next.is_none() {
                panic!("Block comment ended before '*/'");
            }
            let next = next?;

            if next != '*' {
                continue;
            }

            let peek = iterator.peek();
            if peek.is_none() {
                panic!("Block comment ended before '*/'");
            }

            let peek = *peek?;
            if peek == '/' {
                iterator.next()?;
                break;
            }
        },
    }
    
    Some(LexerToken::Comment(String::new()))
}

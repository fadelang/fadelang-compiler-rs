#[derive(Debug)]
pub(crate) enum Keyword {
    Use,
    Return,
}

pub(crate) fn get_keyword(string: &str) -> Option<Keyword> {
    match string {
        "use" => Some(Keyword::Use),
        "return" => Some(Keyword::Return),
        _ => None,
    }
}

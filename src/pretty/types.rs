#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NoLineBreaksString<'a>(&'a str);

impl<'a> NoLineBreaksString<'a> {
    pub fn make(s: &'a str) -> Result<NoLineBreaksString, String> {
        if s.find("\n").is_some() {
            Err(String::from("Malformed string"))
        } else {
            Ok(NoLineBreaksString(s))
        }
    }

    pub fn unwrap(s: NoLineBreaksString) -> &str {
        s.0
    }
}

#[cfg(test)]
mod no_line_breaks_string_test {
    use crate::pretty::types::NoLineBreaksString;

    #[test]
    fn make_fails() {
        assert_eq!(NoLineBreaksString::make(&"aome\nadsf").is_ok(), false);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Document<'a> {
    Empty,
    Concat(Box<Document<'a>>, Box<Document<'a>>),
    Text(NoLineBreaksString<'a>),
    Nest(u16, Box<Document<'a>>),
    Break(NoLineBreaksString<'a>),
    Group(Box<Document<'a>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleDocument<'a> {
    Empty,
    Text(NoLineBreaksString<'a>, Box<SimpleDocument<'a>>),
    Line(u16, Box<SimpleDocument<'a>>),
}

pub trait Pretty {
    fn to_document(&self) -> Document;
}

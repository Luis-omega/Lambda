#[derive(Debug, PartialEq, Eq)]
pub struct NoLineBreaksString(String);

impl NoLineBreaksString {
    pub fn make(s: String) -> Result<NoLineBreaksString, String> {
        if s.find("\n").is_some() {
            Err(String::from("Malformed string"))
        } else {
            Ok(NoLineBreaksString(s))
        }
    }

    pub fn unwrap(s: NoLineBreaksString) -> String {
        s.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Document {
    Empty,
    Concat(Box<Document>, Box<Document>),
    Text(NoLineBreaksString),
    Nest(u16, Box<Document>),
    Break(NoLineBreaksString),
    Group(Box<Document>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleDocument {
    Empty,
    Text(NoLineBreaksString, Box<SimpleDocument>),
    Line(u16, Box<SimpleDocument>),
}

pub trait Pretty {
    fn to_document(&self) -> Document;
}

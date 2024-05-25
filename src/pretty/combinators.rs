use crate::pretty::types::Document;
use crate::pretty::types::Document::*;
use crate::pretty::types::NoLineBreaksString;

pub fn empty<'a>() -> Document<'a> {
    Empty
}

pub fn concat<'a>(d1: Document<'a>, d2: Document<'a>) -> Document<'a> {
    Concat(Box::new(d1), Box::new(d2))
}

pub fn concat_vec<'a>(docs: Vec<Document<'a>>) -> Document<'a> {
    let mut doc = empty();
    for new_doc in docs.into_iter().rev() {
        doc = concat(new_doc, doc)
    }
    doc
}

pub fn text<'a>(s: NoLineBreaksString<'a>) -> Document<'a> {
    Text(s)
}

pub fn nest<'a>(ident: u16, d: Document<'a>) -> Document<'a> {
    Nest(ident, Box::new(d))
}

//Rust has break keyword
pub fn break_<'a>(s: NoLineBreaksString<'a>) -> Document<'a> {
    Break(s)
}

pub fn group<'a>(d: Document<'a>) -> Document<'a> {
    Group(Box::new(d))
}

pub fn from_str<'a>(s: &'a str) -> Document<'a> {
    let v = NoLineBreaksString::decompose(s);
    if v.len() == 0 {
        empty()
    } else {
        if v.len() == 1 {
            text(v[0])
        } else {
            let mut doc = empty();
            for new_string in v.into_iter().rev() {
                doc = concat(break_(new_string), doc);
            }
            doc
        }
    }
}

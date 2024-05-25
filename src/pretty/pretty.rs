use crate::pretty::configuration::PrettifierConfiguration;
use crate::pretty::types::{
    Document, NoLineBreaksString, Pretty, SimpleDocument,
};

// Adapted from the paper Strictly Pretty, Christian Lindig.
// Originally attempt to build a vector of String or $string
// and then join them, but I got problems with the borrow checker.

fn render_simple_document<'a>(document: SimpleDocument<'a>) -> String {
    match document {
        SimpleDocument::Empty => String::from(""),
        SimpleDocument::Text(text, remain) => {
            let mut remain = render_simple_document(*remain);
            let out = String::from(NoLineBreaksString::unwrap(text)) + &*remain;
            out
        }
        SimpleDocument::Line(nl, remain) => {
            let mut result = render_simple_document(*remain);
            let mut prefix = " ".repeat(usize::from(nl));
            let out = String::from("\n") + &*prefix + &*result;
            out
        }
    }
}

#[cfg(test)]
mod simple_document_tests {
    use crate::pretty::pretty::render_simple_document;
    use crate::pretty::types::{NoLineBreaksString, SimpleDocument};
    fn assert_render(document: Option<SimpleDocument>, expected: &str) {
        let rendered = document.map(render_simple_document);
        assert_eq!(rendered, Some(String::from(expected)))
    }

    #[test]
    fn empty() {
        let document = Some(SimpleDocument::Empty);
        let expected = "";
        assert_render(document, expected)
    }

    #[test]
    fn single_text() {
        let raw1 = "hello world";
        let raw2 = ", bye world";
        let non_raw1 = NoLineBreaksString::make(&raw1).ok();
        let document = NoLineBreaksString::make(&raw2)
            .ok()
            .map(|x| {
                non_raw1.map(|y| {
                    SimpleDocument::Text(
                        y,
                        Box::new(SimpleDocument::Text(
                            x,
                            Box::new(SimpleDocument::Empty),
                        )),
                    )
                })
            })
            .flatten();
        let expected = "hello world, bye world";
        assert_render(document, expected)
    }

    #[test]
    fn single_line() {
        let raw = "hello world2";
        let document = NoLineBreaksString::make(&raw).ok().map(|x| {
            SimpleDocument::Line(
                5,
                Box::new(SimpleDocument::Text(
                    x,
                    Box::new(SimpleDocument::Empty),
                )),
            )
        });
        let expected = "\n     hello world2";
        assert_render(document, expected)
    }
}

#[derive(Debug, Clone)]
enum Mode {
    Flat,
    Break,
}

#[derive(Debug, Clone)]
struct FitsParam<'a> {
    ident: u16,
    mode: Mode,
    doc: Document<'a>,
}

fn fits(remain_width: Option<usize>, param: &mut Vec<FitsParam>) -> bool {
    if remain_width.is_none() {
        false
    } else {
        match param.pop() {
            None => true,
            Some(FitsParam { ident, mode, doc }) => match doc {
                Document::Empty => fits(remain_width, param),
                Document::Concat(left, right) => {
                    param.push(FitsParam {
                        ident,
                        mode: mode.clone(),
                        doc: *right,
                    });
                    param.push(FitsParam {
                        ident,
                        mode,
                        doc: *left,
                    });
                    fits(remain_width, param)
                }
                Document::Nest(ident2, remain) => {
                    param.push(FitsParam {
                        ident: ident + ident2,
                        mode,
                        doc: *remain,
                    });
                    fits(remain_width, param)
                }
                Document::Text(s) => fits(
                    remain_width.and_then(|x| {
                        x.checked_sub(
                            NoLineBreaksString::unwrap(s).chars().count(),
                        )
                    }),
                    param,
                ),
                Document::Break(s) => match mode {
                    Mode::Flat => fits(
                        remain_width.and_then(|x| {
                            x.checked_sub(
                                NoLineBreaksString::unwrap(s).chars().count(),
                            )
                        }),
                        param,
                    ),
                    Mode::Break => true,
                },
                Document::Group(remain) => {
                    param.push(FitsParam {
                        ident,
                        mode: Mode::Flat,
                        doc: *remain,
                    });
                    fits(remain_width, param)
                }
            },
        }
    }
}

fn document_to_simple_document_aux<'a>(
    width: usize,
    consumed: usize,
    param: &mut Vec<FitsParam<'a>>,
) -> SimpleDocument<'a> {
    println!("new_iter: {:?}", param);
    println!("params: width={}, consumed={}", width, consumed);
    let next = param.pop();
    println!("taken: {:?}", next);
    match next {
        None => SimpleDocument::Empty,
        Some(FitsParam { ident, mode, doc }) => match doc {
            Document::Empty => {
                document_to_simple_document_aux(width, consumed, param)
            }
            Document::Concat(left, right) => {
                param.push(FitsParam {
                    ident,
                    mode: mode.clone(),
                    doc: *right,
                });
                param.push(FitsParam {
                    ident,
                    mode,
                    doc: *left,
                });
                document_to_simple_document_aux(width, consumed, param)
            }
            Document::Nest(ident2, remain) => {
                param.push(FitsParam {
                    ident: ident + ident2,
                    mode,
                    doc: *remain,
                });
                document_to_simple_document_aux(width, consumed, param)
            }
            Document::Text(s) => {
                println!("text_case: {:?}", s);
                let remain = document_to_simple_document_aux(
                    width,
                    consumed + NoLineBreaksString::unwrap(s).chars().count(),
                    param,
                );
                println!("text_case_remain: {:?}", remain);
                SimpleDocument::Text(s, Box::from(remain))
            }
            Document::Break(s) => match mode {
                Mode::Flat => {
                    let remain = document_to_simple_document_aux(
                        width,
                        consumed
                            + NoLineBreaksString::unwrap(s).chars().count(),
                        param,
                    );
                    SimpleDocument::Text(s, Box::from(remain))
                }
                Mode::Break => {
                    let remain = document_to_simple_document_aux(
                        width,
                        usize::from(ident),
                        param,
                    );
                    SimpleDocument::Line(
                        ident,
                        //The paper is wrong here, they forgot to
                        //put the text inside the break
                        Box::from(SimpleDocument::Text(s, Box::from(remain))),
                    )
                }
            },
            Document::Group(remain) => {
                let mut copy = param.clone();
                let remain_copy = remain.clone();
                copy.push(FitsParam {
                    ident,
                    mode: Mode::Flat,
                    doc: *remain,
                });
                if fits(width.checked_sub(consumed), &mut copy) {
                    param.push(FitsParam {
                        ident,
                        mode: Mode::Flat,
                        doc: *remain_copy,
                    });
                    document_to_simple_document_aux(width, consumed, param)
                } else {
                    param.push(FitsParam {
                        ident,
                        mode: Mode::Break,
                        doc: *remain_copy,
                    });
                    document_to_simple_document_aux(width, consumed, param)
                }
            }
        },
    }
}

fn document_to_simple_document(
    configuration: PrettifierConfiguration,
    document: Document,
) -> SimpleDocument {
    match configuration {
        PrettifierConfiguration { line_width } => {
            let mut params = vec![FitsParam {
                ident: 0,
                mode: Mode::Flat,
                doc: document,
            }];
            document_to_simple_document_aux(
                usize::from(line_width),
                0,
                &mut params,
            )
        }
    }
}

pub fn render(
    configuration: PrettifierConfiguration,
    document: Document,
) -> String {
    let simple = document_to_simple_document(configuration, document);
    println!("SIMPLE: {:?}", simple);
    render_simple_document(simple)
}

#[cfg(test)]
mod render_tests {
    use crate::pretty::combinators;
    use crate::pretty::configuration::PrettifierConfiguration;
    use crate::pretty::pretty::render;
    use crate::pretty::types::{Document, NoLineBreaksString, SimpleDocument};

    fn mk_conf(line_width: u16) -> PrettifierConfiguration {
        PrettifierConfiguration { line_width }
    }

    #[test]
    fn empty() {
        let conf = mk_conf(10);
        assert_eq!("", render(conf, Document::Empty))
    }

    #[test]
    fn concat() {
        let raw1 = "hello world";
        let raw2 = " bye world";
        let document1 = NoLineBreaksString::make(&raw1)
            .ok()
            .map(|x| Document::Text(x));
        let document2 = NoLineBreaksString::make(&raw2)
            .ok()
            .map(|x| Document::Text(x));
        let document = document1
            .map(|x| {
                document2.map(|y| Document::Concat(Box::from(x), Box::from(y)))
            })
            .flatten();
        let conf = mk_conf(10);
        assert_eq!(
            Some(String::from(raw1) + raw2),
            document.map(|x| render(conf, x))
        )
    }

    #[test]
    fn text() {
        let raw = "hello world";
        let document = NoLineBreaksString::make(&raw)
            .ok()
            .map(|x| Document::Text(x));
        let conf = mk_conf(10);
        assert_eq!(Some(String::from(raw)), document.map(|x| render(conf, x)))
    }

    #[test]
    fn nest_break_group() {
        let document = NoLineBreaksString::make("world").map(|x| {
            combinators::group(combinators::concat(
                combinators::from_str("hello"),
                combinators::nest(3, combinators::break_(x)),
            ))
        });
        let conf = mk_conf(2);
        assert_eq!(
            Ok(String::from("hello\n   world")),
            document.map(|x| render(conf, x))
        )
    }

    #[test]
    fn nest_break_group_flat() {
        let document = NoLineBreaksString::make("world").map(|x| {
            combinators::group(combinators::concat(
                combinators::from_str("hello"),
                combinators::nest(3, combinators::break_(x)),
            ))
        });
        let conf = mk_conf(20);
        assert_eq!(
            Ok(String::from("helloworld")),
            document.map(|x| render(conf, x))
        )
    }
}

pub fn prettify(
    configuration: PrettifierConfiguration,
    body: &impl Pretty,
) -> String {
    let document = Pretty::to_document(body);
    render(configuration, document)
}

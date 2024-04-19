use crate::pretty::configuration::PrettifierConfiguration;
use crate::pretty::types::{
    Document, NoLineBreaksString, Pretty, SimpleDocument,
};

fn render_simple_document_aux(document: SimpleDocument) -> Vec<String> {
    match document {
        SimpleDocument::Empty => vec![String::from("")],
        SimpleDocument::Text(text, remain) => {
            let mut remain_vec = render_simple_document_aux(*remain);
            remain_vec.push(NoLineBreaksString::unwrap(text));
            remain_vec
        }
        SimpleDocument::Line(nl, remain) => {
            let mut remain_vec = render_simple_document_aux(*remain);
            remain_vec.push(String::from(" ".repeat(usize::from(nl))));
            remain_vec.push(String::from("\n"));
            remain_vec
        }
    }
}

fn render_simple_document(document: SimpleDocument) -> String {
    let mut vector = render_simple_document_aux(document);
    vector.reverse();
    vector.join("")
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
        let raw = String::from("hellow world");
        let document = NoLineBreaksString::make(raw)
            .ok()
            .map(|x| SimpleDocument::Text(x, Box::new(SimpleDocument::Empty)));
        let expected = "hellow world";
        assert_render(document, expected)
    }

    #[test]
    fn single_line() {
        let raw = String::from("hellow world2");
        let document = NoLineBreaksString::make(raw).ok().map(|x| {
            SimpleDocument::Line(
                5,
                Box::new(SimpleDocument::Text(
                    x,
                    Box::new(SimpleDocument::Empty),
                )),
            )
        });
        let expected = "\n     hellow world2";
        assert_render(document, expected)
    }
}

fn document_to_simple_document(
    configuration: PrettifierConfiguration,
    document: Document,
) -> SimpleDocument {
    SimpleDocument::Empty
}

pub fn render(
    configuration: PrettifierConfiguration,
    document: Document,
) -> String {
    let simple = document_to_simple_document(configuration, document);
    render_simple_document(simple)
}

pub fn prettify(
    configuration: PrettifierConfiguration,
    body: &impl Pretty,
) -> String {
    let document = Pretty::to_document(body);
    render(configuration, document)
}

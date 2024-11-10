use crate::dom::{Document, Element, NodeMut, Tree};
use crate::parsing::Parse;

pub struct Parser {
    cursor: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            cursor: 0,
            input,
        }
    }

    pub fn create_parsed_tree(&mut self) -> Tree {
        let mut tree = Tree::new(Element::Document(Document::new()));
        let mut root = tree.root_mut();
        self.parse_nodes(&mut root);

        tree
    }

    fn parse_nodes(&mut self, parent: &mut NodeMut) {
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            self.parse_node(parent);
        }
    }

    fn parse_node<'a, 'b>(&mut self, parent: &'b mut NodeMut<'a>) -> NodeMut<'b> {
        if self.starts_with("<") {
            self.parse_element(parent)
        } else {
            self.parse_text(parent)
        }
    }

    fn parse_text<'a, 'b>(&mut self, parent: &'b mut NodeMut<'a>) -> NodeMut<'b> {
        parent.append(
            Element::from_text(self.consume_while(|c| c != '<'))
        )
    }

    fn parse_element<'a, 'b>(&mut self, parent: &'b mut NodeMut<'a>) -> NodeMut<'b> {
        self.expect("<");
        let tag_name = self.parse_name();
        self.expect(">");

        let mut parent = parent.append(Element::from_tag_name(&*tag_name));
        self.parse_nodes(&mut parent);

        self.expect("</");
        self.expect(&*tag_name);
        self.expect(">");

        parent

    }
}

impl Parse for Parser {
    fn input(&self) -> &str {
        &self.input
    }

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }
}
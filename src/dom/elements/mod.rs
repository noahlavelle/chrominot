mod htmlelement;
mod text;
pub mod document;

pub use htmlelement::HTMLElement;
pub use text::Text;
use crate::dom::{Document, Node};

pub enum Element {
    Text(Text),
    Document(Document),
    HTMLElement(HTMLElement),
}

impl Element {
    pub fn from_tag_name(name: &str) -> Self {
        match name {
            _ => {
                Element::HTMLElement(HTMLElement::new(name.to_string()))
            }
        }
    }

    pub fn from_text(text: String) -> Self {
        Element::Text(Text::new(text))
    }
}
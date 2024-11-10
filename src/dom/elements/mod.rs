mod htmlelement;
mod text;
pub mod document;

pub mod elements {
    use std::collections::HashMap;
    use crate::dom::{Document, HTMLElement, Text};

    pub enum Element {
        Text(Text),
        Document(Document),
        HTMLElement(HTMLElement),
    }

    impl Element {
        pub fn from_tag_name(name: &str, attributes: HashMap<String, String>) -> Self {
            match name {
                _ => {
                    Element::HTMLElement(HTMLElement::new(name.to_string(), attributes))
                }
            }
        }

        pub fn from_text(text: String) -> Self {
            Element::Text(Text::new(text))
        }
    }

    macro_rules! attr_getter {
        ($attr:ident, $htmlname:expr) => {
            pub fn $attr(&self) -> Option<&String> {
                self.attributes.get($htmlname)
            }
        }
    }

    pub(crate) use attr_getter;
}

pub use crate::dom::elements::htmlelement::HTMLElement;
pub use crate::dom::elements::text::Text;
pub use crate::dom::elements::elements::Element;
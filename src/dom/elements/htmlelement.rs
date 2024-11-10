use std::collections::HashMap;
use crate::dom::elements::elements::attr_getter;

pub struct HTMLElement {
    name: String,
    attributes: HashMap<String, String>,
}

impl HTMLElement {
    pub fn new(name: String, attributes: HashMap<String, String>) -> Self {
        HTMLElement {
            name,
            attributes,
        }
    }

    attr_getter!(id, "id");
    attr_getter!(class_name, "class");
}
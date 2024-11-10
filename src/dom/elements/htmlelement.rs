pub struct HTMLElement {
    name: String,
}

impl HTMLElement {
    pub fn new(name: String) -> Self {
        HTMLElement {
            name,
        }
    }
}
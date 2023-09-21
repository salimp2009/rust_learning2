#[derive(Debug, PartialEq)]
pub struct Post2 {
    content: String,
}

#[derive(Debug, PartialEq)]
pub struct DraftPost2 {
    content: String,
}

impl Post2 {
    pub fn new() -> DraftPost2 {
        DraftPost2 {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost2 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

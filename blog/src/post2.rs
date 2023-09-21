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

    pub fn request_review(self) -> PendingReviewPost2 {
        PendingReviewPost2 {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost2 {
    content: String,
}

impl PendingReviewPost2 {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}

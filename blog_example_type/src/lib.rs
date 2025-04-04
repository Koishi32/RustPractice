pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct Published{
    content:String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost{
        PendingReviewPost{
            content:self.content,
        }
    }
}

pub struct PendingReviewPost{
    content : String,
}

impl PendingReviewPost{
    pub fn approve(self) -> Published{
        Published {
            content: self.content,
        }
    }
}

impl Published {
    pub fn post(self) -> Post{
        Post{
            content:self.content,
        }
    }
}
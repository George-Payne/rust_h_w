pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    reviews: i32,
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
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            reviews: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }

    pub fn approve(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            reviews: self.reviews + 1,
        }
    }

    pub fn publish(self) -> Option<Post> {
        if self.reviews >= 2 {
            return Some(Post {
                content: self.content,
            });
        }

        return None;
    }
}

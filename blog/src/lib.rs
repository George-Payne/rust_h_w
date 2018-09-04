pub struct Post {
    state: Option<Box<State>>,
    content: String,
    draft: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            draft: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.add_text())
        }
        self.draft.push_str(text);
    }

    pub fn draft(&self) -> &str {
        &self.draft
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            let (state, approved) = s.approve();
            self.state = Some(state);
            if approved {
                self.content = self.draft.clone();
            }
        }
    }
}

trait State {
    fn add_text(self: Box<Self>) -> Box<State>;
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> (Box<State>, bool);
    fn reject(self: Box<Self>) -> Box<State>;
}

struct Draft {}

impl State for Draft {
    fn add_text(self: Box<Self>) -> Box<State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview { reviews: 0 })
    }
    fn approve(self: Box<Self>) -> (Box<State>, bool) {
        (self, false)
    }
    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

struct PendingReview {
    reviews: i32,
}

impl State for PendingReview {
    fn add_text(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> (Box<State>, bool) {
        if self.reviews >= 1 {
            return (Box::new(Published {}), true);
        }

        (
            Box::new(PendingReview {
                reviews: self.reviews + 1,
            }),
            false,
        )
    }
    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn add_text(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> (Box<State>, bool) {
        (self, false)
    }
    fn reject(self: Box<Self>) -> Box<State> {
        self
    }
}

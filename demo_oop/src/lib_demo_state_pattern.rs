pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

// ???: What is Default for?
impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // NOTE: Now we can start seeing the advantages of the state pattern:
    // the request_review method on Post is the same no matter its state value.
    // Each state is responsible for its own rules.
    pub fn request_review(&mut self) {
        // !!! This is where the Option in the state field of Post comes in:
        // we call the take method to take the Some value out of the state field and leave a None in its place,
        // because Rust doesn’t let us have unpopulated fields in structs.
        // This lets us move the state value out of Post rather than borrowing it.
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review()) // ??? Why can state call request_review but is of type Box<dyn State>.
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            // The goal is to keep all these rules contained within the State objects.
            self.state = Some(state.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        "" // We provide a default implementation.
    }
}

// One of the state
struct Draft {}

impl State for Draft {
    // NOTE: Here the function is taking ownership of a box containing self,
    // but we are not using it within the function.
    // So essentially we are invalidating the old state and
    // returning a new state that can be used in its place.
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        post.content.as_str()
    }
}

// TODO: review https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#why-not-an-enum

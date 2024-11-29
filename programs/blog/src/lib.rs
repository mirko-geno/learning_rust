pub enum Post {
    Draft(String),
    WaitingRev(String),
    Published(String)
}

impl Post {
    pub fn new() -> Post {
        Post::Draft(String::new())
    }

    pub fn add_text(&mut self, text: &str) {
        if let Post::Draft(t) = self {
            t.push_str(text);
        } else {
            panic!("Post is not Draft variant")
        }
    }

    pub fn request_review(&mut self) {
        if let Post::Draft(t) = self {
            *self = Post::WaitingRev(t.to_string())
        } else {
            panic!("Post is not Draft variant");
        }
    }

    pub fn approve(&mut self) {
        if let Post::WaitingRev(t) = self {
            *self = Post::Published(t.to_string());
        } else {
            panic!("Post is not WaitingRev variant");
        }
    }

    pub fn content(&self) -> &str {
        match self {
            Post::Draft(_) => "",               // Returns an empty &str
            Post::WaitingRev(_) => "",
            Post::Published(t) => &t
        }
    }
}
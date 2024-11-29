pub enum Post {
    Draft(String),
    WaitingRev(String),
    Published(String)
}

impl Post {
    pub fn new() -> Post {
        Post::Draft(String::new())
    }
}

impl Post {
    pub fn add_text(&mut self, text: &str) {
        if let Post::Draft(t) = self {
            t.push_str(text);
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Post::Draft(t) = self {
            t.push_str(text);
        }
    } 
}

impl Post::WaitingRev {
    pub fn approve(&mut self) {
        if let Post::WaitingRev(t) = self {
            *self = Post::Published(t.to_string());
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
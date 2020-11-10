

#[derive(Clone)]
pub struct DiConfig {
    pub name: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub about: &'static str,
}

impl DiConfig {
    pub fn new() -> Self {
        Self {
            name: "di",
            version: "0.0.1",
            author: "me",
            about: " ",
        }
    }
}

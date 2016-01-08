pub enum Field {
    Content(String),
    Priority(u32),
}

pub enum Action {
    Remove,
    Update(Field),
}

pub struct Widget {
    pub priority: u32,
    pub id: u32,
    pub content: String,
}

impl Widget {
    pub fn new(id: u32) -> Widget {
        Widget {
            id: id,
            content: String::new(),
            priority: 1024,
        }
    }

    pub fn update(&mut self, field: &Field) {
        match field {
            &Field::Content(ref s) => {self.content = s.clone()},
            &Field::Priority(p) => {self.priority = p},
        }
    }
}

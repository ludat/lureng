#[derive(Debug)]
pub enum Field {
    Content(String),
}

#[derive(Debug)]
pub enum Action {
    Remove,
    Up,
    Down,
    Update(Field),
}

#[derive(Debug)]
pub struct Widget {
    pub id: u32,
    pub content: String,
}

impl Widget {
    pub fn new(id: u32) -> Widget {
        Widget {
            id: id,
            content: String::new(),
        }
    }

    pub fn update(&mut self, field: &Field) {
        match *field {
            Field::Content(ref s) => {self.content = s.clone()},
        }
    }
}

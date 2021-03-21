#[derive(Debug)]
pub struct Background {
    pub images: Vec<Image>,
}

#[derive(Debug)]
pub enum Image {
    StartTime{
        year: u16,
        month: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
    },
    Static {
        duration: f32,
        file: String,
    },
    Transition {
        kind: String,
        duration: f32,
        from: String,
        to: String,
    }
}

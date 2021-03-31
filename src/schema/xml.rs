#[derive(Debug)]
pub struct Background {
    pub starttime: StartTime,
    pub images: Vec<Image>,
}

#[derive(Debug)]
pub struct StartTime {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub second: u16,
}

#[derive(Debug)]
pub enum Image {
    Static {
        duration: f32,
        file: String,
        idx: usize,
    },
    Transition {
        kind: String,
        duration: f32,
        from: String,
        to: String,
        idx: usize,
    }
}

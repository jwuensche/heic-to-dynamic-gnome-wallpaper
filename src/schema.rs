use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Background {
    #[serde(rename = "$value")]
    images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Image {
    #[serde(rename = "starttime")]
    StartTime(StartTime),
    #[serde(rename = "static")]
    Static {
        duration: f32,
        file: String,
    },
    #[serde(rename = "transition")]
    Transition {
        #[serde(rename = "type")]
        kind: String,
        duration: f32,
        from: String,
        to: String,
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartTime {
    year: u16,
    month: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
}

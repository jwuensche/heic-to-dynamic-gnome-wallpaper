use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WallpaperMetaTime {
    #[serde(rename = "ti")]
    pub time_slices: Vec<TimeSlice>,
    pub ap: Ap,
}

#[derive(Deserialize, Debug)]
pub struct Ap {
    pub d: i32,
    pub l: i32,
}

#[derive(Deserialize, Debug)]
pub struct TimeSlice {
    #[serde(rename = "t")]
    pub time: f32,
    #[serde(rename = "i")]
    pub idx: usize,
}

#[derive(Deserialize, Debug)]
pub struct WallpaperMetaSun {
    // TODO: Add Definition for Sun based Wallpapers
    // The `TimeSlice` struct will have to be replaced here.
}

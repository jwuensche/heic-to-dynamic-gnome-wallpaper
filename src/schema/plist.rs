use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WallpaperMetaTime {
    #[serde(rename = "ti")]
    pub time_slices: Vec<TimeSlice>,
    #[serde(rename = "ap")]
    pub appearance: Appearance,
}

#[derive(Deserialize, Debug)]
pub struct Appearance {
    #[serde(rename = "d")]
    pub dark: i32,
    #[serde(rename = "l")]
    pub light: i32,
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
    #[serde(rename = "si")]
    pub solar_slices: Vec<SolarSlice>,
    // #[serde(rename = "ap")]
    // pub appearance: Appearance,
}

#[derive(Deserialize, Debug)]
pub struct SolarSlice {
    #[serde(rename = "a")]
    pub altitude: f32,
    #[serde(rename = "i")]
    pub idx: usize,
    // #[serde(rename = "o")]
    // pub light_mode: usize,
    #[serde(rename = "z")]
    pub azimuth: f32,
}

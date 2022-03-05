// heic-to-dynamic-gnome-wallpaper
// Copyright (C) 2022 Johannes Wünsche
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
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

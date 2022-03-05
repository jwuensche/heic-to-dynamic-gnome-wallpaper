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
#[derive(Debug)]
pub struct Background {
    pub starttime: StartTime,
    pub images: Vec<Image>,
}

#[derive(Debug)]
pub struct StartTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
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
    },
}

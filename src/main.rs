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
use anyhow::Result;
use colored::*;
use libheif_rs::HeifContext;
use std::path::Path;

use clap::{Arg, Command};

mod image;
mod metadata;
mod schema;
mod serializer;
mod solar;
mod timebased;
mod util;

const INPUT: &str = "IMAGE";
const DIR: &str = "DIR";
const DAY_SECS: f32 = 86400.0;

fn main() -> Result<()> {
    let matches = Command::new("heic-to-dynamic-gnome-wallpaper")
        .arg(Arg::new(INPUT)
             .help("Image which should be transformed")
             .takes_value(true)
             .value_name(INPUT)
             .required(true)
        )
        .arg(Arg::new(DIR)
             .help("Into which directory the output images and schema should be written to.")
             .long_help("Specifies into which directory created images should be written to. Default is the parent directory of the given image.")
             .short('d')
             .long("dir")
             .takes_value(true)
             .value_name(DIR)
        )
        .get_matches();
    let path = matches
        .value_of(INPUT)
        .ok_or_else(|| anyhow::Error::msg("Could not read INPUT"))?;

    let parent_directory = if matches.is_present(DIR) {
        let nu_path = std::path::Path::new(matches.value_of(DIR).unwrap()).to_path_buf();
        if !nu_path.exists() {
            std::fs::create_dir_all(&nu_path)?
        }
        nu_path.canonicalize()?
    } else {
        std::path::Path::new(path)
            .canonicalize()
            .map_err(|e| {
                anyhow::Error::msg(format!("Cannot get absolute path of the given file: {}", e))
            })?
            .ancestors()
            .nth(1)
            .ok_or_else(|| {
                anyhow::Error::msg(format!(
                    "Cannot get parent of given image path: \"{}\"",
                    path
                ))
            })?
            .to_path_buf()
    };
    let image_ctx = HeifContext::read_from_file(path)?;

    // FETCH file wide metadata
    println!(
        "{}: Fetch metadata from image...",
        "Preparation".bright_blue(),
    );
    let base64plist = metadata::get_wallpaper_metadata(&image_ctx);

    if base64plist.is_none() {
        return Err(anyhow::Error::msg("No valid metadata found describing wallpaper! Please check if the mime field is available and carries an apple_desktop:h24 and/or apple_desktop:solar value"));
    }

    let image_name = Path::new(path)
        .file_stem()
        .expect("Could not get file name of path")
        .to_string_lossy();

    println!(
        "{}: Detecting wallpaper description type...",
        "Preparation".bright_blue(),
    );
    match base64plist.unwrap() {
        metadata::WallPaperMode::H24(content) => {
            println!(
                "{}: Detected time-based wallpaper.",
                "Preparation".bright_blue(),
            );
            timebased::compute_time_based_wallpaper(
                image_ctx,
                content,
                &parent_directory,
                &image_name,
            )
        }
        metadata::WallPaperMode::Solar(content) => {
            println!(
                "{}: Detected solar-based wallpaper.",
                "Preparation".bright_blue(),
            );
            solar::compute_solar_based_wallpaper(image_ctx, content, &parent_directory, &image_name)
        }
    }
}

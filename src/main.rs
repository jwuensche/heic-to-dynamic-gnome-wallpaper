use anyhow::Result;
use libheif_rs::HeifContext;
use colored::*;

use clap::{App, Arg};

mod metadata;
mod schema;
mod serializer;
mod timebased;
mod util;

const INPUT: &str = "IMAGE";
const DIR: &str = "DIR";

fn main() -> Result<()> {
    let matches = App::new("heic-to-gnome-xml-wallpaper")
        .arg(Arg::with_name(INPUT)
             .help("Image which should be transformed")
             .takes_value(true)
             .value_name(INPUT)
             .required(true)
        )

.arg(Arg::with_name(DIR)
             .help("Into which directory the output images and schema should be written to.")
             .long_help("Specifies into which directory created images should be written to. Default is the parent directory of the given image.")
             .short("d")
             .long("dir")
             .takes_value(true)
             .value_name(DIR)
        )
        .get_matches();
    let path = matches
        .value_of(INPUT)
        .ok_or(anyhow::Error::msg("Could not read INPUT"))?;

    let parent_directory;
    if matches.is_present(DIR) {
        let nu_path = std::path::Path::new(matches.value_of(DIR).unwrap()).to_path_buf();
        if !nu_path.exists() {
            std::fs::create_dir_all(&nu_path)?
        }
        parent_directory = nu_path.canonicalize()?;
    } else {
        parent_directory = std::path::Path::new(path)
            .ancestors()
            .nth(1)
            .unwrap()
            .canonicalize()
            .unwrap();
    }
    let image_ctx = HeifContext::read_from_file(path).unwrap();

    // FETCH file wide metadata
    println!("{}: {}", "Preparation".bright_blue(), "Fetch metadata from image");
    let base64plist = metadata::get_wallpaper_metadata(&image_ctx);

    if let None = base64plist {
        eprintln!("No valid metadata found describing wallpaper! Please check if the mime field is available and carries an apple_desktop:h24 and/or apple_desktop:solar value");
        return Err(anyhow::Error::msg("No valid metadata"));
    }

    println!("{}: {}", "Preparation".bright_blue(), "Detecting wallpaper description kind");
    match base64plist.unwrap() {
        metadata::WallPaperMode::H24(content) => {
            println!("{}: {}", "Preparation".bright_blue(), "Detected time-based wallpaper");
            timebased::compute_time_based_wallpaper(image_ctx, content, &parent_directory)
        }
        metadata::WallPaperMode::Solar(_content) => {
            println!("{}: {}", "Preparation".bright_blue(), "Detected solar-based wallpaper");
            eprintln!("Solar is not supported at the moment, please use wallpapers only with time based changes.");
            std::process::exit(1)
        }
    }
}

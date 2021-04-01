use libheif_rs::HeifContext;
use anyhow::Result;

use clap::{App, Arg};

mod schema;
mod serializer;
mod metadata;
mod util;
mod timebased;

fn main() -> Result<()> {
    let matches = App::new("heic-to-gxml")
        .arg(Arg::with_name("INPUT")
             .help("Image which should be transformed")
             .takes_value(true)
             .value_name("INPUT")
             .required(true)
             .index(1))
        .get_matches();
    let path = matches.value_of("INPUT").ok_or(anyhow::Error::msg("Could not read INPUT"))?;
    let parent_directory = std::path::Path::new(path).ancestors().nth(1).unwrap().canonicalize().unwrap();
    let image_ctx = HeifContext::read_from_file(path).unwrap();

    // FETCH file wide metadata
    let base64plist = metadata::get_wallpaper_metadata(&image_ctx);

    if let None = base64plist {
        eprintln!("No valid metadata found describing wallpaper! Please check if the mime field is available and carries an apple_desktop:h24 and/or apple_desktop:solar value");
        return Err(anyhow::Error::msg("No valid metadata"))
    }

    match base64plist.unwrap() {
        metadata::WallPaperMode::H24(content) => {
            timebased::compute_time_based_wallpaper(image_ctx, content, &parent_directory)
        },
        metadata::WallPaperMode::Solar(content) => {
            eprintln!("Solar is not supported at the moment, please use wallpapers only with time based changes.");
            std::process::exit(1)
        }
    }

}

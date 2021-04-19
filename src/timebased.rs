use std::path::Path;

use crate::image::{process_img, save_xml, ImagePoint};
use crate::metadata;
use crate::schema::plist::TimeSlice;
use crate::schema::xml::{Background, StartTime};
use crate::DAY_SECS;

use crate::util::time;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use libheif_rs::HeifContext;

pub fn compute_time_based_wallpaper(
    image_ctx: HeifContext,
    content: String,
    parent_directory: &Path,
    image_name: &str,
) -> Result<()> {
    let mut plist = metadata::get_time_plist_from_base64(&content)?;
    //println!("Found plist {:?}", plist);

    plist
        .time_slices
        .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    let start_time = plist.time_slices.get(0).unwrap().time;
    let start_seconds = (start_time * DAY_SECS) as u16;
    let mut xml_background = Background {
        images: Vec::new(),
        starttime: StartTime {
            year: 2011,
            month: 10,
            day: 1,
            hour: time::to_rem_hours(start_seconds),
            minute: time::to_rem_min(start_seconds),
            second: time::to_rem_sec(start_seconds),
        },
    };

    let number_of_images = image_ctx.number_of_top_level_images();
    println!(
        "{}: {} {} {}",
        "Preparation".bright_blue(),
        "Found",
        number_of_images,
        "images"
    );
    let mut image_ids = vec![0u32; number_of_images];
    image_ctx.top_level_image_ids(&mut image_ids);
    println!(
        "{}: {}",
        "Conversion".yellow(),
        "Converting embedded images to png format"
    );
    let pb = ProgressBar::new(number_of_images as u64).with_style(
        ProgressStyle::default_bar()
            .template("Conversion: {wide_bar} {pos}/{len} [ETA: {eta_precise}]")
            .progress_chars("## "),
    );
    for (time_idx, TimeSlice { time, idx }) in
        plist.time_slices.iter().enumerate().progress_with(pb)
    {
        let img_id = *image_ids
            .get(*idx)
            .expect("Could not fetch image id described in metadata");
        //println!("Image ID: {:?}", img_id);
        let pt = ImagePoint {
            image_ctx: &image_ctx,
            img_id,
            index: time_idx,
            background: &mut xml_background,
            parent_directory,
            start_time,
            time: *time,
            next_time: plist
                .time_slices
                .get(time_idx + 1)
                .map(|elem| elem.time)
                .unwrap_or(0f32),
        };
        process_img(pt)?;
    }

    save_xml(&mut xml_background, parent_directory, image_name)
}

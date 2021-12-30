use crate::image::{self, ImagePoint};
use crate::schema::xml::{Background, StartTime};
use crate::util::time;
use crate::DAY_SECS;
use crate::{image::process_img, metadata};
use anyhow::Result;
use chrono::Datelike;
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use libheif_rs::HeifContext;
use std::cmp::Ordering;
use std::path::Path;

use colored::*;
#[derive(Debug)]
struct SolarToHourSlice {
    time: f32,
    index: usize,
}

pub fn compute_solar_based_wallpaper(
    image_ctx: HeifContext,
    content: String,
    parent_directory: &Path,
    image_name: &str,
) -> Result<()> {
    let mut plist = metadata::get_solar_plist_from_base64(&content)?;

    plist
        .solar_slices
        .sort_by(|x, y| x.azimuth.partial_cmp(&y.azimuth).unwrap_or(Ordering::Equal));
    let time_slices: Vec<SolarToHourSlice> = plist
        .solar_slices
        .iter()
        .map(|elem| SolarToHourSlice {
            time: elem.azimuth / 360f32,
            index: elem.idx,
        })
        .collect();
    let mut img_ids = vec![0; image_ctx.number_of_top_level_images()];
    image_ctx.top_level_image_ids(&mut img_ids);

    let start_time = time_slices.get(0).expect("No image has been found").time;
    let start_seconds = (start_time * DAY_SECS) as u16;
    let date = chrono::Local::now();
    let mut background_definition = Background {
        starttime: StartTime {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            hour: time::to_rem_hours(start_seconds),
            minute: time::to_rem_min(start_seconds),
            second: time::to_rem_sec(start_seconds),
        },
        images: vec![],
    };

    println!("{}:", "Conversion".green());
    let pb = ProgressBar::new(image_ctx.number_of_top_level_images() as u64).with_style(
        ProgressStyle::default_bar()
            .template("[{wide_bar}] {pos}/{len} [ETA: {eta_precise}]")
            .progress_chars("## "),
    );
    for (idx, SolarToHourSlice { time, index }) in time_slices.iter().enumerate().progress_with(pb)
    {
        let img_id = img_ids[*index];
        let pt = ImagePoint {
            image_ctx: &image_ctx,
            img_id,
            index: idx,
            background: &mut background_definition,
            parent_directory,
            start_time,
            time: *time,
            next_time: time_slices
                .get(idx + 1)
                .map(|elem| elem.time)
                .unwrap_or(0f32),
        };
        process_img(pt)?;
    }
    image::save_xml(&mut background_definition, parent_directory, image_name)
}

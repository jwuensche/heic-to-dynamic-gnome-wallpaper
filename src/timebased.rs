use std::{io::BufWriter, path::Path};

use anyhow::Result;
use libheif_rs::HeifContext;
use crate::metadata;
use crate::schema::xml::{Image::{Static, Transition}, StartTime, Background};
use crate::schema::plist::TimeSlice;
use crate::util::{time, png};
use crate::serializer::GnomeXMLBackgroundSerializer;

const DAY_SECS: f32 = 86400.0;

pub fn compute_time_based_wallpaper(image_ctx: HeifContext, content: String, parent_directory: &Path) -> Result<()> {
    let mut plist = metadata::get_time_plist_from_base64(&content)?;
    println!("Found plist {:?}", plist);

    plist.time_slices.sort_by(|a,b| a.time.partial_cmp(&b.time).unwrap());
    let first_time = plist.time_slices.get(0).unwrap().time as u16;
    let mut xml_background = Background {
        images: Vec::new(),
        starttime: StartTime {
            year: 2011,
            month: 10,
            day: 1,
            hour: time::to_rem_hours(first_time),
            minute: time::to_rem_min(first_time),
            second: time::to_rem_sec(first_time),
        }};


    let number_of_images = image_ctx.number_of_top_level_images();
    let mut image_ids = Vec::new();
    image_ctx.top_level_image_ids(&mut image_ids);
    for (time_idx, TimeSlice{time, idx}) in plist.time_slices.iter().enumerate() {
        let img_id = *image_ids.get(*idx).expect("Could not fetch image id described in metadata");
        println!("Image ID: {:?}", img_id);
        let prim_image = image_ctx.image_handle(img_id).unwrap();
        png::write_png(format!("{}/{}.png",parent_directory.to_string_lossy(), time_idx).as_str(), prim_image)?;

        // Add to Background Structure
        xml_background.images.push(Static {
            duration: 1 as f32,
            file: format!("{}/{}.png",parent_directory.to_string_lossy(), time_idx),
            idx: time_idx,
        });

        xml_background.images.push(Transition {
            kind: "overlay".to_string(),
            duration: {
                if time_idx < number_of_images - 1 {
                    (time - plist.time_slices.get(time_idx + 1).unwrap().time).abs() * DAY_SECS - 1.0
                } else {
                    let first_time = plist.time_slices.get(0).unwrap().time;
                    (((time - 1.0).abs() + first_time) * DAY_SECS - 1.0).ceil()
                }
            },
            from: format!("{}/{}.png", parent_directory.to_string_lossy(), time_idx),
            to: format!("{}/{}.png", parent_directory.to_string_lossy(), {
                if time_idx < number_of_images - 1 {
                    time_idx + 1
                } else {
                    0
                }
            }),
            idx: time_idx,
        });
    }

    xml_background.images.sort_by(|a,b| {
        match (a,b) {
            (Static{ idx: static_idx, .. }, Transition{ idx: transition_idx, ..}) => static_idx.cmp(&transition_idx),
            (Static{ idx: static_idx, .. }, Static{ idx: transition_idx, ..}) => static_idx.cmp(&transition_idx),
            (Transition{ idx: static_idx, .. }, Static{ idx: transition_idx, ..}) => static_idx.cmp(&transition_idx),
            (Transition{ idx: static_idx, .. }, Transition{ idx: transition_idx, ..}) => static_idx.cmp(&transition_idx),
        }
    });

    let result_file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(format!("{}/default.xml", parent_directory.to_string_lossy()))?;
    let mut result = BufWriter::new(result_file);
    let mut ser = GnomeXMLBackgroundSerializer::new(&mut result);
    ser.serialize(&xml_background)?;
    Ok(())
}

use libheif_rs::{RgbChroma, ColorSpace, HeifContext};
use std::io::{Write, BufWriter};
use anyhow::Result;

use clap::{App, Arg};

mod schema;
mod serializer;
mod metadata;
mod util;

use schema::{plist::TimeSlice, xml::{self}};
use schema::xml::Image::Static;
use schema::xml::Image::Transition;

const DAY_SECS: f32 = 86400.0;

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

    let number_of_images = image_ctx.number_of_top_level_images();
    println!("File contains {} images", number_of_images);

    // FETCH file wide metadata
    let base64plist = metadata::get_wallpaper_metadata(&image_ctx);

    if let None = base64plist {
        eprintln!("No valid metadata found describing wallpaper! Please check if the mime field is available and carries an apple_desktop:h24 and/or apple_desktop:solar value");
        return Err(anyhow::Error::msg("No valid metadata"))
    }

    let mut plist = metadata::get_time_plist_from_base64(&base64plist.unwrap())?;
    println!("Found plist {:?}", plist);

    plist.time_slices.sort_by(|a,b| a.time.partial_cmp(&b.time).unwrap());
    let first_time = plist.time_slices.get(0).unwrap().time as u16;
    let mut xml_background = xml::Background {
        images: Vec::new(),
        starttime: xml::StartTime {
        year: 2011,
        month: 10,
        day: 1,
        hour: util::to_rem_hours(first_time),
        minute: util::to_rem_min(first_time),
        second: util::to_rem_sec(first_time),
        }};


    let image_ids = image_ctx.list_of_image_handle_ids(number_of_images);
    for (time_idx, TimeSlice{time, idx}) in plist.time_slices.iter().enumerate() {
        let img_id = *image_ids.get(*idx).expect("Could not fetch image id described in metadata");
        println!("Image ID: {:?}", img_id);
        let prim_image = image_ctx.image_handle(img_id).unwrap();

        let width = prim_image.width();
        let height = prim_image.height();
        //let decoded = prim_image.decode(ColorSpace::YCbCr(libheif_rs::Chroma::C444), false).unwrap();
        let decoded = prim_image.decode(ColorSpace::Rgb(RgbChroma::C444), false).unwrap();
        let planes = decoded.planes();

        let red = planes.r.unwrap().data;
        let green = planes.g.unwrap().data;
        let blue = planes.b.unwrap().data;

        let file = std::fs::OpenOptions::new().create(true).write(true).open(format!("{}/{}.png",parent_directory.to_string_lossy(), time_idx))?;
        let writer = BufWriter::new(file);

        let mut pngencoder = png::Encoder::new(writer, width, height);
        pngencoder.set_color(png::ColorType::RGB);
        pngencoder.set_depth(png::BitDepth::Eight);
        let image_writer = pngencoder.write_header()?;
        let mut w = image_writer.into_stream_writer();

        println!("Writing image");
        for ((red, green), blue) in red.into_iter().zip(green.into_iter()).zip(blue.into_iter()) {
            w.write(&[*red, *green, *blue])?;
        }

        // Add to Background Structure

        xml_background.images.push(xml::Image::Static {
            duration: 1 as f32,
            file: format!("{}/{}.png",parent_directory.to_string_lossy(), time_idx),
            idx: time_idx,
        });

        xml_background.images.push(xml::Image::Transition {
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
            (Static{ idx: static_idx, .. }, Transition{ idx: transition_idx, ..}) => static_idx.cmp(transition_idx),
            (Static{ idx: static_idx, .. }, Static{ idx: transition_idx, ..}) => static_idx.cmp(transition_idx),
            (Transition{ idx: static_idx, .. }, Static{ idx: transition_idx, ..}) => static_idx.cmp(transition_idx),
            (Transition{ idx: static_idx, .. }, Transition{ idx: transition_idx, ..}) => static_idx.cmp(transition_idx),
        }
    });

    let result_file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open(format!("{}/default.xml", parent_directory.to_string_lossy()))?;
    let mut result = BufWriter::new(result_file);
    let mut ser = serializer::GnomeXMLBackgroundSerializer::new(&mut result);
    ser.serialize(&xml_background)?;
    Ok(())
}

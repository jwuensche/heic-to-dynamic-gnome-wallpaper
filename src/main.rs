use libheif_rs::{RgbChroma, ColorSpace, HeifContext};
use std::io::{Write, BufWriter};
use anyhow::Result;

use clap::{App, Arg};

mod schema;
mod serializer;

use schema::xml;

fn main() -> Result<()> {
    let matches = App::new("heic-to-gxml")
        .arg(Arg::with_name("INPUT")
             .help("Image which should be transformed")
             .takes_value(true)
             .value_name("INPUT")
             .required(true)
             .index(1))
        .get_matches();
    let mut xml_background = xml::Background {
        images: Vec::new(),
        starttime: xml::StartTime {
        year: 2011,
        month: 10,
        day: 1,
        hour: 0b111,
        minute: 0,
        second: 0,
    }};
    let path = matches.value_of("INPUT").ok_or(anyhow::Error::msg("Could not read INPUT"))?;
    let image_ctx = HeifContext::read_from_file(path).unwrap();

    let number_of_images = image_ctx.number_of_top_level_images();
    println!("File contains {} images", number_of_images);

    let average_length = 86400 / number_of_images / 2;

    for (img_no, img_id) in image_ctx.list_of_image_handle_ids(number_of_images).into_iter().enumerate() {
        println!("{:?}", img_id);
        let prim_image = image_ctx.image_handle(img_id).unwrap();
        let metadata = prim_image.list_of_metadata_block_ids("", 100);

        let width = prim_image.width();
        let height = prim_image.height();
        //let decoded = prim_image.decode(ColorSpace::YCbCr(libheif_rs::Chroma::C444), false).unwrap();
        let decoded = prim_image.decode(ColorSpace::Rgb(RgbChroma::C444), false).unwrap();
        let planes = decoded.planes();

        for id in metadata.into_iter() {
            println!("{:?}", prim_image.metadata_type(id));
            println!("{:?}", prim_image.metadata_content_type(id));
            println!("{:?}", String::from_utf8(prim_image.metadata(id).unwrap()));
        }

        let red = planes.r.unwrap().data;
        let green = planes.g.unwrap().data;
        let blue = planes.b.unwrap().data;
        let p = std::path::Path::new(path).ancestors().nth(1).unwrap().canonicalize().unwrap();

        let file = std::fs::OpenOptions::new().create(true).write(true).open(format!("{}/{}.png",p.to_string_lossy(), img_no))?;
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
            duration: average_length as f32,
            file: format!("{}/{}.png",p.to_string_lossy(), img_no)
        });

        xml_background.images.push(xml::Image::Transition {
            kind: "overlay".to_string(),
            duration: average_length as f32,
            from: format!("{}/{}.png", p.to_string_lossy(), img_no),
            to: format!("{}/{}.png", p.to_string_lossy(), {
                if img_no < number_of_images - 1 {
                    img_no + 1
                } else {
                    0
                }
            })

        });
    }

    let result_file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("default.xml")?;
    let mut result = BufWriter::new(result_file);
    let mut ser = serializer::GnomeXMLBackgroundSerializer::new(&mut result);
    ser.serialize(&xml_background)?;
    Ok(())
}

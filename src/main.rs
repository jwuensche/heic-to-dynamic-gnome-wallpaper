use libheif_rs::{RgbChroma, ColorSpace, HeifContext};
use std::io::{Write, BufWriter};
use anyhow::Result;

use clap::{App, Arg};

mod schema;
mod serializer;

fn main() -> Result<()> {
    let matches = App::new("heic-to-gxml")
        .arg(Arg::with_name("INPUT")
             .help("Image which should be transformed")
             .takes_value(true)
             .value_name("INPUT")
             .required(true)
             .index(1))
        .get_matches();

    let mut xml_background = schema::Background {
        images: Vec::new(),
    };
    xml_background.images.push(schema::Image::StartTime {
        year: 2011,
        month: 10,
        day: 1,
        hour: 7,
        minute: 0,
        second: 0,
    });
    let path = matches.value_of("INPUT").ok_or(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not read INPUT"))?;
    let image_ctx = HeifContext::read_from_file(path).unwrap();
    println!("File contains {} images", image_ctx.number_of_top_level_images());
    for (img_no, img_id) in image_ctx.list_of_image_handle_ids(100).into_iter().enumerate() {
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

        xml_background.images.push(schema::Image::Static {
            duration: 3600.0,
            file: format!("{}/{}.png",p.to_string_lossy(), img_no)
        })
    }

    let result_file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("default.xml")?;
    let mut result = BufWriter::new(result_file);
    let mut ser = serializer::GnomeXMLBackgroundSerializer::new(&mut result);
    ser.serialize(&xml_background)?;
    Ok(())
}

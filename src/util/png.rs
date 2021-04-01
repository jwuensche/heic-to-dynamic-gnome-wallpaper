use std::io::{BufWriter, Write};

use anyhow::Result;
use libheif_rs::{ColorSpace, ImageHandle, RgbChroma};

pub fn write_png(path: &str, handle: ImageHandle) -> Result<()> {
    let width = handle.width();
    let height = handle.height();
    //let decoded = handle.decode(ColorSpace::YCbCr(libheif_rs::Chroma::C444), false).unwrap();
    let decoded = handle.decode(ColorSpace::Rgb(RgbChroma::C444), false).unwrap();
    let planes = decoded.planes();

    let red = planes.r.unwrap().data;
    let green = planes.g.unwrap().data;
    let blue = planes.b.unwrap().data;

    let file = std::fs::OpenOptions::new().create(true).write(true).open(path)?;
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
    Ok(())
}

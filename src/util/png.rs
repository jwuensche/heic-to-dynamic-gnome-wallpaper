use colored::*;
use std::io::{BufWriter, Write};

use anyhow::Result;
use libheif_rs::{ColorSpace, ImageHandle, RgbChroma};

pub fn write_png(path: &str, handle: ImageHandle) -> Result<()> {
    let width = handle.width();
    let height = handle.height();
    //let decoded = handle.decode(ColorSpace::YCbCr(libheif_rs::Chroma::C444), false).unwrap();
    let res = handle.decode(ColorSpace::Rgb(RgbChroma::C444), false);
    if let Ok(decoded) = res {
        let planes = decoded.planes();

        let red = planes.r.unwrap().data;
        let green = planes.g.unwrap().data;
        let blue = planes.b.unwrap().data;

        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        let writer = BufWriter::new(file);

        // In some cases data gets read which is not valid in itself, this is the case when the expected size of data (width * height) is superceded by the amount of data encoded in the planes
        // This may have different reasons, being image blocks etc...
        let actual = red.len() + green.len() + blue.len();
        if red.len() != green.len() || green.len() != blue.len() {
            return Err(anyhow::Error::msg("Length of color planes are unequal. The image data is probably invalid, please check the used image in another application."));
        }
        let expected = width * height * 3;
        let offset = (actual as u32 - expected) / 3 / height;

        let mut pngencoder = png::Encoder::new(writer, width, height);
        pngencoder.set_color(png::ColorType::Rgb);
        pngencoder.set_depth(png::BitDepth::Eight);
        let image_writer = pngencoder.write_header()?;
        let mut w = image_writer.into_stream_writer()?;

        //println!("Writing image");
        for (_, ((red, green), blue)) in red
            .iter()
            .zip(green.iter())
            .zip(blue.iter())
            .enumerate()
            .filter(|(id, _)| *id as u32 % (width + offset) < width)
        {
            w.write_all(&[*red, *green, *blue])?;
        }
        return Ok(());
    }
    println!(
        "{}: Could not determine color space. Colorspace RGB C444 could not be applied",
        "Error".red(),
    );
    Err(anyhow::Error::msg(format!(
        "Could not decode the image data in RGB C444 colorspace: {:?}",
        res.err().unwrap()
    )))
}

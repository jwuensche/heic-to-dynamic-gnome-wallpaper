use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result};
use std::io::{Write, BufWriter};

fn main() -> std::io::Result<()> {
    let image_ctx = HeifContext::read_from_file("images/foo.heic").unwrap();
    println!("File contains {} images", image_ctx.number_of_top_level_images());
    for (img_no, img_id) in image_ctx.list_of_image_handle_ids(10).into_iter().enumerate() {
        println!("{:?}", img_id);
        let prim_image = image_ctx.image_handle(img_id).unwrap();
        let metadata = prim_image.list_of_metadata_block_ids("", 10);

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

        let file = std::fs::OpenOptions::new().create(true).write(true).open(format!("images/{}.png", img_no))?;
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



    }
    Ok(())
}

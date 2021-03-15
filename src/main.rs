use libheif_rs::{Channel, RgbChroma, ColorSpace, HeifContext, Result};

fn main() -> std::io::Result<()> {
    let image_ctx = HeifContext::read_from_file("foo.heic").unwrap();
    println!("File contains {} images", image_ctx.number_of_top_level_images());
    for img_id in image_ctx.list_of_image_handle_ids(10) {
        println!("{:?}", img_id);
        let prim_image = image_ctx.image_handle(img_id).unwrap();
        let exifs = prim_image.list_of_metadata_block_ids("", 10);

        // Using non-ycbcr colorspaces leads to silent dropping of planes...
        let decoded = prim_image.decode(ColorSpace::YCbCr(libheif_rs::Chroma::C444), false).unwrap();
        let planes = decoded.planes();

        println!("{:?}", planes.y.unwrap().data);
        println!("{:?}", planes.cb.is_some());
        println!("{:?}", planes.cr.is_some());
        println!("{:?}", planes.r.is_some());
        println!("{:?}", planes.g.is_some());
        println!("{:?}", planes.b.is_some());
        println!("{:?}", planes.a.is_some());

        // for antonov in planes.iter() {
        //     println!("Found plane: {:?}", antonov);
        // }

        for id in exifs.into_iter() {
            println!("{:?}", prim_image.metadata_type(id));
            println!("{:?}", prim_image.metadata_content_type(id));
            println!("{:?}", String::from_utf8(prim_image.metadata(id).unwrap()));
        }
    }
    Ok(())
}

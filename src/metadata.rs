use libheif_rs::HeifContext;
use quick_xml::{Reader, events::Event};
use anyhow::Result;

use crate::schema::plist::WallpaperMetaTime;

pub fn get_wallpaper_metadata(image_ctx: &HeifContext) -> Option<String> {
    // Fetch META information about all images (These are by standard stored in the first images meta information tags)
    let metadatas = image_ctx.primary_image_handle().unwrap().list_of_metadata_block_ids("mime", 1);
    let metadata_id = metadatas.get(0).expect("Could not get metadata information");
    let base64plist = {
        let foo = image_ctx.primary_image_handle().unwrap().metadata(*metadata_id).unwrap();
        let content = String::from_utf8_lossy(&foo);
        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut h24 = None;


        loop {
            match reader.read_event(&mut buf) {
                Ok(quick_xml::events::Event::Empty(ref e)) => {
                    e.attributes()
                        .filter(|att| att.as_ref().unwrap().key == "apple_desktop:h24".as_bytes())
                        .for_each(|att| {
                            h24 = Some(att.unwrap().value.into_owned());
                        });
                    break
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }
        h24
    };
    return base64plist.map(|content| String::from_utf8_lossy(&content).into_owned())
}

pub fn get_time_plist_from_base64(input: &String) -> Result<WallpaperMetaTime> {
    let decoded = base64::decode(input)?;
    let plist = plist::from_bytes(&decoded)?;
    Ok(plist)
}

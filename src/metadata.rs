use anyhow::Result;
use libheif_rs::HeifContext;
use quick_xml::{events::Event, Reader};

use crate::schema::plist::{WallpaperMetaSun, WallpaperMetaTime};

pub enum WallPaperMode {
    H24(String),
    Solar(String),
}

pub fn get_wallpaper_metadata(image_ctx: &HeifContext) -> Option<WallPaperMode> {
    // Fetch META information about all images (These are by standard stored in the first images meta information tags)
    let mut metadatas = vec![0; 1];
    image_ctx
        .primary_image_handle()
        .unwrap()
        .metadata_block_ids("mime", &mut metadatas);
    let metadata_id = metadatas
        .get(0)
        .expect("Could not get metadata information");
    let base64plist = {
        let foo = image_ctx
            .primary_image_handle()
            .unwrap()
            .metadata(*metadata_id)
            .unwrap();
        let content = String::from_utf8_lossy(&foo);
        //println!("{:?}", content);
        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut h24 = None;

        loop {
            match reader.read_event(&mut buf) {
                Ok(quick_xml::events::Event::Empty(ref e)) => {
                    e.attributes()
                        .filter(|att| {
                            att.as_ref().unwrap().key == "apple_desktop:h24".as_bytes()
                                || att.as_ref().unwrap().key == "apple_desktop:solar".as_bytes()
                        })
                        .for_each(|att| match att.as_ref().unwrap().key {
                            s if s == "apple_desktop:h24".as_bytes() => {
                                h24 = Some(WallPaperMode::H24(
                                    String::from_utf8_lossy(&att.unwrap().value.into_owned())
                                        .to_string(),
                                ))
                            }
                            s if s == "apple_desktop:solar".as_bytes() => {
                                h24 = Some(WallPaperMode::Solar(
                                    String::from_utf8_lossy(&att.unwrap().value.into_owned())
                                        .to_string(),
                                ))
                            }
                            _ => panic!("Invalid Branch"),
                        });
                    break;
                }
                Ok(Event::Eof) => break,
                Err(_) => break,
                _ => {}
            }
        }
        h24
    };
    return base64plist;
}

pub fn get_time_plist_from_base64(input: &String) -> Result<WallpaperMetaTime> {
    let decoded = base64::decode(input)?;
    let plist = plist::from_bytes(&decoded)?;
    Ok(plist)
}

pub fn get_solar_plist_from_base64(input: &String) -> Result<WallpaperMetaSun> {
    let decoded = base64::decode(input)?;
    let plist = plist::from_bytes(&decoded)?;
    Ok(plist)
}

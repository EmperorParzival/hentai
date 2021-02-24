const PROTO: &str = "https://";
const BASE: &str = "nhentai.net";

fn get_type(file_type: &str) -> &str {
    if file_type == "j" {
        "jpg"
    } else if file_type == "p" {
        "png"
    } else {
        "gif"
    }
}

pub fn doujin(id: u32) -> String {
    format!("{}{}/api/gallery/{}", PROTO, BASE, id.to_string())
}

pub fn doujin_url(id: u32) -> String {
    format!("{}{}/g/{}", PROTO, BASE, id.to_string())
}

pub fn thumbnail(media_id: &String, file_type: &String) -> String {
    format!(
        "{}t.{}/galleries/{}/thumb.{}",
        PROTO,
        BASE,
        media_id,
        get_type(file_type)
    )
}

pub fn cover(media_id: &String, file_type: &String) -> String {
    format!(
        "{}t.{}/galleries/{}/cover.{}",
        PROTO,
        BASE,
        media_id,
        get_type(file_type)
    )
}

pub fn page(media_id: &String, number: u32, file_type: &String) -> String {
    format!(
        "{}i.{}/galleries/{}/{}.{}",
        PROTO,
        BASE,
        media_id,
        number.to_string(),
        get_type(file_type)
    )
}

pub fn page_thumbnail(media_id: &String, number: u32, file_type: &String) -> String {
    format!(
        "{}t.{}/galleries/{}/{}t.{}",
        PROTO,
        BASE,
        media_id,
        number.to_string(),
        get_type(file_type)
    )
}

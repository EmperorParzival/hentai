#[allow(dead_code)]
pub fn search_tag(tag_id: u32, page: u32) -> String {
    format!(
        "https://nhentai.net/api/gallery/tagged?tag_id={}&page={}",
        tag_id.to_string(),
        page.to_string()
    )
}

#[allow(dead_code)]
pub fn search_similar(doujin_id: u32) -> String {
    format!("https://nhentai.net/api/gallery/{}/related", doujin_id.to_string())
}

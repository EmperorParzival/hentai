mod parse;
pub(crate) mod utility;

use chrono::{DateTime, Utc};
use parse::{Doujin, Title};
use std::{convert::TryInto, path::PathBuf};
use utility::{api::url, error::Result};

#[derive(Debug)]
pub struct Hentai {
    pub url: String,
    pub title: Title,
    pub num_pages: u32,
    pub media_id: String,
    pub scanlator: String,
    pub cover_url: String,
    pub num_favorites: u32,
    pub thumbnail_url: String,
    pub image_urls: Vec<String>,
    pub upload_date: DateTime<Utc>,
}

fn reconstruct_title(raw: &Doujin) -> Title {
    match &raw.title {
        Title {
            pretty,
            english,
            japanese,
        } => Title {
            pretty: pretty.clone(),
            english: english.clone(),
            japanese: japanese.clone(),
        },
    }
}

fn create_urls(raw: &Doujin) -> Vec<String> {
    raw.images
        .pages
        .iter()
        .enumerate()
        .map(|(i, v)| url::page(&raw.media_id, i.try_into().unwrap(), &v.t))
        .collect::<Vec<String>>()
}

fn organize_fields(raw: Doujin) -> Hentai {
    let media_id = &raw.media_id;

    Hentai {
        image_urls: create_urls(&raw),
        title: reconstruct_title(&raw),
        media_id: media_id.to_string(),

        num_pages: raw.num_pages,
        scanlator: raw.scanlator,
        upload_date: raw.upload_date,
        num_favorites: raw.num_favorites,

        url: url::doujin(raw.id),
        cover_url: url::cover(media_id, &raw.images.cover.t),
        thumbnail_url: url::thumbnail(media_id, &raw.images.thumbnail.t),
    }
}

impl Hentai {
    pub async fn new(code: u32) -> Result<Self> {
        let result = Doujin::new(code).await?;

        Ok(organize_fields(result))
    }

    pub fn from_json(path: PathBuf) -> Result<Self> {
        let result = Doujin::from_json(path)?;

        Ok(organize_fields(result))
    }
}

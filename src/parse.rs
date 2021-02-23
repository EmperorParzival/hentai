use crate::utility::error::Result;
use chrono::{serde::ts_seconds, DateTime, Utc};
use hyper::body::{self, Buf};
use serde::Deserialize;
use serde_json;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Deserialize)]
pub struct Title {
    pub pretty: String,
    pub english: String,
    pub japanese: String,
}

#[derive(Deserialize)]
pub struct Image {
    pub t: String,
    pub w: u32,
    pub h: u32,
}

#[derive(Deserialize)]
pub struct Images {
    pub pages: Vec<Image>,
    pub cover: Image,
    pub thumbnail: Image,
}

#[derive(Deserialize)]
pub struct Doujin {
    pub id: u32,
    pub title: Title,
    pub images: Images,
    pub tags: Vec<Tag>,
    pub num_pages: u32,
    pub media_id: String,
    pub scanlator: String,
    pub num_favorites: u32,
    #[serde(with = "ts_seconds")]
    pub upload_date: DateTime<Utc>,
}

impl Doujin {
    pub async fn new(id: u32) -> Result<Self> {
        let client = hyper::Client::new();
        let url = format!("https://nhentai.net/api/gallery/{}", id.to_string());

        let request = client.get(hyper::Uri::from_str(&url)?);
        let response = request.await.expect("Failed to request url");

        let body = body::aggregate(response)
            .await
            .expect("Failed to aggregate body");
        let result: Self =
            serde_json::from_reader(body.reader()).expect("Failed to deserialize json");

        Ok(result)
    }

    pub fn from_json(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let result: Self = serde_json::from_str(&data).expect("Failed to deserialize json");

        Ok(result)
    }
}

// "r#" causes syntax highlighting issues in Atom
#[derive(Deserialize)]
pub struct Tag {
    pub id: u32,
    pub count: u32,
    pub url: String,
    pub name: String,
    pub r#type: String,
}

use crate::utility::{api::url, error::Result};
use chrono::{serde::ts_seconds, DateTime, Utc};
use hyper::{
    body::{self, Buf},
    Client,
};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::{fs, path::PathBuf, str::FromStr};

/// nhentai provides three different types of titles. The first one is `pretty`, which is a
/// simple title meant to stand out. The `english` and `japanese` titles are also provided.
/// These are more fleshed out versions of the `pretty` title.
///
/// None of these fields are guaranteed to be provided. They might be set to an empty `String`, in
/// which case they will be set to `Some("")`. In other cases, the field may not be specified at
/// all, in which case they will be set to `None`.
#[derive(Deserialize, Debug)]
pub struct Title {
    pub pretty: Option<String>,
    pub english: Option<String>,
    pub japanese: Option<String>,
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

/// Tags are used to categorize doujins. Each doujin published on nhentai has tags indicating its
/// content and additional data like the language and artist. For example, the tag with the
/// category field equal to "artist" contains the name by which the author is recognized on nhentai.
#[derive(Deserialize, Debug)]
pub struct Tag {
    pub id: u32,
    pub count: u32,
    pub url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub category: String,
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
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let response = client
            .get(hyper::Uri::from_str(&url::doujin(id))?)
            .await
            .expect("Failed to request url");
        let content = body::aggregate(response)
            .await
            .expect("Failed to aggregate body");
        let result: Self =
            serde_json::from_reader(content.reader()).expect("Failed to deserialize json");

        Ok(result)
    }

    pub async fn random() -> Result<u32> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let response = client
            .get(hyper::Uri::from_str("https://nhentai.xxx/random")?)
            .await
            .expect("Failed to request url");
        let segments = response
            .headers()
            .get(hyper::header::LOCATION)
            .ok_or("Failed to retrieve url")?
            .to_str()?
            .split('/')
            .collect::<Vec<_>>();
        let code = segments
            .iter()
            .next_back()
            .ok_or("Failed to parse url")?
            .parse::<u32>()?;

        Ok(code)
    }

    pub fn from_json(path: PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path).expect("Failed to read file");
        let result: Self = serde_json::from_str(&data).expect("Failed to deserialize json");

        Ok(result)
    }
}

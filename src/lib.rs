//! # hentai
//!
//! The aptly named *hentai* crate provides an easy mechanism to consume nhentai's public facing
//! APIs in Rust. As of now, hentai relies solely on these APIs instead of scraping their website.
//! However, this capability may be added in the near future.
//!
//! hentai will provide information about a doujin given its six-digit code. Alternatively, the
//! JSON response from nhentai's [/api/gallery/{id}](https://nhentai.net/api/gallery/165961)
//! endpoint may be provided directly.
//!
//! hentai is based on the similar [package for python](https://pypi.org/project/hentai/).
//!
//! # Examples
//! *more info for these samples is provided in the documentation for `Hentai`*
//!
//! ```rust
//! use hentai::{Hentai, Result, Website};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let response = Hentai::new(165961, Website::NET).await?;
//!     println!("{:?}", response); // makes use of the Debug trait on Hentai
//!
//!     Ok(())
//! }
//! ```
//!
//! ```rust
//! use hentai::{Hentai, Result, Website};
//! use std::env;
//!
//! fn main() -> Result<()> {
//!     let mut path = env::current_exe()?; // std::path::PathBuf
//!     path.pop();
//!     path.push("sample.json");
//!
//!     if let Ok(result) = Hentai::from_json(path, Website::XXX) {
//!         println!("{:?}", result); // makes use of the Debug trait on Hentai
//!     }
//!
//!     Ok(())
//! }
//! ```

mod parse;
pub(crate) mod utility;

use chrono::{DateTime, Utc};
use parse::Doujin;
use std::{convert::TryInto, path::PathBuf};
use utility::api::url::Make;

pub use parse::{Tag, Title};
pub use utility::{
    api::url::Website,
    error::{HentaiError, Result},
};

/// The main object containing the formatted information retrieved from nhentai. The raw image
/// data is converted to into image URLs. A brief explanation of each field is located below.
///
/// - `url` is the direct link to the doujin created from the provided six-digit code.
/// - `scanlator` is the user that created translations for the doujin (not always present).
/// - Fields suffixed with "url" or "urls" are links to the doujin's image files.
/// - Fields starting with "num" are statistics provided by nhentai. These may not be accurate
/// for newly uploaded doujins.
/// - `title` contains the three different titles for the doujin provided by nhentai.
/// - Likewise, `tags` contains all the tags for the doujin.
/// - `media_id` is the separate code that nhentai uses to denote where the image files for
/// doujins are from. You will see this in the image urls (it is unrelated to the six-digit codes).
/// - `upload_date` is a
/// [chrono::DateTime](https://docs.rs/chrono/0.4.19/chrono/struct.DateTime.html) object that
/// indicates when the doujin was uploaded to nhentai. This is not the creation date of the doujin.
#[derive(Debug)]
pub struct Hentai {
    pub url: String,
    pub title: Title,
    pub tags: Vec<Tag>,
    pub num_pages: u32,
    pub media_id: String,
    pub scanlator: String,
    pub cover_url: String,
    pub num_favorites: u32,
    pub thumbnail_url: String,
    pub image_urls: Vec<String>,
    pub upload_date: DateTime<Utc>,
}

fn create_urls(raw: &Doujin, builder: &Make) -> Vec<String> {
    raw.images
        .pages
        .iter()
        .enumerate()
        .map(|(i, v)| builder.page(&raw.media_id, i.try_into().unwrap(), &v.t))
        .collect()
}

fn organize_fields(raw: Doujin, builder: Make) -> Hentai {
    let media_id = &raw.media_id;

    Hentai {
        image_urls: create_urls(&raw, &builder),
        media_id: media_id.to_string(),

        tags: raw.tags,
        title: raw.title,
        num_pages: raw.num_pages,
        scanlator: raw.scanlator,
        upload_date: raw.upload_date,
        num_favorites: raw.num_favorites,

        url: builder.doujin_url(raw.id),
        cover_url: builder.cover(media_id, &raw.images.cover.t),
        thumbnail_url: builder.cover_thumbnail(media_id, &raw.images.thumbnail.t),
    }
}

impl Hentai {
    /// Generates a new `Hentai` object for the provided six-digit code.
    ///
    /// This makes a request to the primary nhentai endpoint located at
    /// [/api/gallery/{id}](https://nhentai.net/api/gallery/165961).
    /// The JSON response is deserialized and converted into a `Hentai` object.
    /// This can fail at several stages, so it is important to handle the result accordingly.
    /// To make things simpler, `Result<Hentai, HentaiError>` is used.
    ///
    /// A malformed code may result in an invalid url
    /// ([http::uri::InvalidUri](https://docs.rs/http/0.2.3/http/uri/struct.InvalidUri.html)), the
    /// request to the endpoint may fail
    /// ([hyper::Error](https://docs.rs/hyper/0.14.4/hyper/struct.Error.html)), or the JSON
    /// response may be invalid
    /// ([serde_json::Error](https://docs.serde.rs/serde_json/struct.Error.html)).
    ///
    /// More information about the `Website` enum can be found in its section. The sample below
    /// depends on [tokio](https://tokio.rs/).
    /// ```rust
    /// use hentai::{Hentai, Result, Website};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let response = Hentai::new(165961, Website::NET).await?;
    ///     println!("{:?}", response); // makes use of the Debug trait on Hentai
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(code: u32, mode: Website) -> Result<Self> {
        let result = Doujin::new(code).await?;

        Ok(organize_fields(result, Make::new(mode)))
    }

    pub async fn random(mode: Website) -> Result<Self> {
        let code = Doujin::random(&mode).await?;
        let result = Doujin::new(code).await?;

        Ok(organize_fields(result, Make::new(mode)))
    }

    /// Generates a new `Hentai` object from the doujin JSON located at the provided file path.
    ///
    /// This may fail if there is not a file at the provided path
    /// ([std::io::Error](https://doc.rust-lang.org/std/io/struct.Error.html)), or if the contents
    /// of the file are invalid
    /// ([serde_json::Error](https://docs.serde.rs/serde_json/struct.Error.html)).
    ///
    /// The sample below assumes that you have a valid doujin JSON file called
    /// [sample.json](https://github.com/EmperorParzival/hentai/blob/main/src/cli/sample.json) in
    /// the same directory as the executable.
    /// ```rust
    /// use hentai::{Hentai, Result, Website};
    /// use std::env;
    ///
    /// fn main() -> Result<()> {
    ///     let mut path = env::current_exe()?; // std::path::PathBuf
    ///     path.pop();
    ///     path.push("sample.json");
    ///
    ///     if let Ok(result) = Hentai::from_json(path, Website::XXX) {
    ///         println!("{:?}", result); // makes use of the Debug trait on Hentai
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn from_json(path: PathBuf, mode: Website) -> Result<Self> {
        let result = Doujin::from_json(path)?;

        Ok(organize_fields(result, Make::new(mode)))
    }
}

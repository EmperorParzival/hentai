mod parse;
pub mod utility;

use parse::Doujin;
use std::path::PathBuf;

pub struct Hentai {
    pub raw: Doujin,
}

impl Hentai {
    pub async fn new(code: u32) -> Self {
        match Doujin::new(code).await {
            Ok(raw) => Hentai { raw },
            Err(err) => panic!("{}", err),
        }
    }

    pub fn from_json(path: PathBuf) -> Self {
        match Doujin::from_json(path) {
            Ok(raw) => Hentai { raw },
            Err(err) => panic!("{}", err),
        }
    }
}
